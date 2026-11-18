use clap::{Parser, Subcommand};
use lab_report::config::{update_profile_fields, Profile};
use lab_report::courses::{course_root, detect_course_from_metadata, detect_course_from_path, find_workspace_root, resolve_course_key, CourseKind};
use lab_report::diagrams::{render_all_diagrams, render_puml};
use lab_report::generator::scaffold_project;
use lab_report::linter::lint_course_with_options;
use lab_report::packer::pack_submission;
use lab_report::runner::{compile_report, format_remediation_message_bilingual, sync_student_data};
use lab_report::sys_inspect::{format_typst_tools_data, inspect_system, update_tools_file};
use lab_report::verifier::verify_course;
use lab_report::auditor::audit_project;
use lab_report::i18n::{msg_auto_detected, msg_compiling, msg_course_missing, msg_linting, msg_packaging, msg_verifying, msg_watching, tr, Language, MessageKey};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "lab-report", version, about = "Fast native CLI toolchain for ITERA lab reports")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile lab report to PDF or SVG
    Build {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
        /// Module or session number
        #[arg(short = 'm', long = "modul", default_value = "1")]
        module: String,
        /// Watch for changes and recompile
        #[arg(short = 'w', long = "watch")]
        watch: bool,
        /// Custom output file path
        #[arg(short = 'o', long = "out")]
        out: Option<PathBuf>,
        /// Export format ('pdf' or 'svg')
        #[arg(short = 'f', long = "format", default_value = "pdf")]
        format: String,
        /// Shorthand to export as multi-page SVG
        #[arg(long = "svg")]
        svg: bool,
    },
    /// Watch mode with auto recompile
    Watch {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
        /// Module or session number
        #[arg(short = 'm', long = "modul", default_value = "1")]
        module: String,
        /// Export format ('pdf' or 'svg')
        #[arg(short = 'f', long = "format", default_value = "pdf")]
        format: String,
        /// Shorthand to export as multi-page SVG
        #[arg(long = "svg")]
        svg: bool,
    },
    /// Package report and code into submission ZIP
    Pack {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
        /// Module or session number
        #[arg(short = 'm', long = "modul", default_value = "1")]
        module: String,
        /// Custom output directory for ZIP archive
        #[arg(long = "out-dir")]
        out_dir: Option<PathBuf>,
        /// Skip compilation if PDF already exists
        #[arg(long = "skip-compile")]
        skip_compile: bool,
    },
    /// Run declarative coordinate geometry and font verification
    Test {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
    },
    /// Audit gap state and submission package freshness
    Audit {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
    },
    /// Audit typography invariants and template placeholders
    Lint {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
        /// Automatically correct safe typography violations in-place
        #[arg(long = "fix")]
        fix: bool,
        /// Preview prospective changes without modifying disk contents
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
    /// Inspect real host hardware specs for tools-and-materials.typ
    Inspect {
        /// Target course (e.g., database, computer-networks, pbo) - auto-detected if omitted
        course: Option<String>,
        /// Write inspected hardware specifications directly to tools-and-materials.typ
        #[arg(long = "write")]
        write: bool,
    },
    /// Compile PlantUML (.puml) diagrams into vector SVG assets
    Render {
        /// Target course or workspace directory
        course: Option<String>,
        /// Specific PlantUML source file path
        #[arg(short = 'p', long = "path")]
        path: Option<PathBuf>,
    },
    /// Scaffold a new lab report starter project
    New {
        /// Archetype course (e.g., database, computer-networks, pbo)
        course: String,
        /// Destination directory path
        dest: Option<PathBuf>,
    },
    /// Inspect or update student profile configuration
    Profile {
        /// Display current student configuration values
        #[arg(long, default_value = "true")]
        show: bool,
        /// Update student full name
        #[arg(long)]
        name: Option<String>,
        /// Update student NIM
        #[arg(long)]
        nim: Option<String>,
        /// Update student class (e.g. RA, RB)
        #[arg(long)]
        class: Option<String>,
        /// Update preferred interface language (e.g. en, id)
        #[arg(long)]
        lang: Option<String>,
    },
    /// First-run bilingual interactive setup wizard
    Init,
    /// Launch Model Context Protocol (MCP) JSON-RPC server over stdio
    Mcp,
    /// Launch interactive terminal user interface (TUI)
    Tui,
}

