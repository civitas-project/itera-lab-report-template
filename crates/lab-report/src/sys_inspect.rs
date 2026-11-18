// @impl:FR-CLI-07
//! Hardware probing and environment inspection for lab report documentation.

use crate::courses::{course_root, CourseKind};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

/// Empirical host hardware specifications and toolchain versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemInfo {
    /// Processor architecture and model identifier.
    pub cpu: String,
    /// Total installed physical memory.
    pub ram: String,
    /// Primary storage capacity and disk drive model.
    pub storage: String,
    /// Host operating system distribution and Linux kernel release.
    pub os: String,
    /// Installed Typst document compiler version.
    pub typst: String,
    /// Academic course simulator or runtime environment name.
    pub simulator: String,
}

fn probe_cpu() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some((_, val)) = line.split_once(':') {
                    return val.trim().to_string();
                }
            }
        }
    }

    if let Ok(output) = Command::new("lscpu").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            for line in text.lines() {
                if line.contains("Model name:") {
                    if let Some((_, val)) = line.split_once(':') {
                        return val.trim().to_string();
                    }
                }
            }
        }
    }

    "Unknown CPU".to_string()
}

fn probe_ram() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        let gb = kb as f64 / (1024.0 * 1024.0);
                        return format!("{:.0} GB", gb.round());
                    }
                }
            }
        }
    }

    if let Ok(output) = Command::new("free").arg("-h").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            for line in text.lines() {
                if line.starts_with("Mem:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].to_string();
                    }
                }
            }
        }
    }

    "Unknown RAM".to_string()
}

pub fn format_storage_label(model: &str, lsblk_size: &str, decimal_gb: f64) -> String {
    format!(
        "{:.0} GB SSD ({}), {} menurut lsblk, disk OS",
        decimal_gb,
        model.trim(),
        lsblk_size.trim()
    )
}

/// Strips a trailing capacity token (e.g. `120GB`, `1T`) from an lsblk MODEL string.
fn strip_trailing_capacity(model: &str) -> String {
    let tokens: Vec<&str> = model.split_whitespace().collect();
    let Some(&last) = tokens.last() else {
        return model.to_string();
    };
    let suffixes = ["TB", "GB", "MB", "T", "G", "M"];
    if let Some(suf) = suffixes.iter().find(|s| {
        last.len() > s.len() && last[last.len() - s.len()..].eq_ignore_ascii_case(s)
    }) {
        let num = &last[..last.len() - suf.len()];
        if !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()) {
            return tokens[..tokens.len() - 1].join(" ");
        }
    }
    model.to_string()
}

fn probe_storage() -> String {
    let root_source = Command::new("findmnt")
        .args(["-no", "SOURCE", "/"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string());

    let disk = root_source.and_then(|source| {
        Command::new("lsblk")
            .args(["-no", "PKNAME", &source])
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_string())
    });

    let storage = disk.as_deref().and_then(|disk| {
        let decimal_gb = fs::read_to_string(format!("/sys/class/block/{disk}/size"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .map(|sectors| sectors as f64 * 512.0 / 1_000_000_000.0);

        let model_size = Command::new("lsblk")
            .args(["-d", "-o", "NAME,MODEL,SIZE"])
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .and_then(|text| {
                text.lines().find_map(|line| {
                    let mut tokens = line.split_whitespace();
                    let name = tokens.next()?;
                    if name != disk {
                        return None;
                    }
                    let size = tokens.next_back()?;
                    let model: String = tokens.collect::<Vec<_>>().join(" ");
                    Some((model, size.to_string()))
                })
            });

        match (decimal_gb, model_size) {
            (Some(decimal_gb), Some((model, size))) => {
                let clean_model = strip_trailing_capacity(&model);
                Some(format_storage_label(&clean_model, &size, decimal_gb))
            }
            _ => None,
        }
    });

    storage.unwrap_or_else(|| {
        if let Ok(output) = Command::new("lsblk")
            .args(["-d", "-o", "NAME,MODEL,SIZE"])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                for line in text.lines() {
                    let lower = line.to_lowercase();
                    if lower.contains("nvme") || lower.contains("sda") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if let Some(last) = parts.last() {
                            return format!("{} SSD", last);
                        }
                    }
                }
            }
        }

        "512 GB Primary Drive".to_string()
    })
}

fn probe_os() -> String {
    let mut pretty_name = String::new();
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some((_, val)) = line.split_once('=') {
                    pretty_name = val.trim().trim_matches('"').trim_matches('\'').to_string();
                    break;
                }
            }
        }
    }

    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown Kernel".to_string());

    if pretty_name.is_empty() {
        pretty_name = "Linux".to_string();
    }

    format!("{pretty_name} (Kernel {kernel})")
}

