// @impl:FR-CLI-08
//! PlantUML diagram compilation and vector asset scaffolding.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;
use walkdir::WalkDir;

/// Errors arising during diagram rendering or template generation.
#[derive(Debug, Error)]
pub enum DiagramError {
    /// Source PlantUML file does not exist on disk.
    #[error("PlantUML source file not found: {0}")]
    FileNotFound(PathBuf),
    /// Neither `plantuml` nor `spec-engine` was found in PATH.
    #[error("Neither 'plantuml' nor 'spec-engine' binary found on system")]
    BinaryNotFound,
    /// PlantUML CLI compiler failed with non-zero exit code.
    #[error("PlantUML compilation failed: {0}")]
    CompilationFailed(String),
    /// Expected output SVG file was not created.
    #[error("Expected SVG file was not created: {0}")]
    OutputMissing(PathBuf),
    /// Filesystem I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Archetype category for specialized diagram skinparams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramType {
    /// Default generic layout.
    Default,
    /// Computer network topology diagram.
    Network,
    /// Database Entity-Relationship Diagram (ERD).
    Erd,
    /// UML Class Diagram.
    Class,
}

/// Discovers PlantUML executable or compatible renderer in system environment.
#[must_use]
pub fn find_plantuml_bin() -> Option<PathBuf> {
    if let Ok(output) = Command::new("which").arg("plantuml").output() {
        if output.status.success() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let trimmed = path_str.trim();
                if !trimmed.is_empty() {
                    return Some(PathBuf::from(trimmed));
                }
            }
        }
    }

    if let Some(home) = std::env::var_os("HOME") {
        let local_bin = PathBuf::from(home).join(".local/bin/plantuml");
        if local_bin.is_file() {
            return Some(local_bin);
        }
    }

    if let Ok(output) = Command::new("which").arg("spec-engine").output() {
        if output.status.success() {
            if let Ok(path_str) = String::from_utf8(output.stdout) {
                let trimmed = path_str.trim();
                if !trimmed.is_empty() {
                    return Some(PathBuf::from(trimmed));
                }
            }
        }
    }

    None
}

/// Generates documented skinparam styling headers for PlantUML diagrams.
#[must_use]
pub const fn get_puml_header(diagram_type: DiagramType) -> &'static str {
    match diagram_type {
        DiagramType::Default => {
            "' Vector Theme for ITERA Lab Reports\nskinparam backgroundColor white\nskinparam defaultFontName Garamond\nskinparam defaultFontSize 11\nskinparam roundCorner 4\nskinparam shadowing false\nskinparam dpi 300\n\nskinparam ArrowColor #4F81BD\nskinparam ArrowThickness 1.2\n"
        }
        DiagramType::Network => {
            "' Vector Theme for ITERA Lab Reports\nskinparam backgroundColor white\nskinparam defaultFontName Garamond\nskinparam defaultFontSize 11\nskinparam roundCorner 4\nskinparam shadowing false\nskinparam dpi 300\n\nskinparam ArrowColor #4F81BD\nskinparam ArrowThickness 1.2\nskinparam node {\n  BackgroundColor #DBE5F1\n  BorderColor #4F81BD\n  BorderThickness 1.2\n}\nskinparam component {\n  BackgroundColor #FFFFFF\n  BorderColor #4F81BD\n  BorderThickness 1.2\n}\n"
        }
        DiagramType::Erd => {
            "' Vector Theme for ITERA Lab Reports\nskinparam backgroundColor white\nskinparam defaultFontName Garamond\nskinparam defaultFontSize 11\nskinparam roundCorner 4\nskinparam shadowing false\nskinparam dpi 300\n\nskinparam ArrowColor #4F81BD\nskinparam ArrowThickness 1.2\nskinparam class {\n  BackgroundColor #FFFFFF\n  HeaderBackgroundColor #4F81BD\n  BorderColor #4F81BD\n  BorderThickness 1.2\n}\n"
        }
        DiagramType::Class => {
            "' Vector Theme for ITERA Lab Reports\nskinparam backgroundColor white\nskinparam defaultFontName Garamond\nskinparam defaultFontSize 11\nskinparam roundCorner 4\nskinparam shadowing false\nskinparam dpi 300\n\nskinparam ArrowColor #4F81BD\nskinparam ArrowThickness 1.2\nskinparam class {\n  BackgroundColor #FFFFFF\n  HeaderBackgroundColor #DBE5F1\n  BorderColor #4F81BD\n  BorderThickness 1.2\n}\n"
        }
    }
}

