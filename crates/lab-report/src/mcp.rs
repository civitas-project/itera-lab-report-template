// @impl:FR-CLI-11
//! Standard I/O Model Context Protocol (MCP) JSON-RPC 2.0 server.

use crate::config::Profile;
use crate::courses::resolve_course_key;
use crate::diagrams::render_puml;
use crate::linter::lint_course;
use crate::packer::pack_submission;
use crate::runner::compile_report;
use crate::sys_inspect::{inspect_system, update_tools_file};
use crate::verifier::verify_course;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;

const MCP_SERVER_NAME: &str = "lab-report-mcp";
const MCP_SERVER_VERSION: &str = "0.3.0";
const PROTOCOL_VERSION: &str = "2024-11-05";

fn get_tools_registry() -> Vec<Value> {
    let base_tools = vec![
        json!({
            "name": "lab_report_inspect_host",
            "description": "Empirically inspects host hardware (CPU, RAM, OS, Storage, Typst) and optionally writes formatted data to content/tools-and-materials.typ.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "course": {
                        "type": "string",
                        "description": "Target course (computer-networks, database, object-oriented-programming)",
                        "default": "computer-networks"
                    },
                    "write_file": {
                        "type": "boolean",
                        "description": "Write output to content/tools-and-materials.typ",
                        "default": false
                    }
                }
            }
        }),
        json!({
            "name": "lab_report_lint",
            "description": "Audits Typst files for typography invariants (no colons on bold titles, no template placeholders).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "course": {
                        "type": "string",
                        "description": "Target course (computer-networks, database, object-oriented-programming)",
                        "default": "computer-networks"
                    },
                    "fix": {
                        "type": "boolean",
                        "description": "Automatically correct detected violations",
                        "default": false
                    }
                }
            }
        }),
        json!({
            "name": "lab_report_build",
            "description": "Compiles a course lab report to PDF using Typst.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "course": {
                        "type": "string",
                        "description": "Target course (computer-networks, database, object-oriented-programming)"
                    },
                    "module": {
                        "type": "string",
                        "description": "Module number (e.g. '5')"
                    }
                },
                "required": ["course"]
            }
        }),
        json!({
            "name": "lab_report_verify",
            "description": "Executes declarative coordinate geometry verification and font audit.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "course": {
                        "type": "string",
                        "description": "Target course (computer-networks, database, object-oriented-programming)",
                        "default": "computer-networks"
                    }
                }
            }
        }),
        json!({
            "name": "lab_report_render_diagram",
            "description": "Compiles a PlantUML diagram (.puml) into a pure vector SVG asset with a documented palette.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "puml_path": {
                        "type": "string",
                        "description": "Path to the .puml diagram file"
                    },
                    "out_dir": {
                        "type": "string",
                        "description": "Output directory for the compiled .svg"
                    }
                },
                "required": ["puml_path"]
            }
        }),
        json!({
            "name": "lab_report_pack",
            "description": "Packages compiled PDF report and source code into submission ZIP archive.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "course": {
                        "type": "string",
                        "description": "Target course (computer-networks, database, object-oriented-programming)"
                    },
                    "module": {
                        "type": "string",
                        "description": "Module number (e.g. '5')",
                        "default": "1"
                    }
                },
                "required": ["course"]
            }
        }),
        json!({
            "name": "lab_report_profile",
            "description": "Inspects or updates student profile (NIM, Name).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {"type": "string", "description": "Student full name"},
                    "nim": {"type": "string", "description": "Student NIM"}
                }
            }
        }),
    ];

    let mut registry = base_tools.clone();
    for tool in base_tools {
        let mut alias = tool.clone();
        if let Some(name) = alias["name"].as_str() {
            let alias_name = name.replace("lab_report_", "laprak_");
            alias["name"] = json!(alias_name);
            registry.push(alias);
        }
    }
    registry
}

