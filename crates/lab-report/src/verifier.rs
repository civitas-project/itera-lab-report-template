// @impl:FR-CLI-10
//! Declarative coordinate geometry, font, and layout verification engine.

use crate::courses::course_root;
use crate::courses::CourseKind;
use crate::runner::stage_shared_templates;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;
use std::thread;
use thiserror::Error;

/// Errors arising during SVG layout verification.
#[derive(Debug, Error)]
pub enum VerifierError {
    /// Typst CLI tool was not found in PATH.
    #[error("Typst executable not found in PATH")]
    TypstNotFound,
    /// Typst SVG compilation process failed.
    #[error("Typst SVG compilation failed: {0}")]
    CompilationFailed(String),
    /// Filesystem I/O failure.
    #[error("I/O error during verification: {0}")]
    Io(#[from] std::io::Error),
    /// XML parsing error in SVG document.
    #[error("SVG XML parse error: {0}")]
    Xml(String),
}

/// A parsed graphical element inside an SVG canvas.
#[derive(Debug, Clone, PartialEq)]
pub struct SvgElement {
    /// XML tag name (`path`, `image`, `rect`).
    pub tag: String,
    /// Absolute X coordinate in points.
    pub x: f64,
    /// Absolute Y coordinate in points.
    pub y: f64,
    /// Bounding width in points.
    pub width: f64,
    /// Bounding height in points.
    pub height: f64,
}

/// Parsed SVG document containing viewBox bounds and graphical elements.
#[derive(Debug, Clone, PartialEq)]
pub struct SvgDocument {
    /// ViewBox coordinate quad: (min-x, min-y, width, height).
    pub viewbox: (f64, f64, f64, f64),
    /// Geometric elements contained within the page.
    pub elements: Vec<SvgElement>,
}

/// Coordinate anchor specification for layout conformance audits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorSpec {
    /// Display name of anchor object.
    pub name: &'static str,
    /// Expected XML tag.
    pub tag: &'static str,
    /// Expected X coordinate.
    pub x: f64,
    /// Expected Y coordinate.
    pub y: f64,
    /// Expected width if constrained.
    pub width: Option<f64>,
    /// Expected height if constrained.
    pub height: Option<f64>,
    /// Delta tolerance in points.
    pub tolerance: f64,
}

/// Declarative layout and typography contract for an academic course.
#[derive(Debug, Clone, PartialEq)]
pub struct CourseContract {
    /// Course unique identifier.
    pub key: &'static str,
    /// Course display title.
    pub name: &'static str,
    /// Target paper standard (e.g. ISO B5 or ISO A4).
    pub paper_format: &'static str,
    /// Expected SVG viewBox dimensions.
    pub viewbox: (f64, f64, f64, f64),
    /// List of mandatory font files required in fonts/ directory.
    pub required_fonts: &'static [&'static str],
    /// Page 1 geometric anchor elements.
    pub anchors_p1: &'static [AnchorSpec],
}

const B5_VIEWBOX: (f64, f64, f64, f64) = (0.0, 0.0, 498.8976, 708.6614);
const A4_VIEWBOX: (f64, f64, f64, f64) = (0.0, 0.0, 595.2756, 841.8898);

const B5_FONTS: &[&str] = &["GARA.TTF", "GARABD.TTF", "Arial.ttf", "Arial-Bold.ttf", "BERNHC.TTF"];
const A4_FONTS: &[&str] = &["times.ttf", "timesbd.ttf", "ariblk.ttf"];

const B5_ANCHORS: &[AnchorSpec] = &[
    AnchorSpec { name: "Pertemuan Box (Abu-Abu)", tag: "path", x: 379.80, y: 47.96, width: Some(70.34), height: Some(80.78), tolerance: 0.5 },
    AnchorSpec { name: "Title Banner (Abu-Abu)", tag: "path", x: 15.15, y: 237.52, width: Some(466.21), height: Some(97.54), tolerance: 0.5 },
    AnchorSpec { name: "Logo ITERA", tag: "image", x: 42.67, y: 584.67, width: Some(57.6), height: None, tolerance: 0.5 },
    AnchorSpec { name: "Logo FTI", tag: "image", x: 122.67, y: 616.00, width: Some(114.3), height: None, tolerance: 0.5 },
    AnchorSpec { name: "Logo Teknik Informatika", tag: "image", x: 256.67, y: 612.67, width: Some(156.9), height: None, tolerance: 0.5 },
];

const A4_ANCHORS: &[AnchorSpec] = &[
    AnchorSpec { name: "Logo ITERA Cover", tag: "image", x: 229.025, y: 217.747, width: Some(137.25), height: Some(137.25), tolerance: 0.5 },
];

