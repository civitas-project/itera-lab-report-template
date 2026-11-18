// @impl:FR-CLI-02
//! Laboratory course catalog and identifier resolution.

use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors encountered during course resolution or validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CourseError {
    /// Provided course identifier does not match registered catalog entries.
    #[error("Unknown course identifier: {0}")]
    UnknownCourse(String),
    /// Expected course directory does not exist on disk.
    #[error("Course directory not found: {0}")]
    DirectoryNotFound(PathBuf),
}

/// Enumeration of recognized laboratory courses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CourseKind {
    /// Basis Data (Database Systems).
    Database,
    /// Jaringan Komputer (Computer Networks).
    ComputerNetworks,
    /// Pemrograman Berorientasi Objek (Object-Oriented Programming).
    ObjectOrientedProgramming,
}

impl CourseKind {
    /// Directory folder name relative to workspace root.
    #[must_use]
    pub const fn dir_name(self) -> &'static str {
        match self {
            Self::Database => "database",
            Self::ComputerNetworks => "computer-networks",
            Self::ObjectOrientedProgramming => "object-oriented-programming",
        }
    }

    /// Human-readable course title.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Database => "Database Systems",
            Self::ComputerNetworks => "Computer Networks",
            Self::ObjectOrientedProgramming => "Object-Oriented Programming",
        }
    }

    /// Resolves course root directory against `workspace_root`.
    ///
    /// # Errors
    /// Returns [`CourseError::DirectoryNotFound`] when path is not an existing directory.
    pub fn resolve_dir(self, workspace_root: &Path) -> Result<PathBuf, CourseError> {
        let path = workspace_root.join(self.dir_name());
        if path.is_dir() {
            Ok(path)
        } else {
            Err(CourseError::DirectoryNotFound(path))
        }
    }

    /// Course metadata descriptor for agent onboarding and contract verification.
    #[must_use]
    pub const fn metadata(self) -> CourseMetadata {
        match self {
            Self::Database => CourseMetadata {
                required_fonts: &["GARA.TTF", "GARABD.TTF", "Arial.ttf", "Arial-Bold.ttf", "BERNHC.TTF"],
                formula_rule: "Table naming and student attributes derived directly from NIM (data/student.typ).",
                special_notes: "Uses ISO B5 format, 12pt Garamond with uniform 1.5 line spacing (0.75em). SQL execution output must be authentic.",
            },
            Self::ComputerNetworks => CourseMetadata {
                required_fonts: &["GARA.TTF", "GARABD.TTF", "Arial.ttf", "Arial-Bold.ttf", "BERNHC.TTF"],
                formula_rule: "Derive subnetting and host IPs from the module PDF using the student NIM; do not apply a universal formula. Record the derivation in MODULE_VARIABLES.md.",
                special_notes: "Uses ISO B5 format, 12pt Garamond. CLI verification figures must display authentic ping/route replies.",
            },
            Self::ObjectOrientedProgramming => CourseMetadata {
                required_fonts: &["times.ttf", "timesbd.ttf", "ariblk.ttf"],
                formula_rule: "Package namespaces and class identifiers reflect student NIM and module task requirements.",
                special_notes: "Uses ISO A4 format, 12pt Times New Roman. Source code blocks must be cleanly formatted with zero bold-colon violations.",
            },
        }
    }
}

/// Course-specific academic and tooling metadata for agent onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseMetadata {
    /// Required typography font filenames.
    pub required_fonts: &'static [&'static str],
    /// Academic formula / student variable contract description.
    pub formula_rule: &'static str,
    /// Special course instructions or invariants.
    pub special_notes: &'static str,
}

/// Resolves user-supplied course string or alias into canonical [`CourseKind`].
///
/// # Errors
/// Returns [`CourseError::UnknownCourse`] if input key does not match catalog.
    pub fn resolve_course_key(key: &str) -> Result<CourseKind, CourseError> {
    let clean = key.trim().to_ascii_lowercase();
    let clean = clean.strip_prefix("laprak-").unwrap_or(&clean);
    match clean {
        "database" | "basis-data" | "basdat" | "bd" => Ok(CourseKind::Database),
        "computer-networks" | "jarkom" | "networking" | "cn" => Ok(CourseKind::ComputerNetworks),
        "object-oriented-programming" | "pbo" | "oop" => Ok(CourseKind::ObjectOrientedProgramming),
        other => Err(CourseError::UnknownCourse(other.to_string())),
    }
}

// @impl:FR-CLI-13
/// Attempts to detect a recognized [`CourseKind`] from the provided path or current directory.
#[must_use]
pub fn detect_course_from_path(path: &Path) -> Option<CourseKind> {
    for comp in path.components() {
        let name = comp.as_os_str().to_string_lossy().to_ascii_lowercase();
        if let Ok(kind) = resolve_course_key(&name) {
            return Some(kind);
        }
    }
    None
}

/// Resolves the course directory for both monorepo and standalone layouts.
#[must_use]
pub fn course_root(workspace_root: &Path, course: CourseKind) -> PathBuf {
    let nested = workspace_root.join(course.dir_name());
    if nested.join("main.typ").is_file() {
        nested
    } else {
        workspace_root.to_path_buf()
    }
}

/// Detects course kind from `.agent-context.md` metadata or `data/report.typ` course codes.
#[must_use]
pub fn detect_course_from_metadata(root: &Path) -> Option<CourseKind> {
    let context = root.join(".agent-context.md");
    if let Ok(text) = fs::read_to_string(&context) {
        for line in text.lines() {
            if line.contains("Course Key:") {
                if let Some(start) = line.find('`') {
                    if let Some(end) = line[start + 1..].find('`') {
                        let key = &line[start + 1..start + 1 + end];
                        if let Ok(kind) = resolve_course_key(key) {
                            return Some(kind);
                        }
                    }
                }
            }
        }
    }
    if let Ok(text) = fs::read_to_string(root.join("data/report.typ")) {
        for line in text.lines() {
            let kind = match line {
                l if l.contains("IF25-21012") || l.contains("IF22-21010") => CourseKind::Database,
                l if l.contains("IF25-21008") => CourseKind::ComputerNetworks,
                l if l.contains("IF22-20008") => CourseKind::ObjectOrientedProgramming,
                _ => continue,
            };
            return Some(kind);
        }
    }
    None
}

/// Walks up from `start` to locate the workspace root, identified by the
/// presence of both `profile.toml` and `crates/lab-report/Cargo.toml`, or
/// both `profile.toml` and `main.typ` for standalone projects.
#[must_use]
pub fn find_workspace_root(start: &Path) -> Option<PathBuf> {
    let canonical = fs::canonicalize(start).ok()?;
    for ancestor in canonical.ancestors() {
        if ancestor.join("profile.toml").is_file()
            && ancestor.join("crates/lab-report/Cargo.toml").is_file()
        {
            return Some(ancestor.to_path_buf());
        }
        if ancestor.join("profile.toml").is_file() && ancestor.join("main.typ").is_file() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

