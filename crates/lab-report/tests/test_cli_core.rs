use std::path::PathBuf;

// @test:FR-CLI-01
#[test]
fn test_profile_loading_and_default_fallback() {
    use lab_report::config::Profile;

    // Default fallback when path does not exist
    let nonexistent = PathBuf::from("nonexistent_profile.toml");
    let profile = Profile::load_or_default(&nonexistent);
    assert_eq!(profile.student.name, "Timothy Ronald");
    assert_eq!(profile.student.nim, "124140201");
    assert_eq!(profile.student.class, "RA");
    assert_eq!(profile.naming.pattern, "{name}_{nim}_Tugas{module}");

    // Loading valid TOML content
    let valid_toml = r#"
    [student]
    name = "Dzulkifli Anwar"
    nim = "125140075"
    class = "RB"

    [naming]
    pattern = "{name}_{nim}_Modul{module}"
    "#;
    let loaded = Profile::from_toml_str(valid_toml).expect("Must parse valid toml");
    assert_eq!(loaded.student.name, "Dzulkifli Anwar");
    assert_eq!(loaded.student.nim, "125140075");
    assert_eq!(loaded.student.class, "RB");
    assert_eq!(loaded.naming.pattern, "{name}_{nim}_Modul{module}");
}

// @test:FR-CLI-02
#[test]
fn test_course_discovery_and_aliasing() {
    use lab_report::courses::{CourseKind, resolve_course_key};

    assert_eq!(resolve_course_key("database").unwrap(), CourseKind::Database);
    assert_eq!(resolve_course_key("basis-data").unwrap(), CourseKind::Database);
    assert_eq!(resolve_course_key("computer-networks").unwrap(), CourseKind::ComputerNetworks);
    assert_eq!(resolve_course_key("jarkom").unwrap(), CourseKind::ComputerNetworks);
    assert_eq!(resolve_course_key("object-oriented-programming").unwrap(), CourseKind::ObjectOrientedProgramming);
    assert_eq!(resolve_course_key("pbo").unwrap(), CourseKind::ObjectOrientedProgramming);

    // Invalid course returns error
    assert!(resolve_course_key("nonexistent-course").is_err());
}

// @test:FR-CLI-03
#[test]
fn test_artifact_naming_resolution() {
    use lab_report::naming::format_artifact_filename;

    let filename = format_artifact_filename(
        "{name}_{nim}_Tugas{module}",
        "Timothy Ronald",
        "124140201",
        "1",
        "pdf",
    ).expect("Naming must succeed");

    assert_eq!(filename, "Timothy_Ronald_124140201_Tugas1.pdf");

    let zip_filename = format_artifact_filename(
        "{name}_{nim}_Tugas{module}",
        "Dzulkifli Anwar",
        "125140075",
        "5",
        "zip",
    ).expect("Naming must succeed");

    assert_eq!(zip_filename, "Dzulkifli_Anwar_125140075_Tugas5.zip");

    // Empty fields return error
    assert!(format_artifact_filename("{name}_{nim}_Tugas{module}", "", "124140201", "1", "pdf").is_err());
    assert!(format_artifact_filename("{name}_{nim}_Tugas{module}", "Timothy Ronald", "", "1", "pdf").is_err());
}

