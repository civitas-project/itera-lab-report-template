// @impl:FR-CLI-01
//! Student profile and workspace configuration parser.

use crate::courses::CourseKind;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors encountered while loading or parsing configuration files.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// File read failure at system I/O boundary.
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    /// Syntactic or schema violation in TOML payload.
    #[error("Failed to parse TOML configuration: {0}")]
    InvalidToml(#[from] toml::de::Error),
}

fn default_class() -> String {
    "RA".to_string()
}

fn default_study_program() -> String {
    "Teknik Informatika".to_string()
}

fn default_institution() -> String {
    "Institut Teknologi Sumatera".to_string()
}

/// Student metadata block inside `profile.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StudentConfig {
    /// Full name of student.
    pub name: String,
    /// Student identification number (NIM).
    #[serde(alias = "student_id")]
    pub nim: String,
    /// Student class / section (e.g. "RA", "RB").
    #[serde(default = "default_class")]
    pub class: String,
    /// Academic study program.
    #[serde(default = "default_study_program")]
    pub study_program: String,
    /// Educational institution.
    #[serde(default = "default_institution")]
    pub institution: String,
}

impl Default for StudentConfig {
    fn default() -> Self {
        Self {
            name: "Timothy Ronald".to_string(),
            nim: "124140201".to_string(),
            class: "RA".to_string(),
            study_program: default_study_program(),
            institution: default_institution(),
        }
    }
}

/// Artifact naming template block inside `profile.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingConfig {
    /// Format template string with `{name}`, `{nim}`, and `{module}` tokens.
    pub pattern: String,
}

impl Default for NamingConfig {
    fn default() -> Self {
        Self {
            pattern: "{name}_{nim}_Tugas{module}".to_string(),
        }
    }
}

fn default_language() -> String {
    "en".to_string()
}

/// Root workspace profile configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// Student identification details.
    #[serde(default)]
    pub student: StudentConfig,
    /// Output file naming rules.
    #[serde(default)]
    pub naming: NamingConfig,
    /// Preferred interface language ("en" or "id").
    #[serde(default = "default_language")]
    pub language: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            student: StudentConfig::default(),
            naming: NamingConfig::default(),
            language: default_language(),
        }
    }
}

impl Profile {
    /// Deserializes profile configuration from raw TOML text.
    ///
    /// # Errors
    /// Returns [`ConfigError::InvalidToml`] when syntax or schema fails validation.
    pub fn from_toml_str(toml_str: &str) -> Result<Self, ConfigError> {
        let profile: Self = toml::from_str(toml_str)?;
        Ok(profile)
    }

    /// Reads profile from `path` or falls back to default settings when missing or invalid.
    #[must_use]
    pub fn load_or_default(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| Self::from_toml_str(&content).ok())
            .unwrap_or_default()
    }

    /// Resolves preferred interface language.
    #[must_use]
    pub fn get_language(&self) -> crate::i18n::Language {
        crate::i18n::Language::from_input(&self.language)
    }

    /// Serializes profile configuration and saves atomically to disk.
    ///
    /// # Errors
    /// Returns [`ConfigError::Io`] if write fails.
    pub fn save_to_file(&self, path: &Path) -> Result<(), ConfigError> {
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, serialized)?;
        Ok(())
    }
}

// @impl:FR-CLI-19
/// Updates language configuration in `profile.toml`.
///
/// # Errors
/// Returns [`ConfigError`] if profile cannot be read or saved.
pub fn set_profile_language(path: &Path, language: &str) -> Result<Profile, ConfigError> {
    let mut profile = Profile::load_or_default(path);
    profile.language = language.trim().to_ascii_lowercase();
    profile.save_to_file(path)?;
    Ok(profile)
}

// @impl:FR-CLI-14
/// Mutates specific student profile fields and saves directly to `profile.toml`.
///
/// # Errors
/// Returns [`ConfigError`] if loading or writing fails.
pub fn update_profile_fields(
    path: &Path,
    name: Option<&str>,
    nim: Option<&str>,
    class: Option<&str>,
) -> Result<Profile, ConfigError> {
    let mut profile = Profile::load_or_default(path);
    if let Some(n) = name {
        if !n.trim().is_empty() {
            profile.student.name = n.trim().to_string();
        }
    }
    if let Some(i) = nim {
        if !i.trim().is_empty() {
            profile.student.nim = i.trim().to_string();
        }
    }
    if let Some(c) = class {
        if !c.trim().is_empty() {
            profile.student.class = c.trim().to_string();
        }
    }
    profile.save_to_file(path)?;
    Ok(profile)
}

/// Formats the student metadata Typst dictionary for a course archetype.
pub fn format_student_data(course: CourseKind, name: &str, nim: &str) -> String {
    if course == CourseKind::ObjectOrientedProgramming {
        format!("#let student-data = (\n  nama: \"{name}\",\n  nim: \"{nim}\",\n)\n")
    } else {
        format!("// Data peserta utama praktikum\n#let student-data = (\n  name: \"{name}\",\n  nim: \"{nim}\",\n)\n")
    }
}
