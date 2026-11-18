# Product Requirements Document (PRD): Rust Lab Report Toolchain

## Document Metadata
- **Status:** Active
- **Standard:** ISO/IEC/IEEE 29148:2018
- **Governance:** Axiomatic Systems Engineering Protocol (ASEP)
- **Traceability Baseline:** `docs/PRD.md`

---

## 1. System Overview
The Rust Lab Report CLI (`lab-report`) provides a single-binary automation toolchain for compiling, validating, packing, and managing academic laboratory reports at Institut Teknologi Sumatera (ITERA). It replaces the legacy Python tooling (`tools/lab_report`) with a native executable that needs no separate runtime interpreter.

---

## 2. Functional Requirements

### 2.1 Configuration & Profile Resolution

<!-- [@req:FR-CLI-01:MUST] [@criticality:low] -->
#### Requirement FR-CLI-01: Student Profile Configuration Loading
The Configuration Engine must load, parse, and validate student metadata from `profile.toml` located at the workspace root directory.

**Mathematical Invariant:**
$$\text{len}(\text{student.name}) > 0 \land \text{len}(\text{student.nim}) \ge 9$$

**Happy Path Execution:**
1. The Configuration Engine attempts to read `profile.toml` from the workspace root.
2. If `profile.toml` exists, the engine parses its contents as TOML.
3. The engine populates a strongly typed `Profile` structure containing student name, student NIM, student class, and file naming pattern.
4. If `profile.toml` is absent, the engine falls back to default student placeholders (`Jane Smith`, `xxxxxxxxx`, `RX`).

**Failure Modes:**
- Corrupted TOML: Return `ConfigError::InvalidToml` with line and column diagnostics.
- Missing Required Key: Return `ConfigError::MissingField` naming the missing identifier.

---

### 2.2 Course Discovery & Validation

<!-- [@req:FR-CLI-02:MUST] [@criticality:low] -->
#### Requirement FR-CLI-02: Course Discovery and Validation
The Course Manager must discover, index, and validate registered laboratory courses within the workspace root.

**Mathematical Invariant:**
$$\text{course.key} \in \{\text{"database"}, \text{"computer-networks"}, \text{"object-oriented-programming"}, \text{"pbo"}\}$$

**Happy Path Execution:**
1. The Course Manager inspects the workspace directory structure for course folders containing `main.typ` and `template/lib.typ`.
2. The Course Manager resolves alias keys (such as resolving `"pbo"` to `"object-oriented-programming"`).
3. The Course Manager verifies that the resolved course directory contains valid metadata files.

**Failure Modes:**
- Unknown Course Key: Return `CourseError::UnknownCourse` listing all available course identifiers.
- Missing Directory: Return `CourseError::DirectoryNotFound` specifying the unresolved file path.

---

### 2.3 Output Filename Formatting

<!-- [@req:FR-CLI-03:MUST] [@criticality:low] -->
#### Requirement FR-CLI-03: Submission Artifact Naming
The Artifact Formatter must generate compliant submission artifact filenames for PDF documents and ZIP archives based on the student profile and target module number.

**Mathematical Invariant:**
$$\text{artifact\_name} = \text{sanitize}(\text{name}) + \text{"\_"} + \text{nim} + \text{"\_Tugas"} + \text{module\_num} + \text{extension}$$

**Happy Path Execution:**
1. The Artifact Formatter receives student name, student NIM, target module number, and desired extension (`.pdf` or `.zip`).
2. The Artifact Formatter replaces spaces in the student name with underscores.
3. The Artifact Formatter interpolates values into the configured pattern `{name}_{nim}_Tugas{module}`.
4. The Artifact Formatter returns the formatted string (for example, `Jane_Smith_xxxxxxxx_Tugas1.pdf`).

**Failure Modes:**
- Empty Name or NIM: Return `FormatError::EmptyParameter` rejecting the operation.

---

### 2.4 Typst Compilation Runner

<!-- [@req:FR-CLI-04:MUST] [@criticality:medium] -->
#### Requirement FR-CLI-04: Typst Compilation Runner
The Compilation Runner must assemble and execute the `typst compile` or `typst watch` process for target course reports with appropriate `--font-path`, input `main.typ`, and calculated output PDF or multi-page SVG destination (`output_svg/page-{0p}.svg`).

