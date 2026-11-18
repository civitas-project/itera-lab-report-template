// @impl:FR-CLI-12
//! Interactive Terminal User Interface (TUI) menu loop.

use crate::config::Profile;
use crate::courses::CourseKind;
use crate::generator::scaffold_project;
use crate::packer::pack_submission;
use crate::runner::compile_report;
use crate::verifier::verify_course;
use std::io::{BufRead, Write};
use std::path::Path;

fn read_line<R: BufRead>(reader: &mut R) -> Option<String> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}

/// Renders header banner with student profile summary.
pub fn render_banner<W: Write>(writer: &mut W, profile: &Profile) -> std::io::Result<()> {
    writeln!(writer, "==========================================================")?;
    writeln!(writer, "  Lab Report CLI & TUI - ITERA Informatics Engineering   ")?;
    writeln!(writer, "  Name:  {} | NIM: {}", profile.student.name, profile.student.nim)?;
    writeln!(writer, "  Class: {} | Pattern: {}", profile.student.class, profile.naming.pattern)?;
    writeln!(writer, "==========================================================")?;
    Ok(())
}

fn handle_course_menu<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    course: CourseKind,
    workspace_root: &Path,
    profile: &Profile,
) -> std::io::Result<()> {
    write!(writer, "\nModule / Session Number for {} [1]: ", course.display_name())?;
    writer.flush()?;
    let module_input = read_line(reader).unwrap_or_default();
    let module_num = if module_input.is_empty() { "1" } else { &module_input };

    loop {
        writeln!(writer, "\nAction for {} (Module {module_num}):", course.display_name())?;
        writeln!(writer, "  [1] Live Watch (Auto recompile on edit)")?;
        writeln!(writer, "  [2] Build PDF (Compile to submission PDF)")?;
        writeln!(writer, "  [3] Build SVG (Vector Preview)")?;
        writeln!(writer, "  [4] Pack Submission ZIP (PDF + code)")?;
        writeln!(writer, "  [5] Run Tests & Verification")?;
        writeln!(writer, "  [0] Back to Main Menu")?;
        write!(writer, "Select action [0-5]: ")?;
        writer.flush()?;

        let choice = match read_line(reader) {
            Some(c) => c,
            None => break,
        };

        match choice.as_str() {
            "1" => {
                writeln!(writer, "==> Starting watch mode for {}...", course.display_name())?;
                let _ = compile_report(course, workspace_root, profile, module_num, None, "pdf", true, &[]);
                break;
            }
            "2" => {
                writeln!(writer, "==> Compiling PDF for {}...", course.display_name())?;
                match compile_report(course, workspace_root, profile, module_num, None, "pdf", false, &[]) {
                    Ok(_) => writeln!(writer, ">> Compilation successful.")?,
                    Err(e) => writeln!(writer, "Error: {e}")?,
                }
            }
            "3" => {
                writeln!(writer, "==> Compiling SVG for {}...", course.display_name())?;
                match compile_report(course, workspace_root, profile, module_num, None, "svg", false, &[]) {
                    Ok(_) => writeln!(writer, ">> SVG Export successful.")?,
                    Err(e) => writeln!(writer, "Error: {e}")?,
                }
            }
            "4" => {
                writeln!(writer, "==> Packaging submission archive...")?;
                match pack_submission(course, workspace_root, profile, module_num, None, false) {
                    Ok(p) => writeln!(writer, ">> Generated archive: {}", p.display())?,
                    Err(e) => writeln!(writer, "Error: {e}")?,
                }
            }
            "5" => {
                writeln!(writer, "==> Running declarative verification...")?;
                match verify_course(workspace_root, course) {
                    Ok((true, msgs)) => {
                        for m in msgs {
                            writeln!(writer, "  [OK] {m}")?;
                        }
                        writeln!(writer, ">> All tests passed!")?;
                    }
                    Ok((false, msgs)) => {
                        for m in msgs {
                            writeln!(writer, "  {m}")?;
                        }
                        writeln!(writer, ">> Verification failed.")?;
                    }
                    Err(e) => writeln!(writer, "Error: {e}")?,
                }
            }
            "0" | "" => break,
            other => writeln!(writer, "Invalid choice: {other}")?,
        }
    }

    Ok(())
}

fn handle_new_project<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    workspace_root: &Path,
    profile: &Profile,
) -> std::io::Result<()> {
    writeln!(writer, "\nScaffold New Lab Report:")?;
    writeln!(writer, "  [1] Database Systems")?;
    writeln!(writer, "  [2] Computer Networks")?;
    writeln!(writer, "  [3] Object-Oriented Programming")?;
    write!(writer, "Choose course [1-3]: ")?;
    writer.flush()?;

    let course = match read_line(reader).as_deref() {
        Some("1") => CourseKind::Database,
        Some("2") => CourseKind::ComputerNetworks,
        Some("3") => CourseKind::ObjectOrientedProgramming,
        _ => {
            writeln!(writer, "Aborted.")?;
            return Ok(());
        }
    };

    write!(writer, "Target directory path: ")?;
    writer.flush()?;
    let dest_input = match read_line(reader) {
        Some(s) if !s.is_empty() => s,
        _ => {
            writeln!(writer, "Directory path required. Aborted.")?;
            return Ok(());
        }
    };

    let target_dir = workspace_root.join(dest_input);
    match scaffold_project(course, workspace_root, &target_dir, profile) {
        Ok(created) => writeln!(writer, ">> Successfully scaffolded project at {}", created.display())?,
        Err(e) => writeln!(writer, "Error scaffolding project: {e}")?,
    }

    Ok(())
}