fn probe_typst() -> String {
    Command::new("typst")
        .arg("--version")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Typst (latest)".to_string())
}

fn probe_simulator(course: CourseKind) -> String {
    match course {
        CourseKind::ComputerNetworks => "Cisco Packet Tracer v8.2+ (Host/Distrobox)".to_string(),
        CourseKind::Database => {
            if Command::new("which").arg("psql").output().map(|o| o.status.success()).unwrap_or(false) {
                "PostgreSQL DBMS CLI & Client".to_string()
            } else if Command::new("which").arg("mysql").output().map(|o| o.status.success()).unwrap_or(false) {
                "MySQL DBMS CLI & Client".to_string()
            } else {
                "PostgreSQL / MySQL RDBMS".to_string()
            }
        }
        CourseKind::ObjectOrientedProgramming => {
            Command::new("java")
                .arg("--version")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .and_then(|text| text.lines().next().map(|s| s.trim().to_string()))
                .unwrap_or_else(|| "Java OpenJDK Runtime & Compiler".to_string())
        }
    }
}

/// Probes host hardware and runtime environment empirical attributes.
#[must_use]
pub fn inspect_system(course: CourseKind) -> SystemInfo {
    let results: Vec<String> = thread::scope(|s| {
        let handles = vec![
            s.spawn(probe_cpu),
            s.spawn(probe_ram),
            s.spawn(probe_storage),
            s.spawn(probe_os),
            s.spawn(probe_typst),
            s.spawn(move || probe_simulator(course)),
        ];
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let [cpu, ram, storage, os, typst, simulator] = results.try_into().unwrap();

    SystemInfo {
        cpu,
        ram,
        storage,
        os,
        typst,
        simulator,
    }
}

/// Formats system specifications into Typst `#let tools-data = (...)` tabular block.
#[must_use]
pub fn format_typst_tools_data(info: &SystemInfo, course: CourseKind) -> String {
    let sim_label = match course {
        CourseKind::ComputerNetworks => "Simulator Jaringan",
        CourseKind::Database => "Sistem Basis Data (DBMS)",
        CourseKind::ObjectOrientedProgramming => "Lingkungan Pemrograman",
    };

    vec![
        "// Otomatis diinspeksi oleh lab-report sys_inspect".to_string(),
        "#import \"../template/lib.typ\": tabhead, tabband, tabline".to_string(),
        "#let tools-data = (".to_string(),
        format!("  (\"1\", \"Prosesor (CPU)\", \"{}\"),", info.cpu),
        format!("  (\"2\", \"Memori (RAM)\", \"{}\"),", info.ram),
        format!("  (\"3\", \"Penyimpanan\", \"{}\"),", info.storage),
        format!("  (\"4\", \"Sistem Operasi\", \"{}\"),", info.os),
        format!("  (\"5\", \"{sim_label}\", \"{}\"),", info.simulator),
        format!("  (\"6\", \"Document Engine\", \"{}\"),", info.typst),
        ")".to_string(),
        "".to_string(),
        "#let render-tools-and-materials() = {".to_string(),
        "  align(center)[".to_string(),
        "    #set text(font: \"Garamond\", size: 18pt, weight: \"bold\")".to_string(),
        "    Alat dan Bahan".to_string(),
        "  ]".to_string(),
        "  v(4pt)".to_string(),
        "  align(center)[".to_string(),
        "    #table(".to_string(),
        "      columns: (30pt, 120pt, 1fr),".to_string(),
        "      align: (center, left, left),".to_string(),
        "      fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },".to_string(),
        "      stroke: 0.5pt + tabline,".to_string(),
        "      [No], [Komponen / Perangkat], [Spesifikasi / Versi],".to_string(),
        "      ..tools-data.map(row => (".to_string(),
        "        row.at(0),".to_string(),
        "        row.at(1),".to_string(),
        "        row.at(2),".to_string(),
        "      )).flatten()".to_string(),
        "    )".to_string(),
        "  ]".to_string(),
        "}".to_string(),
        "".to_string(),
        "#render-tools-and-materials()".to_string(),
    ]
    .join("\n")
}

/// Writes inspected hardware specifications directly to `content/tools-and-materials.typ`.
///
/// # Errors
/// Returns [`std::io::Error`] if file write fails.
pub fn update_tools_file(
    workspace_root: &Path,
    info: &SystemInfo,
    course: CourseKind,
) -> Result<PathBuf, std::io::Error> {
    let target = course_root(workspace_root, course)
        .join("content")
        .join("tools-and-materials.typ");

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    let rendered = format_typst_tools_data(info, course);
    fs::write(&target, rendered)?;
    Ok(target)
}
