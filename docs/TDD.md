# Technical Design Document (TDD): Rust Lab Report Toolchain

## Document Metadata
- **Status:** Active
- **Standard:** Axiomatic Systems Engineering Protocol (ASEP)
- **Traceability Baseline:** `docs/TDD.md`
- **Architecture Decision:** `docs/adr/0001-migrate-python-lab-report-cli-to-rust.md`

---

## 1. Architectural Architecture Overview
The Rust `lab-report` binary is structured as a modular library crate (`lab_report`) with a lightweight binary frontend (`src/main.rs`). It eliminates dynamic runtime overhead by leveraging compile-time dispatch and static monomorphization.

```text
                  +-----------------------------+
                  |         CLI Frontend        |
                  |         (src/main.rs)       |
                  +--------------+--------------+
                                 |
                  +--------------v--------------+
                  |      Library Root API       |
                  |        (src/lib.rs)         |
                  +--------------+--------------+
                                 |
         +-----------------------+-----------------------+
         |                       |                       |
+--------v--------+     +--------v--------+     +--------v--------+
|     config      |     |     courses     |     |     runner      |
|  (FR-CLI-01)    |     |  (FR-CLI-02)    |     |  (FR-CLI-03)    |
+-----------------+     +-----------------+     +-----------------+
```

---

## 2. Module Specifications

### 2.1 Configuration Module (`src/config.rs`)
<!-- [@design:FR-CLI-01] -->
The `config` module encapsulates student metadata and workspace naming patterns.
- Data Structure: `Profile` containing `student: StudentConfig` and `naming: NamingConfig`.
- Parsing: Uses `serde` with `toml` deserializer.
- Default fallback: If `profile.toml` is unreadable, construct `Profile::default()`.

### 2.2 Course Management Module (`src/courses.rs`)
<!-- [@design:FR-CLI-02] -->
The `courses` module provides deterministic cataloging of supported academic courses.
- Course enum: `CourseKind` with variants `Database`, `ComputerNetworks`, `ObjectOrientedProgramming`.
- Path resolution: Resolves directory locations relative to workspace root.
- Aliasing: Normalizes string inputs (`"pbo"` -> `CourseKind::ObjectOrientedProgramming`).

### 2.3 Artifact Naming Module (`src/naming.rs`)
<!-- [@design:FR-CLI-03] -->
The `naming` module formats standardized submission filenames for PDF and ZIP artifacts.
- String interpolation: Replaces `{name}`, `{nim}`, and `{module}` in template pattern.
- Whitespace sanitization: Replaces whitespace in student name with underscores.

### 2.4 Typst Compilation Runner (`src/runner.rs`)
<!-- [@design:FR-CLI-04] -->
The `runner` module synchronizes student metadata and manages the `typst` child process lifecycle.
- Data synchronization: Writes `data/student.typ` and `data/class.typ` before build execution.
- Command builder: Configures `--font-path` using course-specific fonts directory.
- Execution modes: Supports both single compilation (`compile`) and continuous watcher (`watch`).
- Format selection: Supports PDF and multi-page SVG (`output_svg/page-{0p}.svg`).

### 2.5 Submission Packer Module (`src/packer.rs`)
<!-- [@design:FR-CLI-05] -->
The `packer` module bundles final submission deliverables into a ZIP archive.
- PDF packaging: Stores compiled output PDF at the root of the ZIP.
- OOP Source bundling: Traverses `src/*.java` and adds them directly to the archive root without intermediate folders.
- Compression: Employs Deflate algorithm via the native `zip` crate.

### 2.6 Typography Linter Module (`src/linter.rs`)
<!-- [@design:FR-CLI-06] -->
The `linter` module verifies non-negotiable typography invariants across `.typ` files.
- Placeholder pattern matcher: Evaluates regular expressions for forbidden Indonesian and English instructional text.
- Bold colon checker: Matches `*Header:*` or `*Header*: ` constructs and offers regex-based replacement.
- In-place mutation: When `fix = true`, rewrites matching files directly without changing line ending conventions.

### 2.7 Hardware Inspector Module (`src/sys_inspect.rs`)
<!-- [@design:FR-CLI-07] -->
The `sys_inspect` module performs hardware discovery and formats template data.
- Procfs extraction: Reads `/proc/cpuinfo`, `/proc/meminfo`, and `/etc/os-release` natively without subprocess spawning where possible.
- Fallback subcommands: Executes `lscpu`, `free`, `lsblk`, `uname`, and `typst --version` when procfs fields are absent.
- Typst code generator: Emits tabular definitions for `#let tools-data = (...)` matching course standards.

### 2.8 Diagram Renderer Module (`src/diagrams.rs`)
<!-- [@design:FR-CLI-08] -->
The `diagrams` module compiles PlantUML diagram sources into vector SVG format.
- Discovery: Finds `plantuml` or `spec-engine` wrapper binaries.
- Compilation: Invokes PlantUML with `-tsvg` and output directory specifications.
- Header injection: Embeds standardized vector skinparams for network, ERD, and class diagrams.