fn execute_tool(name: &str, arguments: &Value, workspace_root: &Path) -> Value {
    let profile_path = workspace_root.join("profile.toml");
    let profile = Profile::load_or_default(&profile_path);

    let canonical = name.replace("laprak_", "lab_report_");
    match canonical.as_str() {
        "lab_report_inspect_host" => {
            let course_str = arguments["course"].as_str().unwrap_or("computer-networks");
            let write_file = arguments["write_file"].as_bool().unwrap_or(false);
            let course = match resolve_course_key(course_str) {
                Ok(c) => c,
                Err(e) => return json!({"status": "error", "message": e.to_string()}),
            };

            let info = inspect_system(course);
            let mut written = false;
            if write_file {
                if let Ok(path) = update_tools_file(workspace_root, &info, course) {
                    written = path.is_file();
                }
            }

            json!({
                "status": "success",
                "written": written,
                "cpu": info.cpu,
                "ram": info.ram,
                "storage": info.storage,
                "os": info.os,
                "typst": info.typst,
                "simulator": info.simulator
            })
        }
        "lab_report_lint" => {
            let course_str = arguments["course"].as_str().unwrap_or("computer-networks");
            let fix = arguments["fix"].as_bool().unwrap_or(false);
            let course = match resolve_course_key(course_str) {
                Ok(c) => c,
                Err(e) => return json!({"status": "error", "message": e.to_string()}),
            };

            let course_dir = workspace_root.join(course.dir_name());
            match lint_course(&course_dir, fix) {
                Ok(issues) => {
                    let issue_vals: Vec<Value> = issues
                        .iter()
                        .map(|i| {
                            let rel = i.file_path.strip_prefix(workspace_root).unwrap_or(&i.file_path);
                            json!({
                                "file": rel.display().to_string(),
                                "line": i.line_num,
                                "rule": i.rule_id,
                                "message": i.message,
                                "severity": match i.severity {
                                    crate::linter::Severity::Error => "error",
                                    crate::linter::Severity::Warning => "warning",
                                }
                            })
                        })
                        .collect();

                    json!({
                        "status": "success",
                        "issues_count": issue_vals.len(),
                        "issues": issue_vals
                    })
                }
                Err(e) => json!({"status": "error", "message": e.to_string()}),
            }
        }
        "lab_report_build" => {
            let course_str = match arguments["course"].as_str() {
                Some(s) => s,
                None => return json!({"status": "error", "message": "Missing 'course' parameter."}),
            };
            let course = match resolve_course_key(course_str) {
                Ok(c) => c,
                Err(e) => return json!({"status": "error", "message": e.to_string()}),
            };

            let module_num = arguments["module"].as_str().unwrap_or("1");
            let format_str = arguments["format"].as_str().unwrap_or("pdf");
            match compile_report(course, workspace_root, &profile, module_num, None, format_str, false, &[]) {
                Ok(code) => json!({
                    "status": if code == 0 { "success" } else { "error" },
                    "exit_code": code,
                    "course": course.dir_name()
                }),
                Err(e) => json!({"status": "error", "message": e.to_string()}),
            }
        }
        "lab_report_verify" => {
            let course_str = arguments["course"].as_str().unwrap_or("computer-networks");
            let course = match resolve_course_key(course_str) {
                Ok(c) => c,
                Err(e) => return json!({"status": "error", "message": e.to_string()}),
            };

            match verify_course(workspace_root, course) {
                Ok((success, messages)) => json!({
                    "status": if success { "success" } else { "failure" },
                    "course": course.dir_name(),
                    "messages": messages
                }),
                Err(e) => json!({"status": "error", "message": e.to_string()}),
            }
        }
        "lab_report_render_diagram" => {
            let puml_str = match arguments["puml_path"].as_str() {
                Some(s) => s,
                None => return json!({"status": "error", "message": "Missing 'puml_path' parameter."}),
            };

            let mut puml_path = Path::new(puml_str).to_path_buf();
            if !puml_path.is_absolute() {
                puml_path = workspace_root.join(puml_path);
            }

            let out_dir = arguments["out_dir"].as_str().map(|s| {
                let mut d = Path::new(s).to_path_buf();
                if !d.is_absolute() {
                    d = workspace_root.join(d);
                }
                d
            });

            match render_puml(&puml_path, out_dir.as_deref()) {
                Ok(svg_path) => {
                    let rel = svg_path.strip_prefix(workspace_root).unwrap_or(&svg_path);
                    let size = std::fs::metadata(&svg_path).map(|m| m.len()).unwrap_or(0);
                    json!({
                        "status": "success",
                        "svg_path": rel.display().to_string(),
                        "size_bytes": size
                    })
                }
                Err(e) => json!({"status": "error", "message": e.to_string()}),
            }
        }
        "lab_report_pack" => {
            let course_str = match arguments["course"].as_str() {
                Some(s) => s,
                None => return json!({"status": "error", "message": "Missing 'course' parameter."}),
            };
            let course = match resolve_course_key(course_str) {
                Ok(c) => c,
                Err(e) => return json!({"status": "error", "message": e.to_string()}),
            };

            let module_num = arguments["module"].as_str().unwrap_or("1");
            match pack_submission(course, workspace_root, &profile, module_num, None, false) {
                Ok(zip_path) => {
                    let rel = zip_path.strip_prefix(workspace_root).unwrap_or(&zip_path);
                    let size = std::fs::metadata(&zip_path).map(|m| m.len()).unwrap_or(0);
                    json!({
                        "status": "success",
                        "zip_path": rel.display().to_string(),
                        "size_bytes": size
                    })
                }
                Err(e) => json!({"status": "error", "message": e.to_string()}),
            }
        }
        "lab_report_profile" => {
            json!({
                "status": "success",
                "student": {
                    "name": profile.student.name,
                    "nim": profile.student.nim
                },
                "naming": {
                    "pattern": profile.naming.pattern
                }
            })
        }
        other => json!({"status": "error", "message": format!("Unknown tool: {other}")}),
    }
}