// @test:FR-CLI-04
#[test]
fn test_typst_command_builder_and_student_sync() {
    use lab_report::config::Profile;
    use lab_report::courses::CourseKind;
    use lab_report::runner::{build_compile_command, sync_student_data};
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    // 1. Check build_compile_command construction
    let out_pdf = root.join("report.pdf");
    let cmd = build_compile_command(CourseKind::ComputerNetworks, root, &out_pdf, false);
    assert_eq!(cmd[0], "typst");
    assert_eq!(cmd[1], "compile");
    assert!(cmd.iter().any(|arg| arg == "--font-path"));
    assert!(cmd.iter().any(|arg| arg.ends_with("computer-networks/fonts") || arg.ends_with("fonts")));
    assert!(cmd.iter().any(|arg| arg.ends_with("main.typ")));

    let watch_cmd = build_compile_command(CourseKind::Database, root, &out_pdf, true);
    assert_eq!(watch_cmd[1], "watch");

    // 2. Check sync_student_data
    let profile = Profile::default();
    let pbo_data = root.join("object-oriented-programming/data");
    fs::create_dir_all(&pbo_data).expect("Create pbo data dir");
    let jarkom_data = root.join("computer-networks/data");
    fs::create_dir_all(&jarkom_data).expect("Create jarkom data dir");

    let synced = sync_student_data(root, &profile).expect("Sync must succeed");
    assert_eq!(synced.len(), 3);

    let pbo_file = fs::read_to_string(pbo_data.join("student.typ")).unwrap();
    assert!(pbo_file.contains("nama: \"Timothy Ronald\""));
    assert!(pbo_file.contains("nim: \"124140201\""));

    let jarkom_file = fs::read_to_string(jarkom_data.join("student.typ")).unwrap();
    assert!(jarkom_file.contains("name: \"Timothy Ronald\""));
    assert!(jarkom_file.contains("nim: \"124140201\""));
    assert!(!jarkom_file.contains("student_id:"));
    assert!(!jarkom_file.contains("id:"));

    let jarkom_class = fs::read_to_string(jarkom_data.join("class.typ")).unwrap();
    assert!(jarkom_class.contains("#let class-data = \"RA\""));

    // 3. Check resolve_output_path for SVG vs PDF
    use lab_report::runner::resolve_output_path;
    let svg_path = resolve_output_path(CourseKind::ComputerNetworks, root, &profile, "1", None, "svg").unwrap();
    assert!(svg_path.to_string_lossy().contains("output_svg/page-{0p}.svg"));

    let pdf_path = resolve_output_path(CourseKind::ComputerNetworks, root, &profile, "1", None, "pdf").unwrap();
    assert!(pdf_path.to_string_lossy().contains("dist/"));
    assert!(pdf_path.to_string_lossy().contains("Timothy_Ronald_124140201_Tugas1.pdf"));
}