/// Compiles a `.puml` diagram into a vector `.svg` asset.
///
/// # Errors
/// Returns [`DiagramError::FileNotFound`] if source file does not exist.
/// Returns [`DiagramError::BinaryNotFound`] if compiler executable is absent.
/// Returns [`DiagramError::CompilationFailed`] if CLI compiler returns non-zero.
pub fn render_puml(puml_path: &Path, out_dir: Option<&Path>) -> Result<PathBuf, DiagramError> {
    if !puml_path.is_file() {
        return Err(DiagramError::FileNotFound(puml_path.to_path_buf()));
    }

    let bin = find_plantuml_bin().ok_or(DiagramError::BinaryNotFound)?;
    let target_dir = match out_dir {
        Some(d) => d.to_path_buf(),
        None => puml_path.parent().unwrap_or(Path::new(".")).to_path_buf(),
    };
    fs::create_dir_all(&target_dir)?;

    let stem = puml_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("diagram");
    let expected_svg = target_dir.join(format!("{stem}.svg"));

    let bin_str = bin.to_string_lossy();
    let status = if bin_str.contains("spec-engine") {
        Command::new(&bin)
            .args(["render", &target_dir.to_string_lossy()])
            .status()?
    } else {
        Command::new(&bin)
            .args(["-tsvg", "-o", &target_dir.to_string_lossy(), &puml_path.to_string_lossy()])
            .status()?
    };

    if !status.success() {
        return Err(DiagramError::CompilationFailed(format!(
            "Exit code: {}",
            status.code().unwrap_or(1)
        )));
    }

    if !expected_svg.is_file() {
        return Err(DiagramError::OutputMissing(expected_svg));
    }

    Ok(expected_svg)
}

/// Recursively scans directory and compiles all discovered `.puml` files.
///
/// # Errors
/// Returns [`DiagramError`] if compilation fails for any discovered diagram.
pub fn render_all_diagrams(course_path: &Path) -> Result<Vec<PathBuf>, DiagramError> {
    let mut rendered = Vec::new();
    for entry in WalkDir::new(course_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "puml") {
            let svg = render_puml(path, None)?;
            rendered.push(svg);
        }
    }
    Ok(rendered)
}

/// Scaffolds a starter `.puml` diagram template with official typography skinparams.
///
/// # Errors
/// Returns [`DiagramError::Io`] if file writing fails.
pub fn scaffold_diagram(
    target_path: &Path,
    diagram_type: DiagramType,
    title: &str,
) -> Result<PathBuf, DiagramError> {
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let header = get_puml_header(diagram_type);
    let body = match diagram_type {
        DiagramType::Network => {
            format!("title {title}\n\nnode \"Router-1\\n192.168.1.1\" as R1\nnode \"Switch-1\" as SW1\nnode \"PC-1\\n192.168.1.10\" as PC1\n\nR1 <--> SW1 : Fiber Optic\nSW1 <--> PC1 : Copper Straight\n")
        }
        DiagramType::Erd => {
            format!("title {title}\n\nentity \"Mahasiswa\" as mhs {{\n  * nim : VARCHAR(10) <<PK>>\n  --\n  * nama : VARCHAR(100)\n  prodi : VARCHAR(50)\n}}\n")
        }
        DiagramType::Class => {
            format!("title {title}\n\nclass Animal {{\n  + name: String\n  + makeSound(): void\n}}\nclass Dog extends Animal {{\n  + bark(): void\n}}\n")
        }
        DiagramType::Default => {
            format!("title {title}\n\n[Component A] -> [Component B] : Transmit\n")
        }
    };

    let content = format!("{header}\n{body}");
    fs::write(target_path, content)?;
    Ok(target_path.to_path_buf())
}