**Mathematical Invariant:**
$$\text{compiler\_exit\_code} = 0 \implies \text{exists}(\text{output\_path})$$

**Happy Path Execution:**
1. The runner resolves course directory, `main.typ` path, and `fonts` path.
2. The runner invokes student data synchronization writing `data/student.typ` and `data/class.typ` matching course schema.
3. The runner builds CLI arguments `["typst", mode, "--font-path", fonts_dir, main_typ, output_path]`.
4. If format is SVG, output path automatically includes page number template `{0p}`.
5. The runner invokes the process and monitors exit status.

**Failure Modes:**
- Typst Not Installed: Return `CompileError::TypstNotFound` with install instructions.
- Typst Compilation Failure: Return `CompileError::CompilationFailed(code)` preserving compiler diagnostics.

---

### 2.5 Submission ZIP Packing

<!-- [@req:FR-CLI-05:MUST] [@criticality:medium] -->
#### Requirement FR-CLI-05: Submission Package Generation
The Packaging Engine must assemble a compliant compressed `.zip` archive containing the compiled PDF report, and for OOP courses, include accompanying source code (`*.java`) at archive root.

**Mathematical Invariant:**
$$\text{zip\_entries} \supseteq \{\text{pdf\_name}\} \land (\text{course} = \text{OOP} \implies \text{zip\_entries} \supseteq \text{glob}(\text{"src/*.java"}))$$

**Happy Path Execution:**
1. The packaging engine locates the compiled report PDF or triggers compilation if absent.
2. The engine creates a deflate-compressed ZIP archive named `{stem}.zip`.
3. The engine writes the PDF file to the ZIP archive.
4. If the course is Object-Oriented Programming and `src/` directory exists, every `*.java` file is written to the archive root.

**Failure Modes:**
- Missing PDF Artifact: Return `PackError::MissingPdf` if compilation is skipped and PDF does not exist.
- Archive I/O Error: Return `PackError::ZipIo` when filesystem writes fail.

---

### 2.6 Typography and Quality Linter

<!-- [@req:FR-CLI-06:MUST] [@criticality:medium] -->
#### Requirement FR-CLI-06: Typography and Quality Linter
The Linter Engine must inspect source `.typ` files for typography invariant violations and template placeholder anomalies, supporting deterministic automated in-place fixes.

**Mathematical Invariant:**
$$\text{lint\_issues} = \emptyset \iff \forall l \in \text{lines}: (\text{has\_placeholder}(l) = \text{false} \land \text{has\_bold\_colon}(l) = \text{false})$$

**Happy Path Execution:**
1. The linter discovers all `.typ` files in the course directory or target file path.
2. The linter scans line-by-line for placeholder phrases (`QUAL-01-PLACEHOLDER`).
3. The linter scans line-by-line for colons inside or after bold title headers (`TYPO-01-BOLD-COLON`).
4. If `--fix` is active, the linter strips colons from bold titles in-place without altering other tokens.
5. The linter returns a list of detected issues with file path, line number, rule identifier, and severity.

**Failure Modes:**
- Unreadable File: Return `LintError::Io` specifying file path.

---

### 2.7 Empirical Hardware and Environment Inspector

<!-- [@req:FR-CLI-07:MUST] [@criticality:low] -->
#### Requirement FR-CLI-07: Hardware and Environment Inspector
The Hardware Inspector must probe host empirical specifications (CPU, RAM, OS kernel, primary storage, compiler versions) and render compliant Typst data markup into `tools-and-materials.typ`.

**Mathematical Invariant:**
$$\text{len}(\text{spec.cpu}) > 0 \land \text{len}(\text{spec.ram}) > 0 \land \text{len}(\text{spec.os}) > 0$$

**Happy Path Execution:**
1. The inspector queries `/proc/cpuinfo` or `lscpu` for CPU model name.
2. The inspector queries `/proc/meminfo` or `free -h` for total physical memory.
3. The inspector reads `/etc/os-release` and kernel release via system platform interfaces.
4. The inspector invokes `typst --version` and simulator probe commands.
5. The inspector renders the `#let tools-data = (...)` Typst block into `content/tools-and-materials.typ`.