// @test:FR-CLI-05
#[test]
fn test_submission_packaging_with_sources() {
    use lab_report::config::Profile;
    use lab_report::courses::CourseKind;
    use lab_report::packer::pack_submission;
    use std::fs::{self, File};
    use tempfile::tempdir;
    use zip::ZipArchive;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    let pbo_dir = root.join("object-oriented-programming");
    let src_dir = pbo_dir.join("src");
    fs::create_dir_all(&src_dir).expect("Create pbo src dir");
    fs::write(pbo_dir.join("main.typ"), "// dummy entrypoint").unwrap();

    // Dummy java files
    fs::write(src_dir.join("Percobaan1.java"), "class Percobaan1 {}").unwrap();
    fs::write(src_dir.join("Percobaan2.java"), "class Percobaan2 {}").unwrap();

    // Dummy compiled PDF
    let pdf_name = "Timothy_Ronald_124140201_Tugas1.pdf";
    let dist_dir = pbo_dir.join("dist");
    fs::create_dir_all(&dist_dir).expect("Create pbo dist dir");
    fs::write(dist_dir.join(pdf_name), b"%PDF-1.4 dummy content").unwrap();

    let profile = Profile::default();
    let zip_path = pack_submission(CourseKind::ObjectOrientedProgramming, root, &profile, "1", None, true)
        .expect("Pack submission must succeed");

    assert!(zip_path.exists());
    assert_eq!(zip_path.file_name().unwrap(), "Timothy_Ronald_124140201_Tugas1.zip");

    let file = File::open(&zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let mut file_names = Vec::new();
    for i in 0..archive.len() {
        let entry = archive.by_index(i).unwrap();
        file_names.push(entry.name().to_string());
    }

    assert!(file_names.iter().any(|i: &String| i == pdf_name));
    assert!(file_names.iter().any(|i: &String| i == "Percobaan1.java"));
    assert!(file_names.iter().any(|i: &String| i == "Percobaan2.java"));
}

// @test:FR-CLI-05
#[test]
fn test_submission_packaging_includes_src_for_database() {
    use lab_report::config::Profile;
    use lab_report::courses::CourseKind;
    use lab_report::packer::pack_submission;
    use std::fs::{self, File};
    use tempfile::tempdir;
    use zip::ZipArchive;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    let db_dir = root.join("database");
    let src_dir = db_dir.join("src");
    fs::create_dir_all(&src_dir).expect("Create database src dir");
    fs::write(db_dir.join("main.typ"), "// dummy entrypoint").unwrap();

    // Dummy source file
    fs::write(src_dir.join("script.sql"), "CREATE TABLE users (id INT);").unwrap();

    // Dummy compiled PDF in dist/
    let pdf_name = "Timothy_Ronald_124140201_Tugas1.pdf";
    let dist_dir = db_dir.join("dist");
    fs::create_dir_all(&dist_dir).expect("Create database dist dir");
    fs::write(dist_dir.join(pdf_name), b"%PDF-1.4 dummy content").unwrap();

    let profile = Profile::default();
    let zip_path = pack_submission(CourseKind::Database, root, &profile, "1", None, true)
        .expect("Pack submission must succeed");

    assert!(zip_path.exists());
    assert_eq!(zip_path.file_name().unwrap(), "Timothy_Ronald_124140201_Tugas1.zip");

    let file = File::open(&zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let mut file_names = Vec::new();
    for i in 0..archive.len() {
        let entry = archive.by_index(i).unwrap();
        file_names.push(entry.name().to_string());
    }

    assert!(file_names.iter().any(|i: &String| i == pdf_name));
    assert!(file_names.iter().any(|i: &String| i == "script.sql"));
}

// @test:FR-CLI-06
#[test]
fn test_linter_rules_and_autofix() {
    use lab_report::linter::{lint_file, Severity};
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().expect("Must create temp directory");
    let test_file = temp.path().join("test.typ");

    // 1. Detect prohibited colon inside & outside bold
    fs::write(
        &test_file,
        "+ *Langkah 1:* Konfigurasi IP\n+ *Langkah 2*: Lakukan ping\n",
    )
    .unwrap();

    let issues = lint_file(&test_file, false).expect("Linting must succeed");
    assert_eq!(issues.len(), 2);
    assert!(issues.iter().all(|i| i.rule_id == "TYPO-01-BOLD-COLON"));
    assert!(issues.iter().all(|i| i.severity == Severity::Error));

    // 2. Test auto-fix
    let fixed_issues = lint_file(&test_file, true).expect("Linting fix must succeed");
    assert_eq!(fixed_issues.len(), 2);

    let fixed_content = fs::read_to_string(&test_file).unwrap();
    assert!(!fixed_content.contains("*Langkah 1:*"));
    assert!(!fixed_content.contains("*Langkah 2*:"));
    assert!(fixed_content.contains("*Langkah 1*"));
    assert!(fixed_content.contains("*Langkah 2*"));

    // 3. Detect placeholder
    fs::write(
        &test_file,
        "Jelaskan Langkah-langkah yang dilakukan selama praktikum\n",
    )
    .unwrap();

    let placeholder_issues = lint_file(&test_file, false).expect("Linting placeholder must succeed");
    assert_eq!(placeholder_issues.len(), 1);
    assert_eq!(placeholder_issues[0].rule_id, "QUAL-01-PLACEHOLDER");
}

// @test:FR-CLI-07
#[test]
fn test_system_inspector_and_tools_generation() {
    use lab_report::courses::CourseKind;
    use lab_report::sys_inspect::{format_typst_tools_data, inspect_system, update_tools_file};
    use std::fs;
    use tempfile::tempdir;

    let info = inspect_system(CourseKind::ComputerNetworks);
    assert!(!info.cpu.is_empty());
    assert!(!info.ram.is_empty());
    assert!(!info.os.is_empty());
    assert!(!info.typst.is_empty());

    let rendered = format_typst_tools_data(&info, CourseKind::ComputerNetworks);
    assert!(rendered.contains("#let tools-data = ("));
    assert!(rendered.contains(&info.cpu));
    assert!(rendered.contains(&info.ram));

    let temp = tempdir().expect("Must create temp directory");
    let content_dir = temp.path().join("computer-networks/content");
    fs::create_dir_all(&content_dir).unwrap();

    let target = update_tools_file(temp.path(), &info, CourseKind::ComputerNetworks)
        .expect("Update tools file must succeed");
    assert!(target.exists());

    let file_content = fs::read_to_string(&target).unwrap();
    assert!(file_content.contains("#let tools-data = ("));
}

// @test:FR-CLI-07
#[test]
fn test_storage_label_format() {
    use lab_report::sys_inspect::format_storage_label;

    let label = format_storage_label("RESCUE05RM22AR120SDK 120GB", "111.8G", 120.0);
    assert_eq!(
        label,
        "120 GB SSD (RESCUE05RM22AR120SDK 120GB), 111.8G menurut lsblk, disk OS"
    );
    assert!(label.contains("disk OS"));
}

// @test:FR-CLI-08
#[test]
fn test_diagram_header_and_scaffolding() {
    use lab_report::diagrams::{get_puml_header, scaffold_diagram, DiagramType};
    use std::fs;
    use tempfile::tempdir;

    let header = get_puml_header(DiagramType::Network);
    assert!(header.contains("skinparam backgroundColor white"));
    assert!(header.contains("skinparam node"));

    let temp = tempdir().expect("Create temp dir");
    let target = temp.path().join("test_topology.puml");

    let created = scaffold_diagram(&target, DiagramType::Network, "Test Network Topology")
        .expect("Scaffold diagram must succeed");
    assert!(created.exists());

    let content = fs::read_to_string(&created).unwrap();
    assert!(content.contains("title Test Network Topology"));
    assert!(content.contains("skinparam backgroundColor white"));
}

// @test:FR-CLI-09
#[test]
fn test_project_generator_scaffolding() {
    use lab_report::config::Profile;
    use lab_report::courses::CourseKind;
    use lab_report::generator::scaffold_project;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().expect("Create temp dir");
    let root = temp.path();

    // Create a mock archetype course
    let archetype_dir = root.join("database");
    fs::create_dir_all(archetype_dir.join("content")).unwrap();
    fs::create_dir_all(archetype_dir.join("data")).unwrap();
    fs::write(archetype_dir.join("main.typ"), "// Main database entrypoint").unwrap();
    fs::write(archetype_dir.join("data/student.typ"), "// Student data").unwrap();
    fs::create_dir_all(archetype_dir.join("template")).unwrap();
    fs::write(
        archetype_dir.join("template/lib.typ"),
        "#import \"/templates/b5/lib.typ\"\n",
    )
    .unwrap();

    fs::create_dir_all(root.join("templates/b5")).unwrap();
    fs::write(root.join("templates/b5/lib.typ"), "// shared").unwrap();

    let target_dir = root.join("my-new-database");
    let profile = Profile::default();

    let scaffolded = scaffold_project(CourseKind::Database, root, &target_dir, &profile)
        .expect("Project scaffold must succeed");

    assert!(scaffolded.exists());
    assert!(scaffolded.join("main.typ").exists());
    assert!(scaffolded.join("content").is_dir());
    assert!(scaffolded.join("data/student.typ").exists());
    assert!(scaffolded.join("templates/b5/lib.typ").exists());
    let shim = fs::read_to_string(scaffolded.join("template/lib.typ")).unwrap();
    assert!(shim.contains("/templates/b5/"));

    let profile_file = fs::read_to_string(scaffolded.join("profile.toml")).unwrap();
    assert!(profile_file.contains("name = \"Timothy Ronald\""));
    assert!(profile_file.contains("nim = \"124140201\""));
    assert!(profile_file.contains("class = \"RA\""));
}

// @test:FR-CLI-10
#[test]
fn test_declarative_svg_verifier_and_contracts() {
    use lab_report::courses::CourseKind;
    use lab_report::verifier::{parse_svg_string, verify_contract, CourseContract};
    use tempfile::tempdir;

    // 1. Test SVG parser
    let sample_svg = r##"<svg viewBox="0 0 498.8976 708.6614" width="498.8976pt" height="708.6614pt" xmlns="http://www.w3.org/2000/svg">
        <path d="M 379.80 47.96 L 450.14 47.96 L 450.14 128.74 L 379.80 128.74 Z" fill="#808080"/>
        <path d="M 15.15 237.52 L 481.36 237.52 L 481.36 335.06 L 15.15 335.06 Z" fill="#808080"/>
        <image x="42.67" y="584.67" width="57.6" height="57.6" href="logo.png"/>
        <image x="122.67" y="616.00" width="114.3" height="50.0" href="fti.png"/>
        <image x="256.67" y="612.67" width="156.9" height="50.0" href="if.png"/>
    </svg>"##;

    let doc = parse_svg_string(sample_svg).expect("Parse SVG string");
    assert_eq!(doc.viewbox, (0.0, 0.0, 498.8976, 708.6614));
    assert_eq!(doc.elements.len(), 5);

    // 2. Test contract verification
    let temp = tempdir().expect("Create temp dir");
    let fonts_dir = temp.path().join("fonts");
    std::fs::create_dir_all(&fonts_dir).unwrap();

    let contract = CourseContract::for_course(CourseKind::Database);
    // Missing fonts should fail
    let (success, messages) = verify_contract(&contract, std::slice::from_ref(&doc), &fonts_dir);
    assert!(!success);
    assert!(messages.iter().any(|m| m.contains("Missing required fonts")));

    // When required fonts are present
    for font_name in contract.required_fonts {
        std::fs::write(fonts_dir.join(font_name), b"dummy font").unwrap();
    }

    let (success_with_fonts, messages2) = verify_contract(&contract, &[doc], &fonts_dir);
    assert!(success_with_fonts);
    assert!(messages2.iter().any(|m| m.contains("All 5 required fonts present")));
    assert!(messages2.iter().any(|m| m.contains("MATCH")));
}

// @test:FR-CLI-11
#[test]
fn test_mcp_server_protocol() {
    use lab_report::mcp::handle_mcp_request_str;
    use serde_json::Value;

    // 1. Initialize
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    let init_resp = handle_mcp_request_str(init_req, std::path::Path::new("."))
        .expect("Initialize must return response");
    let init_val: Value = serde_json::from_str(&init_resp).unwrap();
    assert_eq!(init_val["id"], 1);
    assert_eq!(init_val["result"]["serverInfo"]["name"], "lab-report-mcp");

    // 2. Tools list
    let list_req = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#;
    let list_resp = handle_mcp_request_str(list_req, std::path::Path::new("."))
        .expect("tools/list must return response");
    let list_val: Value = serde_json::from_str(&list_resp).unwrap();
    assert_eq!(list_val["id"], 2);
    let tools = list_val["result"]["tools"].as_array().unwrap();
    assert!(tools.iter().any(|t| t["name"] == "lab_report_lint"));
    assert!(tools.iter().any(|t| t["name"] == "laprak_lint"));

    // 3. Notification (no response)
    let notif_req = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let notif_resp = handle_mcp_request_str(notif_req, std::path::Path::new("."));
    assert!(notif_resp.is_none());

    // 4. Call profile tool
    let call_req = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"lab_report_profile","arguments":{}}}"#;
    let call_resp = handle_mcp_request_str(call_req, std::path::Path::new("."))
        .expect("tools/call must return response");
    let call_val: Value = serde_json::from_str(&call_resp).unwrap();
    assert_eq!(call_val["id"], 3);
    assert_eq!(call_val["result"]["content"][0]["type"], "text");
    assert!(call_val["result"]["content"][0]["text"].as_str().unwrap().contains("Timothy Ronald"));
}

