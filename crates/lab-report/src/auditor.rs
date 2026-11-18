//! Project audit: GAP_REGISTER gap handling plus ZIP-vs-PDF byte freshness.

use std::fs;
use std::io::Read;
use std::path::Path;

use crate::courses::{course_root, CourseKind};

const GAP_FILE: &str = "GAP_REGISTER.md";
const DIST_DIR: &str = "dist";

/// Audits a course project.
///
/// Checks every `GR-*` row in `<course>/GAP_REGISTER.md`:
/// - A stale ERROR is raised only for rows whose `Fase` parses to >= 6 and
///   whose `Status` is not `ditangani`. Rows with `Keputusan == "no-go"` are
///   warnings only and never produce a stale ERROR, regardless of `Fase`.
/// - Then scans `dist/` for each `*.pdf` and compares its bytes against the
///   same-named entry in a sibling `<stem>.zip`. Mismatch is an error, match
///   is an OK line, missing ZIP is a warning.
///
/// # Errors
/// Returns an [`std::io::Error`] when project files cannot be read (for
/// example permission denied on `dist/`). Malformed ZIP archives are
/// reported as message lines instead of failing the call.
pub fn audit_project(
    workspace_root: &Path,
    course: CourseKind,
) -> std::io::Result<(bool, Vec<String>)> {
    let course_dir = course_root(workspace_root, course);
    let gap_path = course_dir.join(GAP_FILE);
    let mut messages: Vec<String> = Vec::new();
    let mut ok = true;

    if !gap_path.is_file() {
        messages.push(format!(
            "Error: GAP_REGISTER not found at {}",
            gap_path.display()
        ));
        ok = false;
    } else {
        let content = fs::read_to_string(&gap_path)?;
        for line in content.lines() {
            let line = line.trim();
            if !line.starts_with("| GR-") {
                continue;
            }
            let cols: Vec<&str> = line
                .split('|')
                .map(|c| c.trim())
                .filter(|c| !c.is_empty())
                .collect();
            if cols.len() < 10 {
                messages.push(format!(
                    "Error: GAP_REGISTER malformed row (expected 10 columns): {line}"
                ));
                ok = false;
                continue;
            }

            let id = cols[0];
            let fase = cols[1];
            let kategori = cols[3];
            let bukti = cols[4];
            let status = cols[5];
            let keputusan = cols[7];
            let catatan = cols[8];

            let fase_num: Option<u64> = fase.parse().ok();
            let is_no_go = keputusan == "no-go";

            // Stale ERROR applies only to Fase >= 6, and never to no-go rows.
            if !is_no_go {
                if let Some(n) = fase_num {
                    if n >= 6 && status != "ditangani" {
                        messages.push(format!(
                            "Error: GAP_REGISTER item {id} is stale: status `{status}` is not `ditangani`"
                        ));
                        ok = false;
                    }
                }
            }

            if is_no_go {
                messages.push(format!(
                    "GAP_REGISTER item {id} has keputusan `no-go` (warning only): {catatan}"
                ));
            }
            if kategori.is_empty() || kategori == "-" {
                messages.push(format!(
                    "GAP_REGISTER item {id} has empty kategori (warning only)"
                ));
            }
            if bukti.is_empty() || bukti == "-" {
                messages.push(format!(
                    "GAP_REGISTER item {id} has empty bukti (warning only)"
                ));
            }
        }
    }

    // ZIP-vs-PDF freshness: dynamic scan of every *.pdf directly in dist/.
    let dist_dir = course_dir.join(DIST_DIR);
    if dist_dir.is_dir() {
        let mut pdf_entries: Vec<_> = fs::read_dir(&dist_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "pdf"))
            .collect();
        pdf_entries.sort_by_key(|e| e.file_name());
        for entry in pdf_entries {
            let pdf_path = entry.path();
            let pdf_name = pdf_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let stem = pdf_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let zip_path = dist_dir.join(format!("{stem}.zip"));

            if zip_path.is_file() {
                let pdf_bytes = fs::read(&pdf_path)?;
                match fs::File::open(&zip_path) {
                    Ok(zip_file) => match zip::ZipArchive::new(zip_file) {
                        Ok(mut archive) => {
                            let entry_bytes = match archive.by_name(&pdf_name) {
                                Ok(mut entry) => {
                                    let mut buf = Vec::new();
                                    match entry.read_to_end(&mut buf) {
                                        Ok(_) => Some(buf),
                                        Err(_) => None,
                                    }
                                }
                                Err(_) => None,
                            };
                            match entry_bytes {
                                Some(bytes) if bytes == pdf_bytes => {
                                    messages.push(format!(
                                        "ZIP freshness OK: {} matches entry in {}",
                                        pdf_name,
                                        zip_path.display()
                                    ));
                                }
                                Some(_) => {
                                    messages.push(format!(
                                        "Error: zip mismatch: {} entry `{pdf_name}` is stale relative to {} (bytes differ)",
                                        zip_path.display(),
                                        pdf_path.display()
                                    ));
                                    ok = false;
                                }
                                None => {
                                    messages.push(format!(
                                        "Error: zip mismatch: {} missing entry `{pdf_name}`",
                                        zip_path.display()
                                    ));
                                    ok = false;
                                }
                            }
                        }
                        Err(_) => {
                            messages.push(format!(
                                "Error: zip mismatch: {} is not a valid archive",
                                zip_path.display()
                            ));
                            ok = false;
                        }
                    },
                    Err(_) => {
                        messages.push(format!(
                            "Error: zip mismatch: {} cannot be opened",
                            zip_path.display()
                        ));
                        ok = false;
                    }
                }
            } else {
                messages.push(format!(
                    "ZIP freshness warning: no ZIP sibling for {}",
                    pdf_name
                ));
            }
        }
    } else {
        messages.push(format!(
            "ZIP freshness warning: dist directory {} does not exist",
            dist_dir.display()
        ));
    }

    Ok((ok, messages))
}