**Failure Modes:**
- Hardware Probe Unavailable: Return fallback string (`"Unknown CPU"`, `"Unknown RAM"`) without panicking.

---

### 2.8 PlantUML Vector Diagram Renderer

<!-- [@req:FR-CLI-08:MUST] [@criticality:medium] -->
#### Requirement FR-CLI-08: PlantUML Vector Diagram Renderer
The Diagram Engine must discover, compile, and validate PlantUML (`.puml`) source diagrams into pure vector SVG assets with a documented skinparam palette.

**Mathematical Invariant:**
$$\text{puml\_source} \implies \text{exists}(\text{output\_svg}) \land \text{is\_valid\_svg}(\text{output\_svg})$$

**Happy Path Execution:**
1. The engine locates `plantuml` or `spec-engine` in system path or standard user bins.
2. The engine compiles target `.puml` files into `.svg` format with `-tsvg`.
3. The engine verifies the generated SVG asset exists at the expected destination path.

**Failure Modes:**
- PlantUML Binary Missing: Return `DiagramError::BinaryNotFound`.
- Syntax Error: Return `DiagramError::CompilationFailed` preserving tool diagnostic output.

---

### 2.9 Starter Project Scaffolding Engine

<!-- [@req:FR-CLI-09:MUST] [@criticality:low] -->
#### Requirement FR-CLI-09: Starter Project Scaffolding Engine
The Project Scaffolder must generate a compliant lab report project directory containing standardized entry points (`main.typ`), templates, and content modules from existing course archetypes.

**Mathematical Invariant:**
$$\text{scaffolded\_dir} \supseteq \{\text{"main.typ"}, \text{"template"}, \text{"content"}, \text{"data"}\}$$

**Happy Path Execution:**
1. The scaffolder resolves archetype course source directory.
2. The scaffolder recursively copies template components, default content, and data configurations.
3. The scaffolder initializes student metadata conforming to current profile.

**Failure Modes:**
- Source Template Missing: Return `ScaffoldError::TemplateNotFound`.
- Target Destination Conflict: Return `ScaffoldError::DestinationExists` if target folder already exists and overwrite is false.

---

### 2.10 Declarative Geometry & Font Verifier

<!-- [@req:FR-CLI-10:MUST] [@criticality:high] -->
#### Requirement FR-CLI-10: Declarative Geometry & Font Verifier
The Verification Engine must compile the report to SVG pages and verify canvas viewBox dimensions, required typography fonts, and geometric coordinates of anchor elements against zero-tolerance course contracts.

**Mathematical Invariant:**
$$\forall p \in \text{pages}: |\text{vb}_w - \text{contract}_w| \le 1.0 \land |\text{vb}_h - \text{contract}_h| \le 1.0$$

**Happy Path Execution:**
1. The verifier checks that every font in `required_fonts` exists in the course `fonts/` directory.
2. The verifier compiles `main.typ` into page-by-page SVGs using `typst compile`.
3. The verifier parses each SVG page viewBox and asserts conformance to ISO B5 or ISO A4 bounds.
4. The verifier locates Page 1 anchor elements (ITERA logo, banners, boxes) and verifies coordinate delta $\le \text{tolerance}$.

**Failure Modes:**
- Missing Fonts: Return `VerifyError::MissingFonts` listing unavailable files.
- Geometric Deviation: Return `VerifyError::GeometryMismatch` detailing coordinate deltas.

---

### 2.11 Model Context Protocol (MCP) JSON-RPC Server

<!-- [@req:FR-CLI-11:MUST] [@criticality:medium] -->
#### Requirement FR-CLI-11: Standard I/O Model Context Protocol Server
The CLI toolchain must provide a headless Model Context Protocol (MCP) JSON-RPC 2.0 server over stdin and stdout to expose verification, linting, compilation, packing, diagram rendering, and inspection tools to AI agents.

**Mathematical Invariant:**
$$\text{valid\_jsonrpc}(req) \implies \text{valid\_jsonrpc}(res) \land res.id = req.id$$

