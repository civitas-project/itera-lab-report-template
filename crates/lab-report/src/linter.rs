// @impl:FR-CLI-06
//! Typography invariant audit and automatic correction engine.

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walkdir::WalkDir;

/// Severity classification of a lint diagnostic issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Non-negotiable typography or structural invariant violation.
    Error,
    /// Advisory recommendation.
    Warning,
}

/// A single typography or quality defect identified by the linter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintIssue {
    /// File path where defect was identified.
    pub file_path: PathBuf,
    /// 1-based line number in source file.
    pub line_num: usize,
    /// Formal rule identifier (e.g. `TYPO-01-BOLD-COLON`).
    pub rule_id: &'static str,
    /// Descriptive explanation of the issue.
    pub message: String,
    /// Diagnostic severity rating.
    pub severity: Severity,
}

static PLACEHOLDER_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)jelaskan\s+langkah-langkah").unwrap(),
        Regex::new(r"(?i)tuliskan\s+kesimpulan").unwrap(),
        Regex::new(r"(?i)tuliskan\s+deskripsi").unwrap(),
        Regex::new(r"(?i)\[tuliskan").unwrap(),
        Regex::new(r"(?i)placeholder\s+text").unwrap(),
    ]
});

static BOLD_COLON_INSIDE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\s*(?:[-+*•–]|\d+\.|\([a-zA-Z\d]+\)|[a-zA-Z]\.)?\s*)\*([^*\n:]+):(\*)").unwrap()
});

static BOLD_COLON_OUTSIDE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\s*(?:[-+*•–]|\d+\.|\([a-zA-Z\d]+\)|[a-zA-Z]\.)?\s*)\*([^*\n:]+)\*(:)").unwrap()
});

/// Audits a single `.typ` file for typography invariants and optionally applies fixes.
///
/// # Errors
/// Returns [`std::io::Error`] if reading or writing to `file_path` fails.
pub fn lint_file(file_path: &Path, fix: bool) -> Result<Vec<LintIssue>, std::io::Error> {
    lint_file_with_options(file_path, fix, false)
}

// @impl:FR-CLI-15
/// Audits a single `.typ` file with optional in-place fix or dry-run simulation.
///
/// # Errors
/// Returns [`std::io::Error`] if reading or writing fails.
pub fn lint_file_with_options(file_path: &Path, fix: bool, dry_run: bool) -> Result<Vec<LintIssue>, std::io::Error> {
    if !file_path.is_file() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(file_path)?;
    let mut issues = Vec::new();
    let mut modified = false;
    let mut new_lines = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let line_num = idx + 1;
        let mut current_line = line.to_string();

        for p in PLACEHOLDER_PATTERNS.iter() {
            if p.is_match(&current_line) {
                let preview = current_line.trim();
                let short_preview = if preview.len() > 50 {
                    &preview[..50]
                } else {
                    preview
                };
                issues.push(LintIssue {
                    file_path: file_path.to_path_buf(),
                    line_num,
                    rule_id: "QUAL-01-PLACEHOLDER",
                    message: format!("Instructional template placeholder detected: '{short_preview}...'"),
                    severity: Severity::Error,
                });
            }
        }

        if BOLD_COLON_INSIDE.is_match(&current_line) {
            issues.push(LintIssue {
                file_path: file_path.to_path_buf(),
                line_num,
                rule_id: "TYPO-01-BOLD-COLON",
                message: "Prohibited colon detected inside bold title header (*Title:*). Remove colon and use '*Title* \\'.".to_string(),
                severity: Severity::Error,
            });
            if fix {
                current_line = BOLD_COLON_INSIDE.replace(&current_line, "${1}*${2}*").to_string();
                modified = true;
            }
        } else if BOLD_COLON_OUTSIDE.is_match(&current_line) {
            issues.push(LintIssue {
                file_path: file_path.to_path_buf(),
                line_num,
                rule_id: "TYPO-01-BOLD-COLON",
                message: "Prohibited colon detected after bold title header (*Title*:). Remove colon and use '*Title* \\'.".to_string(),
                severity: Severity::Error,
            });
            if fix {
                current_line = BOLD_COLON_OUTSIDE.replace(&current_line, "${1}*${2}*").to_string();
                modified = true;
            }
        }

        new_lines.push(current_line);
    }

    if fix && !dry_run && modified {
        let output = if content.ends_with('\n') {
            format!("{}\n", new_lines.join("\n"))
        } else {
            new_lines.join("\n")
        };
        fs::write(file_path, output)?;
    }

    Ok(issues)
}

/// Recursively scans course directory and audits all Typst files.
///
/// # Errors
/// Returns [`std::io::Error`] if file tree reading fails.
pub fn lint_course(course_path: &Path, fix: bool) -> Result<Vec<LintIssue>, std::io::Error> {
    lint_course_with_options(course_path, fix, false)
}

// @impl:FR-CLI-15
/// Recursively scans course directory with optional dry-run preview mode.
///
/// # Errors
/// Returns [`std::io::Error`] if file tree reading fails.
pub fn lint_course_with_options(
    course_path: &Path,
    fix: bool,
    dry_run: bool,
) -> Result<Vec<LintIssue>, std::io::Error> {
    if !course_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Course directory not found: {}", course_path.display()),
        ));
    }

    let mut all_issues = Vec::new();

    let content_dir = course_path.join("content");
    if content_dir.is_dir() {
        for entry in WalkDir::new(&content_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "typ") {
                let issues = lint_file_with_options(path, fix, dry_run)?;
                all_issues.extend(issues);
            }
        }
    }

    let main_typ = course_path.join("main.typ");
    if main_typ.is_file() {
        let issues = lint_file_with_options(&main_typ, fix, dry_run)?;
        all_issues.extend(issues);
    }

    Ok(all_issues)
}