// @test:FR-CLI-12
#[test]
fn test_interactive_tui_loop() {
    use lab_report::tui::run_tui_loop;
    use std::io::Cursor;
    use std::path::Path;

    // 1. Immediate exit
    let mut input = Cursor::new(b"0\n");
    let mut output = Vec::new();
    let res = run_tui_loop(&mut input, &mut output, Path::new("."));
    assert_eq!(res, 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Lab Report CLI & TUI"));
    assert!(output_str.contains("Timothy Ronald"));

    // 2. Select profile (5), then exit (0)
    let mut input2 = Cursor::new(b"5\n0\n");
    let mut output2 = Vec::new();
    let res2 = run_tui_loop(&mut input2, &mut output2, Path::new("."));
    assert_eq!(res2, 0);

    let output_str2 = String::from_utf8(output2).unwrap();
    assert!(output_str2.contains("124140201"));
}

// @test:FR-CLI-13
#[test]
fn test_course_auto_detection() {
    use lab_report::courses::{detect_course_from_path, CourseKind};
    use std::path::Path;

    assert_eq!(detect_course_from_path(Path::new("computer-networks")), Some(CourseKind::ComputerNetworks));
    assert_eq!(detect_course_from_path(Path::new("database/content")), Some(CourseKind::Database));
    assert_eq!(detect_course_from_path(Path::new("object-oriented-programming/src")), Some(CourseKind::ObjectOrientedProgramming));
    assert_eq!(detect_course_from_path(Path::new("some/random/dir")), None);
}

// @test:FR-CLI-14
#[test]
fn test_profile_cli_mutation() {
    use lab_report::config::{update_profile_fields, Profile};
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let profile_path = temp.path().join("profile.toml");

    let initial = Profile::default();
    initial.save_to_file(&profile_path).unwrap();

    let updated = update_profile_fields(
        &profile_path,
        Some("Jane Doe"),
        Some("123456789"),
        Some("RC"),
    ).unwrap();

    assert_eq!(updated.student.name, "Jane Doe");
    assert_eq!(updated.student.nim, "123456789");
    assert_eq!(updated.student.class, "RC");

    let reloaded = Profile::load_or_default(&profile_path);
    assert_eq!(reloaded.student.name, "Jane Doe");
    assert_eq!(reloaded.student.nim, "123456789");
    assert_eq!(reloaded.student.class, "RC");
}

// @test:FR-CLI-15
#[test]
fn test_lint_dry_run_simulation() {
    use lab_report::linter::lint_course_with_options;
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let file_path = temp.path().join("main.typ");
    let content = "= Header\n*Bad Colon:* Explanation\n";
    std::fs::write(&file_path, content).unwrap();

    // With fix = true, dry_run = true -> file content must NOT change
    let issues = lint_course_with_options(temp.path(), true, true).unwrap();
    assert!(!issues.is_empty());

    let unchanged = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(unchanged, content);
}

// @test:FR-CLI-16
#[test]
fn test_typst_not_found_remediation_message() {
    use lab_report::runner::{format_remediation_message, RunnerError};

    let msg = format_remediation_message(&RunnerError::TypstNotFound);
    assert!(msg.contains("typst"));
    assert!(msg.contains("cargo install --locked typst-cli"));
}

// @test:FR-CLI-17
#[test]
fn test_agent_context_generation_on_scaffold() {
    use lab_report::config::Profile;
    use lab_report::courses::CourseKind;
    use lab_report::generator::scaffold_project;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().expect("Create temp dir");
    let root = temp.path();

    let archetype_dir = root.join("database");
    fs::create_dir_all(archetype_dir.join("content")).unwrap();
    fs::create_dir_all(archetype_dir.join("data")).unwrap();
    fs::write(archetype_dir.join("main.typ"), "// Main database entrypoint").unwrap();
    fs::write(archetype_dir.join("data/student.typ"), "// Student data").unwrap();

    let target_dir = root.join("scaffolded-database");
    let profile = Profile::default();

    let scaffolded = scaffold_project(CourseKind::Database, root, &target_dir, &profile)
        .expect("Project scaffold must succeed");

    let context_file = scaffolded.join(".agent-context.md");
    assert!(context_file.is_file(), ".agent-context.md must be generated in scaffolded project");

    let content = fs::read_to_string(context_file).unwrap();
    assert!(content.contains("itera-lab-report-standard"));
    assert!(content.contains("Database Systems"));
    assert!(content.contains("lab-report build"));
}

// @test:FR-CLI-18
#[test]
fn test_installer_scripts_and_release_config_validity() {
    use std::fs;
    use std::path::Path;

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(manifest_dir);

    // Release CI is intentionally absent; this test covers the installers only.

    let install_sh = root.join("install.sh");
    assert!(install_sh.is_file(), "install.sh must exist at repo root");
    let sh_content = fs::read_to_string(&install_sh).unwrap();
    assert!(sh_content.contains("codeberg.org"));
    assert!(sh_content.contains("lab-report"));
    assert!(sh_content.contains("sha256"), "install.sh must verify sha256 checksum");

    let install_ps1 = root.join("install.ps1");
    assert!(install_ps1.is_file(), "install.ps1 must exist at repo root");
    let ps1_content = fs::read_to_string(&install_ps1).unwrap();
    assert!(ps1_content.contains("codeberg.org"));
    assert!(ps1_content.contains("lab-report"));
    assert!(ps1_content.contains("SHA256") || ps1_content.contains("sha256"), "install.ps1 must verify sha256 checksum");
}

// @test:FR-CLI-19
#[test]
fn test_bilingual_localization_and_wizard() {
    use lab_report::i18n::{tr, Language, MessageKey};
    use lab_report::runner::{format_remediation_message_bilingual, RunnerError};
    use lab_report::tui::run_init_wizard;
    use std::io::Cursor;
    use tempfile::tempdir;

    // 1. Translation dictionary check
    assert_eq!(Language::En.code(), "en");
    assert_eq!(Language::Id.code(), "id");

    let en_welcome = tr(Language::En, MessageKey::Welcome);
    let id_welcome = tr(Language::Id, MessageKey::Welcome);
    assert!(en_welcome.contains("Welcome"), "En welcome must contain Welcome");
    assert!(id_welcome.contains("Selamat datang"), "Id welcome must contain Selamat datang");

    let en_remediation = format_remediation_message_bilingual(&RunnerError::TypstNotFound, Language::En);
    assert!(en_remediation.contains("Typst executable was not found"));
    let id_remediation = format_remediation_message_bilingual(&RunnerError::TypstNotFound, Language::Id);
    assert!(id_remediation.contains("Aplikasi Typst tidak ditemukan"));

    // 2. Interactive Wizard simulation
    let temp = tempdir().unwrap();
    let root = temp.path();

    // Mock inputs:
    // 2 (Indonesian)
    // Budi Santoso (Name)
    // 124140001 (NIM)
    // RB (Class)
    // n (Do not scaffold now)
    let mut input = Cursor::new(b"2\nBudi Santoso\n124140001\nRB\nn\n");
    let mut output = Vec::new();

    let exit_code = run_init_wizard(&mut input, &mut output, root);
    assert_eq!(exit_code, 0);

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Selamat datang") || output_str.contains("Bahasa"));
    assert!(output_str.contains("Budi Santoso"));

    // Verify profile.toml was created with student data and language
    let profile_path = root.join("profile.toml");
    assert!(profile_path.is_file());
    let profile_content = std::fs::read_to_string(profile_path).unwrap();
    assert!(profile_content.contains("Budi Santoso"));
    assert!(profile_content.contains("124140001"));
    assert!(profile_content.contains("RB"));
    assert!(profile_content.contains("language = \"id\""));
}

// @test:FR-CLI-20
#[test]
fn test_workspace_root_discovery() {
    use lab_report::courses::find_workspace_root;
    use std::fs;
    use tempfile::tempdir;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    fs::write(root.join("profile.toml"), "name = \"Dzulkifli Anwar\"\n").unwrap();
    fs::create_dir_all(root.join("crates/lab-report")).unwrap();
    fs::write(
        root.join("crates/lab-report/Cargo.toml"),
        "[package]\nname = \"lab-report\"\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("computer-networks")).unwrap();

    assert_eq!(
        find_workspace_root(&root.join("computer-networks")),
        Some(root.to_path_buf())
    );
    assert_eq!(find_workspace_root(root), Some(root.to_path_buf()));

    let empty = tempdir().expect("Must create temp directory");
    assert_eq!(find_workspace_root(empty.path()), None);
}

// @test:FR-CLI-21
#[test]
fn test_lint_missing_course_directory_is_error() {
    use lab_report::linter::lint_course_with_options;
    use tempfile::tempdir;

    let temp = tempdir().unwrap();
    let result = lint_course_with_options(&temp.path().join("missing"), false, false);
    assert!(result.is_err());
}

// @test:FR-CLI-22
#[test]
fn test_course_metadata_required_fonts_exist() {
    use lab_report::courses::CourseKind;

    let db = CourseKind::Database.metadata();
    let cn = CourseKind::ComputerNetworks.metadata();
    assert!(db.required_fonts.contains(&"GARA.TTF"));
    assert!(cn.required_fonts.contains(&"GARA.TTF"));

    let oop = CourseKind::ObjectOrientedProgramming.metadata();
    assert!(oop.required_fonts.contains(&"times.ttf"));
    assert!(oop.special_notes.contains("A4"));
}

// @test:FR-CLI-13
#[test]
fn test_standalone_project_detection() {
    use lab_report::courses::{course_root, detect_course_from_metadata, find_workspace_root, CourseKind};
    use std::fs;
    use tempfile::tempdir;

    // Standalone project: profile.toml + main.typ at project root.
    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    fs::write(
        root.join("profile.toml"),
        "[student]\nname = \"A\"\nnim = \"125140075\"\nclass = \"RB\"\n",
    )
    .unwrap();
    fs::write(root.join("main.typ"), "// standalone").unwrap();
    fs::write(
        root.join(".agent-context.md"),
        "# Agent Context\n- **Course Key:** `database`\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("content")).unwrap();

    assert_eq!(detect_course_from_metadata(root), Some(CourseKind::Database));
    assert_eq!(course_root(root, CourseKind::Database), root);
    assert_eq!(
        find_workspace_root(&root.join("content")),
        Some(root.to_path_buf())
    );

    // Monorepo-like layout: course lives in its own folder.
    let mono_temp = tempdir().expect("Must create temp directory");
    let mono = mono_temp.path();
    fs::create_dir_all(mono.join("database")).unwrap();
    fs::write(mono.join("database/main.typ"), "// db entrypoint").unwrap();

    assert_eq!(
        course_root(mono, CourseKind::Database),
        mono.join("database")
    );
}


// @test:FR-CLI-04
#[test]
fn test_standalone_student_sync() {
    use lab_report::config::Profile;
    use lab_report::runner::sync_student_data;
    use std::fs;
    use tempfile::tempdir;

    // Standalone Database project: profile + main.typ + .agent-context.md at root.
    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    fs::write(
        root.join("profile.toml"),
        "[student]\nname = \"Uji Mandiri\"\nnim = \"125140075\"\nclass = \"RB\"\n\n[naming]\npattern = \"{name}_{nim}_Tugas{module}\"\n",
    )
    .unwrap();
    fs::write(root.join("main.typ"), "// standalone database entrypoint").unwrap();
    fs::write(
        root.join(".agent-context.md"),
        "# Agent Context\n- **Course Key:** `database`\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("data")).unwrap();

    let profile = Profile::load_or_default(&root.join("profile.toml"));
    let synced = sync_student_data(root, &profile).expect("Sync must succeed");
    assert!(!synced.is_empty());

    let student = fs::read_to_string(root.join("data/student.typ")).unwrap();
    assert!(student.contains("name: \"Uji Mandiri\""));
    assert!(student.contains("nim: \"125140075\""));
    assert!(!student.contains("nama:"));
    assert!(!student.contains("student_id:"));

    let class = fs::read_to_string(root.join("data/class.typ")).unwrap();
    assert!(class.contains("#let class-data = \"RB\""));

    // Standalone OOP project
    let oop_temp = tempdir().expect("Must create temp directory");
    let oop_root = oop_temp.path();

    fs::write(
        oop_root.join("profile.toml"),
        "[student]\nname = \"Uji Mandiri\"\nnim = \"125140075\"\nclass = \"RB\"\n",
    )
    .unwrap();
    fs::write(oop_root.join("main.typ"), "// standalone oop entrypoint").unwrap();
    fs::write(
        oop_root.join(".agent-context.md"),
        "# Agent Context\n- **Course Key:** `pbo`\n",
    )
    .unwrap();
    fs::create_dir_all(oop_root.join("data")).unwrap();

    let oop_profile = Profile::load_or_default(&oop_root.join("profile.toml"));
    sync_student_data(oop_root, &oop_profile).expect("Sync must succeed");

    let oop_student = fs::read_to_string(oop_root.join("data/student.typ")).unwrap();
    assert!(oop_student.contains("nama: \"Uji Mandiri\""));
    assert!(!oop_student.contains("name:"));
}

// @test:FR-CLI-08
#[test]
fn test_audit_detects_stale_gap_and_zip() {
    use lab_report::auditor::audit_project;
    use lab_report::courses::CourseKind;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::ZipWriter;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    let db = root.join("database");
    let dist = db.join("dist");
    fs::create_dir_all(&dist).expect("Create database/dist");
    fs::write(db.join("main.typ"), "// database entrypoint").unwrap();

    let gap = "# GAP_REGISTER\n\n| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |\n|----|------|------|----------|-------|--------|---------|-----------|---------|-----------|\n| GR-F6-01 | 6 | x | halusinasi-modul | step.typ:1 | bolong | AI | go | catatan | ref |\n";
    fs::write(db.join("GAP_REGISTER.md"), gap).unwrap();

    fs::write(dist.join("laporan.pdf"), b"new").unwrap();

    let zip_path = dist.join("laporan.zip");
    let file = fs::File::create(&zip_path).unwrap();
    let mut writer = ZipWriter::new(file);
    writer
        .start_file("laporan.pdf", zip::write::FileOptions::<()>::default())
        .unwrap();
    writer.write_all(b"old").unwrap();
    writer.finish().unwrap();

    let (success, messages) = audit_project(root, CourseKind::Database).expect("Audit must run");
    assert!(!success);
    assert!(
        messages.iter().any(|m| m.contains("stale") && m.contains("GR-F6-01")),
        "stale-gap wording expected"
    );
    assert!(
        messages.iter().any(|m| m.contains("zip mismatch")),
        "zip mismatch wording expected"
    );
}

// @test:FR-CLI-08
#[test]
fn test_audit_passes_clean_project() {
    use lab_report::auditor::audit_project;
    use lab_report::courses::CourseKind;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::ZipWriter;

    let temp = tempdir().expect("Must create temp directory");
    let root = temp.path();

    let db = root.join("database");
    let dist = db.join("dist");
    fs::create_dir_all(&dist).expect("Create database/dist");
    fs::write(db.join("main.typ"), "// database entrypoint").unwrap();

    let gap = "# GAP_REGISTER\n\n| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |\n|----|------|------|----------|-------|--------|---------|-----------|---------|-----------|\n| GR-F6-01 | 6 | substansi modul | halusinasi-modul | step.typ:1 | ditangani | AI | go | diperbaiki sesuai modul | MODUL 6 |\n| GR-F6-02 | 6 | variabel NIM | analisis-terlewat | MODULE_VARIABLES.md | ditangani | Manusia | no-go | menunggu klarifikasi dosen | MODUL 6 hal. 3 |\n";
    fs::write(db.join("GAP_REGISTER.md"), gap).unwrap();

    fs::write(dist.join("laporan.pdf"), b"same").unwrap();

    let zip_path = dist.join("laporan.zip");
    let file = fs::File::create(&zip_path).unwrap();
    let mut writer = ZipWriter::new(file);
    writer
        .start_file("laporan.pdf", zip::write::FileOptions::<()>::default())
        .unwrap();
    writer.write_all(b"same").unwrap();
    writer.finish().unwrap();

    let (success, messages) = audit_project(root, CourseKind::Database).expect("Audit must run");
    assert!(success);
    assert!(
        messages.iter().any(|m| m.contains("no-go")),
        "no-go row must surface as warning only"
    );
}








