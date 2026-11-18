// @impl:FR-CLI-04
//! Typst compilation orchestration and student metadata synchronization.

use crate::config::{format_student_data, Profile};
use crate::courses::{course_root, detect_course_from_metadata, CourseKind};
use crate::naming::format_artifact_filename;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

/// Errors arising during Typst compilation or workspace synchronization.
#[derive(Debug, Error)]
pub enum RunnerError {
    /// File read or write error during student data synchronization.
    #[error("I/O error during synchronization: {0}")]
    Io(#[from] std::io::Error),
    /// Typst CLI tool is not installed or discoverable in PATH.
    #[error("Typst executable not found in PATH")]
    TypstNotFound,
    /// Artifact naming format failure.
    #[error("Naming error: {0}")]
    Naming(#[from] crate::naming::NamingError),
    /// Typst process execution failed with non-zero exit code.
    #[error("Typst compilation exited with non-zero status code: {0}")]
    CompilationFailed(i32),
}

// @impl:FR-CLI-16
/// Formats runner errors into actionable remediation messages with installation commands.
#[must_use]
pub fn format_remediation_message(err: &RunnerError) -> String {
    format_remediation_message_bilingual(err, crate::i18n::Language::En)
}

// @impl:FR-CLI-19
/// Formats runner errors with localized language support.
#[must_use]
pub fn format_remediation_message_bilingual(err: &RunnerError, lang: crate::i18n::Language) -> String {
    use crate::i18n::Language;
    match err {
        RunnerError::TypstNotFound => match lang {
            Language::En => {
                r#"Error: Typst executable was not found in your PATH.

Remediation:
  Please install Typst CLI using one of the following commands:
  • Rust / Cargo:  cargo install --locked typst-cli
  • Arch Linux:    pacman -S typst
  • Homebrew:      brew install typst
  • Windows:       winget install --id Typst.Typst
  • Binary:        Download from https://github.com/typst/typst/releases"#
                    .to_string()
            }
            Language::Id => {
                r#"Kesalahan: Aplikasi Typst tidak ditemukan di PATH sistem Anda.

Solusi Remediasi:
  Silakan instal Typst CLI menggunakan salah satu perintah berikut:
  • Rust / Cargo:  cargo install --locked typst-cli
  • Arch Linux:    pacman -S typst
  • Homebrew:      brew install typst
  • Windows:       winget install --id Typst.Typst
  • Unduhan:       Unduh dari https://github.com/typst/typst/releases"#
                    .to_string()
            }
        },
        RunnerError::CompilationFailed(code) => match lang {
            Language::En => format!("Error: Typst compiler exited with status code {code}. Check console output above for syntax diagnostics."),
            Language::Id => format!("Kesalahan: Kompiler Typst gagal dengan kode status {code}. Periksa pesan log di atas."),
        },
        RunnerError::Io(io_err) => format!("I/O Error: {io_err}"),
        RunnerError::Naming(naming_err) => format!("Naming Error: {naming_err}"),
    }
}

/// Assembles CLI command vector for executing Typst compiler.
#[must_use]
pub fn build_compile_command(
    course: CourseKind,
    workspace_root: &Path,
    output_path: &Path,
    watch: bool,
) -> Vec<String> {
    let mode = if watch { "watch" } else { "compile" };
    let abs_root = workspace_root
        .canonicalize()
        .unwrap_or_else(|_| workspace_root.to_path_buf());
    let course_dir = course_root(&abs_root, course);
    let fonts_path = course_dir.join("fonts");
    let main_typ = course_dir.join("main.typ");

    vec![
        "typst".to_string(),
        mode.to_string(),
        "--font-path".to_string(),
        fonts_path.display().to_string(),
        main_typ.display().to_string(),
        output_path.display().to_string(),
    ]
}

/// Synchronizes student profile metadata across registered course data directories.
///
/// # Errors
/// Returns [`RunnerError::Io`] if directory scanning or file writing fails.
pub fn sync_student_data(workspace_root: &Path, profile: &Profile) -> Result<Vec<PathBuf>, RunnerError> {
    let mut updated = Vec::new();
    let name = &profile.student.name;
    let nim = &profile.student.nim;
    let class_val = &profile.student.class;

    // A standalone project has a single course identified by its metadata;
    // the monorepo synchronizes all registered course data directories.
    let standalone_course = detect_course_from_metadata(workspace_root);
    let targets: Vec<CourseKind> = match standalone_course {
        Some(c) => vec![c],
        None => vec![
            CourseKind::ObjectOrientedProgramming,
            CourseKind::ComputerNetworks,
            CourseKind::Database,
        ],
    };

    for course in targets {
        let nested = workspace_root.join(course.dir_name());
        let data_dir = if nested.is_dir() {
            nested.join("data")
        } else {
            course_root(workspace_root, course).join("data")
        };
        if !data_dir.is_dir() {
            continue;
        }

        let student_file = data_dir.join("student.typ");
        let content = format_student_data(course, name, nim);
        fs::write(&student_file, content)?;
        updated.push(student_file);

        if !matches!(course, CourseKind::ObjectOrientedProgramming) {
            let class_file = data_dir.join("class.typ");
            let class_content = format!("// Kelas praktikum\n#let class-data = \"{class_val}\"\n");
            fs::write(&class_file, class_content)?;
            updated.push(class_file);
        }
    }

    Ok(updated)
}

/// Stages the shared B5 template inside the course root so Typst's
/// project sandbox can resolve `/templates/b5/...` imports.
///
/// # Errors
/// Returns [`std::io::Error`] if filesystem staging fails.
pub fn stage_shared_templates(workspace_root: &Path, course: CourseKind) -> std::io::Result<()> {
    if !matches!(course, CourseKind::Database | CourseKind::ComputerNetworks) {
        return Ok(());
    }
    let source = workspace_root.join("templates/b5");
    if !source.is_dir() {
        return Ok(());
    }
    let dest = course_root(workspace_root, course).join("templates/b5");
    if dest == source {
        return Ok(());
    }
    if dest.exists() {
        fs::remove_dir_all(&dest)?;
    }
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(&source)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, dest.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Resolves target artifact destination path for PDF or multi-page SVG export.
///
/// # Errors
/// Returns [`RunnerError::Naming`] if filename generation fails.
pub fn resolve_output_path(
    course: CourseKind,
    workspace_root: &Path,
    profile: &Profile,
    module_num: &str,
    output_path: Option<&Path>,
    format_opt: &str,
) -> Result<PathBuf, RunnerError> {
    let abs_root = workspace_root
        .canonicalize()
        .unwrap_or_else(|_| workspace_root.to_path_buf());
    let course_dir = course_root(&abs_root, course);

    let target = match output_path {
        Some(p) => {
            let mut resolved = if p.is_absolute() {
                p.to_path_buf()
            } else {
                abs_root.join(p)
            };

            if (format_opt == "svg" || resolved.extension().and_then(|e| e.to_str()) == Some("svg"))
                && !resolved.to_string_lossy().contains("{p}")
                && !resolved.to_string_lossy().contains("{0p}")
            {
                if resolved.is_dir() || resolved.extension().is_none() {
                    resolved = resolved.join("page-{0p}.svg");
                } else if let Some(stem) = resolved.file_stem().and_then(|s| s.to_str()) {
                    let parent = resolved.parent().unwrap_or(&abs_root);
                    resolved = parent.join(format!("{stem}-{{0p}}.svg"));
                }
            }
            resolved
        }
        None => {
            if format_opt == "svg" {
                course_dir.join("output_svg").join("page-{0p}.svg")
            } else {
                let filename = format_artifact_filename(
                    &profile.naming.pattern,
                    &profile.student.name,
                    &profile.student.nim,
                    module_num,
                    "pdf",
                )?;
                course_dir.join("dist").join(filename)
            }
        }
    };

    Ok(target)
}

/// Compiles a course lab report to PDF or SVG using the local Typst executable.
///
/// # Errors
/// Returns [`RunnerError::TypstNotFound`] if `typst` cannot be spawned.
/// Returns [`RunnerError::CompilationFailed`] if `typst` exits with non-zero code.
#[allow(clippy::too_many_arguments)]
pub fn compile_report(
    course: CourseKind,
    workspace_root: &Path,
    profile: &Profile,
    module_num: &str,
    output_path: Option<&Path>,
    format_opt: &str,
    watch: bool,
    extra_args: &[String],
) -> Result<i32, RunnerError> {
    sync_student_data(workspace_root, profile)?;
    stage_shared_templates(workspace_root, course)?;

    let abs_root = workspace_root
        .canonicalize()
        .unwrap_or_else(|_| workspace_root.to_path_buf());
    let course_dir = course_root(&abs_root, course);

    let target_path = resolve_output_path(
        course,
        workspace_root,
        profile,
        module_num,
        output_path,
        format_opt,
    )?;

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut args = build_compile_command(course, &abs_root, &target_path, watch);
    for extra in extra_args {
        args.push(extra.clone());
    }

    let status = Command::new(&args[0])
        .args(&args[1..])
        .current_dir(&course_dir)
        .status()
        .map_err(|_| RunnerError::TypstNotFound)?;

    if status.success() {
        Ok(0)
    } else {
        Err(RunnerError::CompilationFailed(status.code().unwrap_or(1)))
    }
}