**Happy Path Execution:**
1. The server reads newline-delimited JSON-RPC requests from standard input.
2. The server handles `initialize`, responding with MCP protocol version, capabilities, and server metadata.
3. The server handles `tools/list`, returning catalog descriptors and JSON schemas for all available commands.
4. The server handles `tools/call`, executing the specified tool with arguments and responding with formatted text output.
5. The server handles `notifications/initialized` without emitting an erroneous response.

**Failure Modes:**
- Invalid JSON: Return JSON-RPC error code `-32700` (Parse error).
- Unknown Method: Return JSON-RPC error code `-32601` (Method not found).

---

### 2.12 Interactive Terminal User Interface (TUI)

<!-- [@req:FR-CLI-12:MUST] [@criticality:low] -->
#### Requirement FR-CLI-12: Interactive Terminal User Interface
The CLI toolchain must provide an interactive terminal menu when launched without subcommands or via `lab-report tui`, allowing students to discover courses, trigger compilations (PDF/SVG), watch for edits, scaffold starter projects, package submissions, and run tests interactively.

**Mathematical Invariant:**
$$\forall opt \in \text{valid\_choices}: \text{dispatch}(opt) \in \{\text{Ok}(\text{ExitCode}), \text{Exit}\}$$

**Happy Path Execution:**
1. The TUI renders a banner summarizing student metadata from `profile.toml`.
2. The TUI lists available courses and common commands (`New Project`, `Profile`, `Exit`).
3. If a course is selected, the TUI queries the module number and provides sub-actions (`Watch`, `Build PDF`, `Build SVG`, `Pack`, `Test`).
4. Upon action completion, control returns cleanly to the menu loop or exits when 0 is chosen.

**Failure Modes:**
- End of File / Broken Pipe on stdin: Gracefully exit with status code 0 without panic.

---

### 2.13 Context-Aware Course Auto-Detection

<!-- [@req:FR-CLI-13:MUST] [@criticality:low] -->
#### Requirement FR-CLI-13: Context-Aware Course Auto-Detection
The CLI toolchain must automatically detect the target academic course from current working directory path or localized workspace markers when course argument is omitted or set to `auto`.

**Mathematical Invariant:**
$$\text{cwd\_is\_course}(dir) \implies \text{detect}(dir) = \text{Some}(course)$$

**Happy Path Execution:**
1. User invokes command without course argument while inside a course folder (e.g. `cd computer-networks && lab-report build`).
2. Detection engine checks current directory name and parent paths for recognized course identifiers (`database`, `computer-networks`, `object-oriented-programming`).
3. If detected, engine sets resolved course target automatically without error.

**Failure Modes:**
- Ambiguous Root: If at repository root and no course argument given, return error prompting user to specify course.

---

### 2.14 Command-Line Profile Mutator

<!-- [@req:FR-CLI-14:MUST] [@criticality:low] -->
#### Requirement FR-CLI-14: Command-Line Profile Mutator
The CLI toolchain must allow direct modification of student profile metadata via command-line arguments without requiring text editor or interactive menu.

**Mathematical Invariant:**
$$\text{update}(\text{name}, \text{nim}, \text{class}) \implies \text{read}(\text{profile.toml}) = (\text{name}, \text{nim}, \text{class})$$

**Happy Path Execution:**
1. User executes `lab-report profile --name "Name" --nim "NIM" --class "Class"`.
2. Mutation engine updates non-empty fields in `profile.toml` atomically.
3. Mutation engine triggers `sync_student_data` across registered courses.

**Failure Modes:**
- Write Permission Error: Return `ConfigError::Io` with system diagnostic.

---

### 2.15 Safe Non-Destructive Lint Preview & Dry-Run

<!-- [@req:FR-CLI-15:MUST] [@criticality:low] -->
#### Requirement FR-CLI-15: Safe Non-Destructive Lint Preview & Dry-Run
The Typography Linter must support a `--dry-run` inspection flag alongside `--fix` that reports prospective in-place modifications without mutating disk contents.

**Mathematical Invariant:**
$$\text{dry\_run} = \text{true} \implies \text{mtime}(files)_{after} = \text{mtime}(files)_{before}$$