### 2.9 Project Scaffolder Module (`src/generator.rs`)
<!-- [@design:FR-CLI-09] -->
The `generator` module scaffolds new lab report projects.
- Directory cloning: Clones archetype directories (`main.typ`, `template/`, `content/`, `data/`) while omitting transient artifacts.
- Metadata substitution: Personalizes `student.typ` matching target course conventions.

### 2.10 Declarative Verifier Module (`src/verifier.rs`)
<!-- [@design:FR-CLI-10] -->
The `verifier` module executes zero-tolerance geometry and typography audits.
- SVG Parsing: Extracts viewBox bounds and element bounding boxes using `quick-xml`.
- Geometric Anchor Matching: Compares Page 1 elements against course contracts within delta tolerance $\le 0.5\text{ pt}$.
- Typography Font Auditing: Verifies presence of mandatory `.ttf` files in course fonts directory.

### 2.11 Model Context Protocol Server (`src/mcp.rs`)
<!-- [@design:FR-CLI-11] -->
The `mcp` module provides standard JSON-RPC 2.0 communication over standard input/output.
- Protocol handling: Handles `initialize`, `notifications/initialized`, `tools/list`, and `tools/call`.
- Schema definitions: Generates static JSON Schema specifications for every callable tool (`lab_report_*` and `laprak_*`).
- Dispatch loop: Streams responses as single-line JSON serialized structures terminated by `\n`.

### 2.12 Interactive Terminal User Interface Module (`src/tui.rs`)
<!-- [@design:FR-CLI-12] -->
The `tui` module provides an interactive menu loop using standard input/output streams.
- Stream Abstraction: Takes generic `BufRead` and `Write` interfaces for 100% deterministic unit testing without terminal allocation.
- Menu Options: Dispatches course workflows (`watch`, `build` PDF/SVG, `pack`, `test`), project scaffolding (`new`), and profile display.
- Graceful Termination: Handles EOF on stdin cleanly.

### 2.13 Course Auto-Detection Engine (`src/courses.rs`)
<!-- [@design:FR-CLI-13] -->
The `courses` module provides context detection from current working directories.
- Context Heuristics: Checks directory names (`database`, `computer-networks`, `object-oriented-programming`) and localized directory entries.
- Fallback logic: Returns `Option<CourseKind>` allowing optional CLI course parameters.

### 2.14 Profile Mutator Engine (`src/config.rs`)
<!-- [@design:FR-CLI-14] -->
The `config` module provides atomic mutation functions.
- Atomicity: Modifies student fields and serializes back to `profile.toml` using `toml::to_string_pretty`.
- Auto-sync: Triggers `sync_student_data` immediately upon profile mutation.

### 2.15 Safe Lint Preview Engine (`src/linter.rs`)
<!-- [@design:FR-CLI-15] -->
The `linter` module adds dry-run simulation mode.
- Simulation Flag: Distinguishes between `--fix` (disk mutation) and `--dry-run` (in-memory diff generation).
- Issue Reporting: Outputs planned changes with line numbers and preview replacements.

### 2.16 Actionable Remediation Formatter (`src/runner.rs`)
<!-- [@design:FR-CLI-16] -->
The `runner` module provides platform-aware diagnostic formatting.
- Error enrichment: Enriches `TypstNotFound` with OS-specific package manager instructions (`pacman`, `apt`, `brew`, `winget`, `cargo`).

### 2.17 Autonomous Agent Context Generator (`src/generator.rs`)
<!-- [@design:FR-CLI-17] -->
The `generator` module synthesizes localized context for AI agents.
- Context Generation: Produces `.agent-context.md` referencing `itera-lab-report-standard`, mandatory fonts, verification commands, and student metadata.

### 2.18 Binary Distribution & One-Liner Installers (`.woodpecker.yml`, `install.sh`)
<!-- [@design:FR-CLI-18] -->
The distribution system packages and provisions standalone binaries.
- Woodpecker Pipeline: Builds release binaries on tag pushes and generates SHA-256 checksum manifests.
- Portable Install Scripts: `install.sh` (POSIX sh) and `install.ps1` (PowerShell) detect host architecture and install to user bin directory.

### 2.19 Bilingual Localization Engine & First-Run Wizard (`src/i18n.rs`)
<!-- [@design:FR-CLI-19] -->
The localization module provides dual-language support (English / Indonesian) and interactive onboarding.
- Language Resolution: Evaluates `--lang` flag, `profile.toml` setting, `LANG`/`LC_ALL` environment variable, or falls back to English.
- Translation Catalogs: Compiles embedded message dictionaries for UI prompts, error diagnostics, and success reports.
- Interactive Wizard: Interactive setup querying language preference, student identity, and initial course scaffold.