impl CourseContract {
    /// Obtains predefined geometric and font contract for a course archetype.
    #[must_use]
    pub fn for_course(course: CourseKind) -> Self {
        match course {
            CourseKind::Database => CourseContract {
                key: "database",
                name: "Database Systems",
                paper_format: "ISO B5",
                viewbox: B5_VIEWBOX,
                required_fonts: B5_FONTS,
                anchors_p1: B5_ANCHORS,
            },
            CourseKind::ComputerNetworks => CourseContract {
                key: "computer-networks",
                name: "Computer Networks",
                paper_format: "ISO B5",
                viewbox: B5_VIEWBOX,
                required_fonts: B5_FONTS,
                anchors_p1: B5_ANCHORS,
            },
            CourseKind::ObjectOrientedProgramming => CourseContract {
                key: "object-oriented-programming",
                name: "Object-Oriented Programming",
                paper_format: "ISO A4",
                viewbox: A4_VIEWBOX,
                required_fonts: A4_FONTS,
                anchors_p1: A4_ANCHORS,
            },
        }
    }
}

static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[-+]?\d*\.?\d+(?:[eE][-+]?\d+)?").unwrap());
static TRANSLATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"translate\(\s*([\d\.\-]+)(?:[\s,]+([\d\.\-]+))?\s*\)").unwrap()
});
static VH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"v\s*([\d\.\-]+)\s*h\s*([\d\.\-]+)").unwrap()
});
static HV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"h\s*([\d\.\-]+)\s*v\s*([\d\.\-]+)").unwrap()
});

fn parse_translate(transform_str: &str) -> (f64, f64) {
    if let Some(caps) = TRANSLATE_RE.captures(transform_str) {
        let tx = caps.get(1).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
        let ty = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
        (tx, ty)
    } else {
        (0.0, 0.0)
    }
}

fn extract_path_dimensions(d: &str) -> (f64, f64, f64, f64) {
    if let Some(caps) = VH_RE.captures(d) {
        let h = caps.get(1).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0).abs();
        let w = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0).abs();
        return (0.0, 0.0, w, h);
    }

    if let Some(caps) = HV_RE.captures(d) {
        let w = caps.get(1).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0).abs();
        let h = caps.get(2).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0).abs();
        return (0.0, 0.0, w, h);
    }

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut is_x = true;

    for m in NUM_RE.find_iter(d) {
        if let Ok(val) = m.as_str().parse::<f64>() {
            if is_x {
                xs.push(val);
            } else {
                ys.push(val);
            }
            is_x = !is_x;
        }
    }

    if xs.is_empty() || ys.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let min_x = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let max_x = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_y = ys.iter().copied().fold(f64::INFINITY, f64::min);
    let max_y = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    (min_x, min_y, max_x - min_x, max_y - min_y)
}