**Happy Path Execution:**
1. User invokes `lab-report lint <course> --fix --dry-run`.
2. Linter scans files, matches violation patterns, and computes planned text replacements.
3. Linter displays diff or preview of changes without writing back to files.

**Failure Modes:**
- File Read Error: Return `LintError::Io` with path context.

---

### 2.16 Actionable Remediation & Diagnostic Formatter

<!-- [@req:FR-CLI-16:MUST] [@criticality:low] -->
#### Requirement FR-CLI-16: Actionable Remediation & Diagnostic Formatter
The Execution Engine must format runtime failures into actionable remediation messages with concrete shell commands for dependency installation.

**Mathematical Invariant:**
$$\text{err} = \text{TypstNotFound} \implies \text{contains}(\text{msg}, \text{"cargo install --locked typst-cli"})$$

**Happy Path Execution:**
1. User executes command requiring external binary when binary is absent from `PATH`.
2. Formatter catches process spawn failure and displays structured remediation box with installation options for Linux, macOS, and Windows.

**Failure Modes:**
- None: Formatter always outputs actionable plaintext.

---

### 2.17 Autonomous Agent Context Generator

<!-- [@req:FR-CLI-17:MUST] [@criticality:low] -->
#### Requirement FR-CLI-17: Autonomous Agent Context Generator
The Project Scaffolder must generate a localized `.agent-context.md` inside newly scaffolded course projects detailing universal skill requirements, course contracts, required fonts, and CLI workflows.

**Mathematical Invariant:**
$$\text{scaffolded\_dir} \implies \exists (\text{scaffolded\_dir} / \text{".agent-context.md"})$$

**Happy Path Execution:**
1. User invokes `lab-report new <course> [dest]`.
2. Scaffolder copies template assets and personalizes student data.
3. Scaffolder generates `.agent-context.md` embedding course display name, universal skill reference (`itera-lab-report-standard`), required fonts, and commands.

**Failure Modes:**
- Write Error: Return `GeneratorError::Io` if context file creation fails.

---

### 2.18 Binary Distribution & One-Liner Installers

<!-- [@req:FR-CLI-18:MUST] [@criticality:low] -->
#### Requirement FR-CLI-18: Binary Distribution & One-Liner Installers
The toolchain must provide automated Woodpecker CI pipeline specifications and portable shell/PowerShell installer scripts enabling zero-Rust installation directly from Codeberg Releases.

**Mathematical Invariant:**
$$\forall os \in \{\text{linux}, \text{windows}, \text{macos}\}: \exists \text{artifact}(os, ver) \land \exists \text{installer}(os)$$

**Happy Path Execution:**
1. A release tag `v*` triggers Woodpecker CI pipeline `.woodpecker.yml`.
2. Pipeline compiles release binaries across targets and bundles archives with SHA-256 checksums.
3. Users install on Linux/macOS via `curl -fsSL ... | sh` or Windows via `irm ... | iex`.

**Failure Modes:**
- Download Failure: Installer script verifies HTTP response code and aborts with diagnostic message.
- Checksum Mismatch: Installer verifies SHA-256 digest before moving executable to system PATH.

---

### 2.19 Bilingual Localization Engine & First-Run Wizard

<!-- [@req:FR-CLI-19:MUST] [@criticality:low] -->
#### Requirement FR-CLI-19: Bilingual Localization Engine & First-Run Wizard
The CLI toolchain must provide a bilingual localization engine (English default, Indonesian alternative) and an interactive first-run wizard `lab-report init` that prompts for language selection, student metadata, and course onboarding.

**Mathematical Invariant:**
$$\forall lang \in \{\text{En}, \text{Id}\}: \forall msg\_key: \text{translate}(lang, msg\_key) \neq \emptyset$$

**Happy Path Execution:**
1. User invokes `lab-report init`.
2. Wizard queries preferred language (English default `[1]`, Indonesian `[2]`).
3. Wizard saves language preference in configuration and queries student name, NIM, and class.
4. Wizard presents available course templates and scaffolds requested project.
5. All subsequent CLI commands render diagnostics, help messages, and confirmations in the selected language.

**Failure Modes:**
- Non-Interactive / Pipe: When stdin is not a TTY and flags are omitted, wizard applies default values safely without crashing.








