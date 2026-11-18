// @impl:FR-CLI-09
//! Project scaffolding engine for generating new lab report workspaces.

use crate::config::{format_student_data, Profile};
use crate::courses::CourseKind;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

/// Errors arising during lab report project scaffolding.
#[derive(Debug, Error)]
pub enum GeneratorError {
    /// Archetype source template not found.
    #[error("Archetype source template directory not found: {0}")]
    TemplateNotFound(PathBuf),
    /// Target destination directory already exists.
    #[error("Target directory already exists: {0}")]
    DestinationExists(PathBuf),
    /// Filesystem I/O failure.
    #[error("I/O error during project scaffolding: {0}")]
    Io(#[from] std::io::Error),
}

/// Recursively copies directory tree ignoring build artifacts and hidden state.
fn copy_dir_clean(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let rel = match path.strip_prefix(src) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if rel.as_os_str().is_empty() {
            continue;
        }

        let rel_str = rel.to_string_lossy();
        if rel_str.starts_with(".git")
            || rel_str.starts_with(".superpowers")
            || rel_str.starts_with("output_svg")
            || rel_str.ends_with(".pdf")
            || rel_str.ends_with(".zip")
        {
            continue;
        }

        let target = dst.join(rel);
        if path.is_dir() {
            fs::create_dir_all(&target)?;
        } else if path.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }
    Ok(())
}

/// Scaffolds a new lab report workspace from a registered course archetype.
///
/// # Errors
/// Returns [`GeneratorError::TemplateNotFound`] if course template is missing.
/// Returns [`GeneratorError::DestinationExists`] if destination path is already occupied.
/// Returns [`GeneratorError::Io`] if file copying fails.
pub fn scaffold_project(
    course: CourseKind,
    workspace_root: &Path,
    target_dir: &Path,
    profile: &Profile,
) -> Result<PathBuf, GeneratorError> {
    let source_dir = workspace_root.join(course.dir_name());
    if !source_dir.is_dir() {
        return Err(GeneratorError::TemplateNotFound(source_dir));
    }

    if target_dir.exists() && fs::read_dir(target_dir)?.next().is_some() {
        return Err(GeneratorError::DestinationExists(target_dir.to_path_buf()));
    }

    copy_dir_clean(&source_dir, target_dir)?;

    // Write full student profile so standalone project does not fall back
    profile
        .save_to_file(&target_dir.join("profile.toml"))
        .map_err(|e| GeneratorError::Io(std::io::Error::other(e.to_string())))?;

    // Copy shared template source into standalone project when it exists
    let shared_source = workspace_root.join("templates/b5");
    if shared_source.is_dir() {
        copy_dir_clean(&shared_source, &target_dir.join("templates/b5"))?;
    }

    // Personalize student metadata in target data directory
    let data_dir = target_dir.join("data");
    if data_dir.is_dir() {
        let student_file = data_dir.join("student.typ");
        let name = &profile.student.name;
        let nim = &profile.student.nim;

        let content = format_student_data(course, name, nim);
        fs::write(&student_file, content)?;
    }

    // Generate .agent-context.md onboarding file
    generate_agent_context(course, target_dir, profile)?;

    Ok(target_dir.to_path_buf())
}

// @impl:FR-CLI-17
/// Generates localized `.agent-context.md` onboarding file for AI coding agents.
///
/// # Errors
/// Returns [`GeneratorError::Io`] if file write fails.
pub fn generate_agent_context(
    course: CourseKind,
    target_dir: &Path,
    profile: &Profile,
) -> Result<PathBuf, GeneratorError> {
    let meta = course.metadata();
    let course_name = course.display_name();
    let course_dir = course.dir_name();
    let font_list = meta.required_fonts.join(", ");
    let name = &profile.student.name;
    let nim = &profile.student.nim;
    let class = &profile.student.class;

    let content = format!(
r#"# Agent Context: {course_name}

**Universal Skill:** `itera-lab-report-standard` (load first via skill tool)

## Detailed Guide
- **Required reading:** `REFERENCE.md` in this project root (copied from the course archetype).
- It defines file ownership, section contracts, grounding rules, evidence rules, and quality gates.
- Keep this file short; put procedural detail in REFERENCE.md, not here.
- **Proof contract:** `PROOF_CONTRACT.md` in this project root defines required evidence per module (type, command, file location, pass criteria).
- **Module variables:** `MODULE_VARIABLES.md` in this project root records NIM-derived parameters and the module PDF rule behind each one.

## Active Student Metadata
- **Name:** {name}
- **NIM:** {nim}
- **Class:** {class}

## Course Contracts & Rules
- **Course Key:** `{course_dir}`
- **Required Fonts:** {font_list}
- **NIM / Variable Rule:** {formula_rule}
- **Typography & Formatting:** {special_notes}

## Absolute Typography Invariants
1. **Uniform 1.5 Line Spacing:** `leading: 0.75em, spacing: 0.75em` (no extra `#v(...)` or double blank lines).
2. **Prohibited Bold Header Colon:** Never write `*Judul:* Penjelasan`. Always use `*Judul* \ \n Penjelasan`.
3. **Authentic Outputs:** All screenshots and terminal listings must show real execution outputs, never empty canvas.
4. **Zero Placeholder Text:** Never leave `[Isi...]` or template instructions in completed reports.

## Essential CLI Commands
Execute with the native `lab-report` binary:
- `lab-report build -m 1` : Compile report to PDF (auto-detects course in directory)
- `lab-report watch -m 1` : Live compilation watcher during drafting
- `lab-report test`       : Run declarative SVG geometry and font verification
- `lab-report lint --fix` : Audit typography violations and fix bold colons
- `lab-report pack -m 1`  : Bundle PDF and source code into submission ZIP
- `lab-report audit`    : Check gap closure and that dist ZIP still matches the final PDF
"#,
        formula_rule = meta.formula_rule,
        special_notes = meta.special_notes,
    );

    let context_path = target_dir.join(".agent-context.md");
    fs::write(&context_path, content)?;
    Ok(context_path)
}