/// Runs interactive terminal UI loop reading commands from reader and printing to writer.
pub fn run_tui_loop<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    workspace_root: &Path,
) -> i32 {
    let profile_path = workspace_root.join("profile.toml");
    let profile = Profile::load_or_default(&profile_path);

    let _ = render_banner(writer, &profile);

    loop {
        let _ = writeln!(writer, "\nSelect Action or Lab Course:");
        let _ = writeln!(writer, "  [1] Database Systems (database)");
        let _ = writeln!(writer, "  [2] Computer Networks (computer-networks)");
        let _ = writeln!(writer, "  [3] Object-Oriented Programming (object-oriented-programming)");
        let _ = writeln!(writer, "  [4] New Lab Report (Starter Project Generator)");
        let _ = writeln!(writer, "  [5] View Student Profile");
        let _ = writeln!(writer, "  [0] Exit");
        let _ = write!(writer, "Choose option [0-5]: ");
        let _ = writer.flush();

        let choice = match read_line(reader) {
            Some(c) => c,
            None => break,
        };

        match choice.as_str() {
            "1" => {
                let _ = handle_course_menu(reader, writer, CourseKind::Database, workspace_root, &profile);
            }
            "2" => {
                let _ = handle_course_menu(reader, writer, CourseKind::ComputerNetworks, workspace_root, &profile);
            }
            "3" => {
                let _ = handle_course_menu(reader, writer, CourseKind::ObjectOrientedProgramming, workspace_root, &profile);
            }
            "4" => {
                let _ = handle_new_project(reader, writer, workspace_root, &profile);
            }
            "5" => {
                let _ = writeln!(writer, "\nCurrent Profile:");
                let _ = writeln!(writer, "  Name:    {}", profile.student.name);
                let _ = writeln!(writer, "  NIM:     {}", profile.student.nim);
                let _ = writeln!(writer, "  Class:   {}", profile.student.class);
                let _ = writeln!(writer, "  Pattern: {}", profile.naming.pattern);
            }
            "0" | "exit" | "q" => break,
            other => {
                let _ = writeln!(writer, "Invalid option: {other}");
            }
        }
    }

    let _ = writeln!(writer, "Goodbye!");
    0
}

// @impl:FR-CLI-19
/// Executes the interactive bilingual first-run wizard `lab-report init`.
pub fn run_init_wizard<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    workspace_root: &Path,
) -> i32 {
    use crate::i18n::{tr, Language, MessageKey};

    let profile_path = workspace_root.join("profile.toml");
    let existing = Profile::load_or_default(&profile_path);

    let _ = writeln!(writer, "==========================================================");
    let _ = writeln!(writer, "  {}  ", tr(Language::En, MessageKey::Welcome));
    let _ = writeln!(writer, "  {}  ", tr(Language::Id, MessageKey::Welcome));
    let _ = writeln!(writer, "==========================================================");

    let _ = writeln!(writer, "\nLanguage Selection / Pemilihan Bahasa:");
    let _ = writeln!(writer, "  [1] English (default)");
    let _ = writeln!(writer, "  [2] Bahasa Indonesia");
    let _ = write!(writer, "Choice [1-2]: ");
    let _ = writer.flush();

    let lang_choice = read_line(reader).unwrap_or_default();
    let lang = Language::from_input(&lang_choice);

    let mut updated_profile = existing;

    let _ = writeln!(writer);
    let _ = write!(writer, "{}: [{}] ", tr(lang, MessageKey::NamePrompt), updated_profile.student.name);
    let _ = writer.flush();
    let name_input = read_line(reader).unwrap_or_default();
    if !name_input.is_empty() {
        updated_profile.student.name = name_input;
    }

    let _ = write!(writer, "{}: [{}] ", tr(lang, MessageKey::NimPrompt), updated_profile.student.nim);
    let _ = writer.flush();
    let nim_input = read_line(reader).unwrap_or_default();
    if !nim_input.is_empty() {
        updated_profile.student.nim = nim_input;
    }

    let _ = write!(writer, "{}: [{}] ", tr(lang, MessageKey::ClassPrompt), updated_profile.student.class);
    let _ = writer.flush();
    let class_input = read_line(reader).unwrap_or_default();
    if !class_input.is_empty() {
        updated_profile.student.class = class_input;
    }

    // Save profile
    updated_profile.language = lang.code().to_string();

    if let Err(e) = updated_profile.save_to_file(&profile_path) {
        let _ = writeln!(writer, "Error saving profile: {e}");
        return 1;
    }

    let _ = writeln!(writer, "\n>> {}", tr(lang, MessageKey::SuccessInit));
    let _ = writeln!(writer, "   {} | {} | {}", updated_profile.student.name, updated_profile.student.nim, updated_profile.student.class);

    let _ = write!(writer, "\n{} ", tr(lang, MessageKey::ScaffoldPrompt));
    let _ = writer.flush();
    let scaffold_choice = read_line(reader).unwrap_or_default().to_ascii_lowercase();

    if scaffold_choice == "y" || scaffold_choice == "yes" {
        let _ = handle_new_project(reader, writer, workspace_root, &updated_profile);
    }

    let _ = writeln!(writer, "\n{}", tr(lang, MessageKey::NextSteps));
    0
}
