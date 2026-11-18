// @impl:FR-CLI-03
//! Artifact filename formatting and token interpolation.

use thiserror::Error;

/// Errors encountered during artifact filename generation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum NamingError {
    /// Parameter value is empty or whitespace.
    #[error("Required parameter is empty: {0}")]
    EmptyParameter(&'static str),
}

/// Generates sanitized submission filename with token interpolation.
///
/// # Errors
/// Returns [`NamingError::EmptyParameter`] if `name` or `nim` is empty.
pub fn format_artifact_filename(
    pattern: &str,
    name: &str,
    nim: &str,
    module: &str,
    extension: &str,
) -> Result<String, NamingError> {
    let trimmed_name = name.trim();
    if trimmed_name.is_empty() {
        return Err(NamingError::EmptyParameter("name"));
    }

    let trimmed_nim = nim.trim();
    if trimmed_nim.is_empty() {
        return Err(NamingError::EmptyParameter("nim"));
    }

    let sanitized_name = trimmed_name.replace(' ', "_");
    let base = pattern
        .replace("{name}", &sanitized_name)
        .replace("{nim}", trimmed_nim)
        .replace("{module}", module.trim());

    let clean_ext = extension.trim().trim_start_matches('.');
    Ok(format!("{base}.{clean_ext}"))
}