/// Handles a single incoming JSON-RPC string message and returns response if required.
#[must_use]
pub fn handle_mcp_request_str(req_str: &str, workspace_root: &Path) -> Option<String> {
    let req: Value = serde_json::from_str(req_str).ok()?;
    let req_id = req.get("id").cloned();
    let method = req["method"].as_str().unwrap_or("");

    if method == "initialize" {
        let resp = json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": MCP_SERVER_NAME,
                    "version": MCP_SERVER_VERSION
                }
            }
        });
        return Some(resp.to_string());
    }

    if method == "tools/list" {
        let resp = json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "tools": get_tools_registry()
            }
        });
        return Some(resp.to_string());
    }

    if method == "tools/call" {
        let params = &req["params"];
        let tool_name = params["name"].as_str().unwrap_or("");
        let arguments = &params["arguments"];
        let result_obj = execute_tool(tool_name, arguments, workspace_root);
        let formatted = serde_json::to_string_pretty(&result_obj).unwrap_or_default();

        let resp = json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "content": [
                    {
                        "type": "text",
                        "text": formatted
                    }
                ]
            }
        });
        return Some(resp.to_string());
    }

    if method == "notifications/initialized" {
        return None;
    }

    // Method not found error
    let resp = json!({
        "jsonrpc": "2.0",
        "id": req_id,
        "error": {
            "code": -32601,
            "message": format!("Method not found: {method}")
        }
    });
    Some(resp.to_string())
}

/// Runs standard I/O JSON-RPC loop over standard input and standard output.
///
/// # Errors
/// Returns [`std::io::Error`] on unrecoverable stream error.
pub fn run_mcp_server(workspace_root: &Path) -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line_res in stdin.lock().lines() {
        let line = line_res?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(resp) = handle_mcp_request_str(trimmed, workspace_root) {
            writeln!(stdout, "{resp}")?;
            stdout.flush()?;
        }
    }

    Ok(())
}