fn parse_attr_f64(val: Option<&[u8]>) -> f64 {
    val.and_then(|b| std::str::from_utf8(b).ok())
        .and_then(|s| s.trim().trim_end_matches("pt").parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// Parses an SVG document string extracting canvas viewBox and geometric element positions.
///
/// # Errors
/// Returns [`VerifierError::Xml`] if XML parsing encounters a fatal syntax error.
pub fn parse_svg_string(svg_str: &str) -> Result<SvgDocument, VerifierError> {
    let mut reader = Reader::from_str(svg_str);
    reader.config_mut().trim_text(true);

    let mut viewbox = (0.0, 0.0, 0.0, 0.0);
    let mut elements = Vec::new();
    let mut buf = Vec::new();

    let mut transform_stack: Vec<(f64, f64)> = vec![(0.0, 0.0)];
    let mut inside_defs = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_raw = e.name().into_inner();
                let tag_name = String::from_utf8_lossy(tag_raw).to_string();

                if tag_name == "defs" {
                    inside_defs += 1;
                }

                let mut node_tx = 0.0;
                let mut node_ty = 0.0;

                for attr in e.attributes().filter_map(|a| a.ok()) {
                    let key = attr.key.into_inner();
                    if key == b"viewBox" && tag_name == "svg" {
                        let val_str = String::from_utf8_lossy(&attr.value);
                        let parts: Vec<f64> = val_str
                            .split_whitespace()
                            .filter_map(|p| p.parse::<f64>().ok())
                            .collect();
                        if parts.len() >= 4 {
                            viewbox = (parts[0], parts[1], parts[2], parts[3]);
                        }
                    } else if key == b"transform" {
                        let t_str = String::from_utf8_lossy(&attr.value);
                        let (tx, ty) = parse_translate(&t_str);
                        node_tx = tx;
                        node_ty = ty;
                    }
                }

                let (cum_x, cum_y) = *transform_stack.last().unwrap_or(&(0.0, 0.0));
                let cur_tx = cum_x + node_tx;
                let cur_ty = cum_y + node_ty;
                transform_stack.push((cur_tx, cur_ty));

                if inside_defs == 0 && tag_name != "svg" && tag_name != "g" && tag_name != "defs" {
                    let mut x = cur_tx;
                    let mut y = cur_ty;
                    let mut width = 0.0;
                    let mut height = 0.0;
                    let mut d_val = String::new();

                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        match attr.key.into_inner() {
                            b"x" => x = cur_tx + parse_attr_f64(Some(&attr.value)),
                            b"y" => y = cur_ty + parse_attr_f64(Some(&attr.value)),
                            b"width" => width = parse_attr_f64(Some(&attr.value)),
                            b"height" => height = parse_attr_f64(Some(&attr.value)),
                            b"d" => d_val = String::from_utf8_lossy(&attr.value).to_string(),
                            _ => {}
                        }
                    }

                    if tag_name == "path" {
                        let (px, py, pw, ph) = extract_path_dimensions(&d_val);
                        // If path defines explicit M coordinates inside d without transform
                        if (x == cur_tx && y == cur_ty) && (px != 0.0 || py != 0.0) {
                            x = cur_tx + px;
                            y = cur_ty + py;
                        }
                        if width == 0.0 {
                            width = pw;
                        }
                        if height == 0.0 {
                            height = ph;
                        }
                    }

                    elements.push(SvgElement {
                        tag: tag_name,
                        x,
                        y,
                        width,
                        height,
                    });
                }
            }
            Ok(Event::Empty(e)) => {
                let tag_raw = e.name().into_inner();
                let tag_name = String::from_utf8_lossy(tag_raw).to_string();

                if inside_defs == 0 {
                    let (cum_x, cum_y) = *transform_stack.last().unwrap_or(&(0.0, 0.0));
                    let mut node_tx = 0.0;
                    let mut node_ty = 0.0;

                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key.into_inner() == b"transform" {
                            let t_str = String::from_utf8_lossy(&attr.value);
                            let (tx, ty) = parse_translate(&t_str);
                            node_tx = tx;
                            node_ty = ty;
                        }
                    }

                    let cur_tx = cum_x + node_tx;
                    let cur_ty = cum_y + node_ty;

                    let mut x = cur_tx;
                    let mut y = cur_ty;
                    let mut width = 0.0;
                    let mut height = 0.0;
                    let mut d_val = String::new();

                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        match attr.key.into_inner() {
                            b"x" => x = cur_tx + parse_attr_f64(Some(&attr.value)),
                            b"y" => y = cur_ty + parse_attr_f64(Some(&attr.value)),
                            b"width" => width = parse_attr_f64(Some(&attr.value)),
                            b"height" => height = parse_attr_f64(Some(&attr.value)),
                            b"d" => d_val = String::from_utf8_lossy(&attr.value).to_string(),
                            _ => {}
                        }
                    }

                    if tag_name == "path" {
                        let (px, py, pw, ph) = extract_path_dimensions(&d_val);
                        if (x == cur_tx && y == cur_ty) && (px != 0.0 || py != 0.0) {
                            x = cur_tx + px;
                            y = cur_ty + py;
                        }
                        if width == 0.0 {
                            width = pw;
                        }
                        if height == 0.0 {
                            height = ph;
                        }
                    }

                    elements.push(SvgElement {
                        tag: tag_name,
                        x,
                        y,
                        width,
                        height,
                    });
                }
            }
            Ok(Event::End(e)) => {
                let tag_raw = e.name().into_inner();
                let tag_name = String::from_utf8_lossy(tag_raw).to_string();
                if tag_name == "defs" && inside_defs > 0 {
                    inside_defs -= 1;
                }
                if transform_stack.len() > 1 {
                    transform_stack.pop();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(VerifierError::Xml(format!("XML error: {e}"))),
            _ => {}
        }
        buf.clear();
    }

    Ok(SvgDocument { viewbox, elements })
}

/// Parses an SVG file from the filesystem.
///
/// # Errors
/// Returns [`VerifierError::Io`] if file cannot be read.
pub fn parse_svg_file(path: &Path) -> Result<SvgDocument, VerifierError> {
    let content = fs::read_to_string(path)?;
    parse_svg_string(&content)
}

