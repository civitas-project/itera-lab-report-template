// @impl:FR-CLI-05
//! Submission archive packaging engine for lab reports.

use crate::config::Profile;
use crate::courses::course_root;
use crate::courses::CourseKind;
use crate::naming::format_artifact_filename;
use crate::runner::compile_report;
use std::fs::{self, File};
use walkdir::WalkDir;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

/// Errors arising during submission archive creation.
#[derive(Debug, Error)]
pub enum PackError {
    /// File read or write error at system I/O boundary.
    #[error("I/O error during archive packing: {0}")]
    Io(#[from] std::io::Error),
    /// Zip formatting error during compression or writing.
    #[error("Zip compression error: {0}")]
    Zip(#[from] zip::result::ZipError),
    /// Artifact naming format failure.
    #[error("Naming error: {0}")]
    Naming(#[from] crate::naming::NamingError),
    /// Compilation runner error when building prerequisite PDF.
    #[error("Runner error: {0}")]
    Runner(#[from] crate::runner::RunnerError),
    /// Expected PDF deliverable not found on disk.
    #[error("Compiled PDF report not found: {0}")]
    MissingPdf(PathBuf),
}

/// Packages compiled lab report PDF and accompanying source code into a compressed ZIP file.
///
/// # Errors
/// Returns [`PackError::MissingPdf`] if target PDF does not exist and compilation is skipped.
/// Returns [`PackError::Io`] or [`PackError::Zip`] if filesystem operations fail.
pub fn pack_submission(
    course: CourseKind,
    workspace_root: &Path,
    profile: &Profile,
    module_num: &str,
    out_dir: Option<&Path>,
    skip_compile: bool,
) -> Result<PathBuf, PackError> {
    let course_dir = course_root(workspace_root, course);
    let pdf_filename = format_artifact_filename(
        &profile.naming.pattern,
        &profile.student.name,
        &profile.student.nim,
        module_num,
        "pdf",
    )?;
    let pdf_path = course_dir.join("dist").join(&pdf_filename);

    if !skip_compile && !pdf_path.exists() {
        compile_report(course, workspace_root, profile, module_num, Some(&pdf_path), "pdf", false, &[])?;
    }

    if !pdf_path.exists() {
        return Err(PackError::MissingPdf(pdf_path));
    }

    let target_dir = match out_dir {
        Some(d) => d.to_path_buf(),
        None => course_dir.join("dist"),
    };
    fs::create_dir_all(&target_dir)?;

    let stem = pdf_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("submission");
    let zip_path = target_dir.join(format!("{stem}.zip"));

    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file(&pdf_filename, options)?;
    let mut pdf_bytes = Vec::new();
    File::open(&pdf_path)?.read_to_end(&mut pdf_bytes)?;
    zip.write_all(&pdf_bytes)?;

    let src_dir = course_dir.join("src");
    if src_dir.is_dir() {
        let mut entries: Vec<PathBuf> = WalkDir::new(&src_dir)
            .into_iter()
            .filter_map(|res| res.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect();
        entries.sort();

        for file_path in entries {
            if file_path.file_name().and_then(|n| n.to_str()) == Some(".gitkeep") {
                continue;
            }
            let rel_path = file_path
                .strip_prefix(&src_dir)
                .unwrap_or(&file_path)
                .to_string_lossy()
                .replace('\\', "/");
            if rel_path.is_empty() {
                continue;
            }
            zip.start_file(&rel_path, options)?;
            let mut file_bytes = Vec::new();
            File::open(&file_path)?.read_to_end(&mut file_bytes)?;
            zip.write_all(&file_bytes)?;
        }
    }

    zip.finish()?;
    Ok(zip_path)
}