fn resolve_target_course(supplied: Option<String>, root: &Path, lang: Language) -> Result<CourseKind, ExitCode> {
    if let Some(key) = supplied {
        resolve_course_key(&key).map_err(|e| {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        })
    } else if let Ok(cwd) = std::env::current_dir() {
        if let Some(detected) = detect_course_from_path(&cwd) {
            println!("{}", msg_auto_detected(lang, detected.display_name()));
            Ok(detected)
        } else if let Some(detected) = detect_course_from_metadata(root) {
            println!("{}", msg_auto_detected(lang, detected.display_name()));
            Ok(detected)
        } else {
            eprintln!("{}", msg_course_missing(lang));
            Err(ExitCode::FAILURE)
        }
    } else {
        eprintln!("{}", msg_course_missing(lang));
        Err(ExitCode::FAILURE)
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let root = find_workspace_root(&cwd).unwrap_or_else(|| PathBuf::from("."));
    let profile_path = root.join("profile.toml");
    let mut profile = Profile::load_or_default(&profile_path);
    let lang = profile.get_language();

    let command = match cli.command {
        Some(cmd) => cmd,
        None => Commands::Tui,
    };

    match command {
        Commands::Build { course, module, watch, out, format, svg } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            let chosen_format = if svg { "svg" } else { &format };

            println!("{}", msg_compiling(lang, course_kind.display_name(), &module, chosen_format));
            match compile_report(course_kind, &root, &profile, &module, out.as_deref(), chosen_format, watch, &[]) {
                Ok(_) => {
                    println!("{}", tr(lang, MessageKey::CompilationSuccess));
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("{}", format_remediation_message_bilingual(&err, lang));
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Watch { course, module, format, svg } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            let chosen_format = if svg { "svg" } else { &format };

            println!("{}", msg_watching(lang, course_kind.display_name(), &module, chosen_format));
            match compile_report(course_kind, &root, &profile, &module, None, chosen_format, true, &[]) {
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("{}", format_remediation_message_bilingual(&err, lang));
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Pack { course, module, out_dir, skip_compile } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            println!("{}", msg_packaging(lang, course_kind.display_name()));
            match pack_submission(course_kind, &root, &profile, &module, out_dir.as_deref(), skip_compile) {
                Ok(path) => {
                    println!("{} ({})", tr(lang, MessageKey::PackSuccess), path.display());
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Test { course } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            println!("{}", msg_verifying(lang, course_kind.display_name()));
            match verify_course(&root, course_kind) {
                Ok((success, messages)) => {
                    for msg in messages {
                        let prefix = if msg.contains("MATCH") || msg.contains("conform") || msg.contains("present") {
                            "  [OK]"
                        } else {
                            "  [FAIL]"
                        };
                        println!("{prefix} {msg}");
                    }
                    if success {
                        println!("{}", tr(lang, MessageKey::VerificationPassed));
                        ExitCode::SUCCESS
                    } else {
                        eprintln!("{}", tr(lang, MessageKey::VerificationFailed));
                        ExitCode::FAILURE
                    }
                }
                Err(err) => {
                    eprintln!("Error during verification: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Audit { course } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            match audit_project(&root, course_kind) {
                Ok((success, messages)) => {
                    for msg in messages {
                        if msg.contains("Error") || msg.contains("belum memiliki") || msg.contains("berbeda dengan") {
                            eprintln!("  [FAIL] {msg}");
                        } else {
                            println!("  [OK] {msg}");
                        }
                    }
                    if success { ExitCode::SUCCESS } else { ExitCode::FAILURE }
                }
                Err(err) => {
                    eprintln!("Error during audit: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Lint { course, fix, dry_run } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            let course_dir = course_root(&root, course_kind);
            println!("{}", msg_linting(lang, course_kind.display_name(), dry_run));

            match lint_course_with_options(&course_dir, fix, dry_run) {
                Ok(issues) => {
                    if issues.is_empty() {
                        println!("{}", tr(lang, MessageKey::LintPassed));
                        ExitCode::SUCCESS
                    } else {
                        let prefix = if dry_run { "[DRY RUN] " } else { "" };
                        eprintln!(">> {prefix}{}: Found {} issue(s) in {}:", tr(lang, MessageKey::LintFailed), issues.len(), course_kind.display_name());
                        for issue in issues {
                            eprintln!("  [{}] {}:{} -> {}", issue.rule_id, issue.file_path.display(), issue.line_num, issue.message);
                        }
                        if dry_run {
                            println!(">> Prospective fixes simulated. Re-run with '--fix' to apply changes to disk.");
                            ExitCode::SUCCESS
                        } else {
                            ExitCode::FAILURE
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error during lint: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Inspect { course, write } => {
            let course_kind = match resolve_target_course(course, &root, lang) {
                Ok(k) => k,
                Err(code) => return code,
            };

            let info = inspect_system(course_kind);
            if write {
                match update_tools_file(&root, &info, course_kind) {
                    Ok(path) => {
                        println!(">> Successfully updated {}", path.display());
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("Error writing tools file: {err}");
                        ExitCode::FAILURE
                    }
                }
            } else {
                println!("{}", format_typst_tools_data(&info, course_kind));
                ExitCode::SUCCESS
            }
        }
        Commands::Render { course, path } => {
            if let Some(p) = path {
                println!("==> Rendering PlantUML diagram: {}", p.display());
                match render_puml(&p, None) {
                    Ok(svg) => {
                        println!(">> Created vector asset: {}", svg.display());
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("Error rendering diagram: {err}");
                        ExitCode::FAILURE
                    }
                }
            } else {
                let target_dir = match course {
                    Some(c) => match resolve_course_key(&c) {
                        Ok(k) => course_root(&root, k),
                        Err(err) => {
                            eprintln!("Error: {err}");
                            return ExitCode::FAILURE;
                        }
                    },
                    None => root.clone(),
                };

                println!("==> Scanning and compiling diagrams in {}...", target_dir.display());
                match render_all_diagrams(&target_dir) {
                    Ok(rendered) => {
                        println!(">> Compiled {} diagram(s).", rendered.len());
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("Error rendering diagrams: {err}");
                        ExitCode::FAILURE
                    }
                }
            }
        }
        Commands::New { course, dest } => {
            let course_kind = match resolve_course_key(&course) {
                Ok(k) => k,
                Err(err) => {
                    eprintln!("Error: {err}");
                    return ExitCode::FAILURE;
                }
            };

            let target_path = match dest {
                Some(d) => d,
                None => root.join(format!("{}-new", course_kind.dir_name())),
            };

            println!("==> Scaffolding new {} project at {}...", course_kind.display_name(), target_path.display());
            match scaffold_project(course_kind, &root, &target_path, &profile) {
                Ok(created) => {
                    println!(">> Successfully scaffolded project at {}", created.display());
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("Error scaffolding project: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Profile { show, name, nim, class, lang } => {
            let has_mutations = name.is_some() || nim.is_some() || class.is_some() || lang.is_some();
            if has_mutations {
                if let Some(l) = lang.as_deref() {
                    let _ = lab_report::config::set_profile_language(&profile_path, l);
                }
                match update_profile_fields(&profile_path, name.as_deref(), nim.as_deref(), class.as_deref()) {
                    Ok(p) => {
                        profile = p;
                        println!(">> Profile updated successfully in {}", profile_path.display());
                        if let Err(e) = sync_student_data(&root, &profile) {
                            eprintln!("Warning: Failed to sync student metadata: {e}");
                        }
                    }
                    Err(err) => {
                        eprintln!("Error updating profile: {err}");
                        return ExitCode::FAILURE;
                    }
                }
            }

            if show || !has_mutations {
                println!("Student Name:  {}", profile.student.name);
                println!("Student NIM:   {}", profile.student.nim);
                println!("Student Class: {}", profile.student.class);
                println!("Language:      {}", profile.language);
                println!("File Pattern:  {}", profile.naming.pattern);
            }
            ExitCode::SUCCESS
        }
        Commands::Init => {
            let code = lab_report::tui::run_init_wizard(
                &mut std::io::stdin().lock(),
                &mut std::io::stdout(),
                &root,
            );
            if code == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Commands::Mcp => {
            match lab_report::mcp::run_mcp_server(&root) {
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("Error in MCP server: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Tui => {
            let code = lab_report::tui::run_tui_loop(
                &mut std::io::stdin().lock(),
                &mut std::io::stdout(),
                &root,
            );
            if code == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