/// Compiles a target course report into SVG pages.
///
/// # Errors
/// Returns [`VerifierError::TypstNotFound`] if typst cannot be spawned.
/// Returns [`VerifierError::CompilationFailed`] if compilation produces errors.
pub fn compile_course_to_svg(
    course_dir: &Path,
    output_dir: &Path,
) -> Result<Vec<PathBuf>, VerifierError> {
    fs::create_dir_all(output_dir)?;
    let pattern = output_dir.join("page-{0p}.svg");
    let main_typ = course_dir.join("main.typ");
    let fonts_dir = course_dir.join("fonts");

    let mut cmd = Command::new("typst");
    cmd.arg("compile");
    if fonts_dir.is_dir() {
        cmd.args(["--font-path", &fonts_dir.to_string_lossy()]);
    }
    cmd.args([main_typ.as_os_str(), pattern.as_os_str()]);

    let output = cmd.output().map_err(|_| VerifierError::TypstNotFound)?;
    if !output.status.success() {
        return Err(VerifierError::CompilationFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let mut pages: Vec<PathBuf> = fs::read_dir(output_dir)?
        .filter_map(|r| r.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|s| s.starts_with("page-") && s.ends_with(".svg"))
        })
        .collect();
    pages.sort();
    Ok(pages)
}

/// Evaluates course contracts against parsed SVG pages and font directories.
#[must_use]
pub fn verify_contract(
    contract: &CourseContract,
    pages: &[SvgDocument],
    fonts_dir: &Path,
) -> (bool, Vec<String>) {
    let mut messages = Vec::new();
    let mut success = true;

    let mut missing_fonts = Vec::new();
    for font_file in contract.required_fonts {
        if !fonts_dir.join(font_file).exists() {
            missing_fonts.push(*font_file);
        }
    }

    if !missing_fonts.is_empty() {
        messages.push(format!("Font Verification: Missing required fonts: {missing_fonts:?}"));
        success = false;
    } else {
        messages.push(format!("Font Verification: All {} required fonts present.", contract.required_fonts.len()));
    }

    if pages.is_empty() {
        messages.push("Error: No SVG pages generated to verify.".to_string());
        return (false, messages);
    }

    for (idx, page) in pages.iter().enumerate() {
        let page_num = idx + 1;
        let dx = (page.viewbox.2 - contract.viewbox.2).abs();
        let dy = (page.viewbox.3 - contract.viewbox.3).abs();

        if dx > 1.0 || dy > 1.0 {
            messages.push(format!(
                "Page {page_num}: Invalid viewBox ({:.1}x{:.1} pt, expected {:.1}x{:.1} pt)",
                page.viewbox.2, page.viewbox.3, contract.viewbox.2, contract.viewbox.3
            ));
            success = false;
        } else if page_num <= 3 {
            messages.push(format!(
                "Page {page_num}: Canvas dimensions conform to {} ({:.1}x{:.1} pt)",
                contract.paper_format, page.viewbox.2, page.viewbox.3
            ));
        }
    }

    let p1 = &pages[0];
    for spec in contract.anchors_p1 {
        let mut best_dist = f64::INFINITY;
        let mut matched = false;

        for elem in &p1.elements {
            if elem.tag == spec.tag {
                let dx = (elem.x - spec.x).abs();
                let dy = (elem.y - spec.y).abs();
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < best_dist {
                    best_dist = dist;
                }

                if dist <= spec.tolerance {
                    matched = true;
                    break;
                }
            }
        }

        if matched {
            messages.push(format!("Anchor '{}': MATCH (delta: {:.2}pt)", spec.name, best_dist));
        } else {
            messages.push(format!(
                "Anchor '{}': MISMATCH (best delta: {:.2}pt, tolerance: {:.2}pt)",
                spec.name, best_dist, spec.tolerance
            ));
            success = false;
        }
    }

    (success, messages)
}

/// Executes full declarative verification for a course directory.
///
/// # Errors
/// Returns [`VerifierError`] if SVG compilation or reading fails.
pub fn verify_course(
    workspace_root: &Path,
    course: CourseKind,
) -> Result<(bool, Vec<String>), VerifierError> {
    let course_dir = course_root(workspace_root, course);
    stage_shared_templates(workspace_root, course).map_err(VerifierError::Io)?;
    let temp_svg_dir = course_dir.join("output_svg");
    let svg_files = compile_course_to_svg(&course_dir, &temp_svg_dir)?;

    let mut pages = Vec::with_capacity(svg_files.len());
    thread::scope(|s| -> Result<(), VerifierError> {
        let mut handles = Vec::with_capacity(svg_files.len());
        for path in &svg_files {
            handles.push(s.spawn(move || parse_svg_file(path)));
        }
        for h in handles {
            pages.push(
                h.join()
                    .map_err(|_| VerifierError::Io(std::io::Error::other("SVG parser thread panicked")))??,
            );
        }
        Ok(())
    })?;

    let contract = CourseContract::for_course(course);
    let fonts_dir = course_dir.join("fonts");
    let result = verify_contract(&contract, &pages, &fonts_dir);

    // Clean up temporary SVG files
    let _ = fs::remove_dir_all(&temp_svg_dir);

    Ok(result)
}
