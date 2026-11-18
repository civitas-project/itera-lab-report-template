# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0-alpha.1] - 2026-10-31

### Added
- **Native Rust Toolchain (`crates/lab-report`)** \
  Inaugural clean alpha release of high-performance standalone binary (`lab-report`) with sub-10ms startup latency and zero runtime Python interpreter dependencies.
- **Axiomatic Systems Engineering Protocol (ASEP) Traceability** \
  Comprehensive 1:1 requirements verification (`@req` <-> `@impl` <-> `@test`) passing all axiomatic SDLC quality gates (`FR-CLI-01` through `FR-CLI-19`).
- **Native Multi-Page SVG Export Engine** \
  Added `-f, --format svg` and `--svg` flags to `build` and `watch` commands, compiling reports directly into vector SVG pages (`output_svg/page-{0p}.svg`).
- **Automated Class Metadata Synchronization** \
  Synchronizes student section/class (`class.typ`) across Database, Computer Networks, and OOP courses from `profile.toml`.
- **Pure Native Model Context Protocol (MCP) Server** \
  Native JSON-RPC 2.0 stdio server (`lab-report mcp`) handling autonomous agent tool execution with zero cold-start delay.
- **Interactive Terminal UI Engine (`FR-CLI-12`)** \
  Native zero-dependency terminal menu loop (`lab-report tui`) with stream abstraction for automated report builds, watches, packages, and tests.
- **Context-Aware Course Auto-Detection (`FR-CLI-13`)** \
  Automatically detects active course when command is executed from inside course directories (`database/`, `computer-networks/`, `object-oriented-programming/`).
- **Command-Line Profile Mutator (`FR-CLI-14`)** \
  Directly updates student metadata in `profile.toml` and triggers automatic template data sync via `lab-report profile --name "..." --nim "..." --class "..."`.
- **Safe Linter Dry-Run Mode (`FR-CLI-15`)** \
  Previews typography invariant corrections without modifying disk files via `lab-report lint --fix --dry-run`.
- **Actionable Remediation Diagnostics (`FR-CLI-16`)** \
  Formats process execution errors (e.g. missing Typst executable) with concrete OS-specific installation commands.
- **Autonomous Agent Context Generator (`FR-CLI-17`)** \
  Automatically generates localized `.agent-context.md` during project scaffolding (`lab-report new`) providing immediate guidance on universal skill requirements, course contracts, required fonts, and commands.
- **Binary Distribution & Codeberg Releases Pipeline (`FR-CLI-18`)** \
  Native Woodpecker CI pipeline (`.woodpecker.yml`) building Linux and Windows x86_64 binaries, with one-liner installer scripts (`install.sh`, `install.ps1`) verifying SHA-256 checksums without requiring a local Rust toolchain.
- **Bilingual Localization & First-Run Wizard (`FR-CLI-19`)** \
  Interactive first-run wizard (`lab-report init`) and dual-language localization engine (`i18n.rs`) supporting English (default) and Bahasa Indonesia with persistent language preferences.

### Removed
- **Deprecated Python Toolchain (`tools/lab_report`)** \
  Removed redundant legacy Python package, heavy wheel dependencies, and duplicated test suites in favor of native Rust binary (`crates/lab-report`). Course `build.py` scripts remain pure Python stdlib.

---

## [0.2.0] - 2026-10-27

### Added
- **Starter Project Generator (`tools/lab_report/generator.py`)** \
  Dual-mode project scaffolding supporting both in-repo module generation and portable standalone export (`lab-report new`).
- **Standalone Zero-Dependency Verifier (`verify.py`)** \
  Generated projects bundle a pure standard-library SVG geometric anchor auditor and typography checker requiring zero `pip` dependencies.
- **Declarative Centralized Verifier (`tools/lab_report/verifier.py`)** \
  Reusable verification engine eliminating duplicated test scripts across course test directories.
- **TUI Project Generator Wizard** \
  Interactive project initialization option directly within `uv run lab-report`.

### Changed
- **English Directory & Tooling Renaming** \
  Course directories renamed to standard English (`database/`, `computer-networks/`, `object-oriented-programming/`) with full backwards-compatible alias resolution.
- **Unified Tooling Rebrand (`lab-report`)** \
  Package renamed to `lab-report-tools` under `tools/lab_report/`, exposing primary CLI binary `lab-report` alongside backwards-compatible `laprak` alias.
- **Zero Shell Script Architecture** \
  Replaced all legacy `.sh` scripts (`build.sh`, `test_fonts.sh`, `test_full_compile.sh`) with cross-platform Python `build.py` running on standard library and Typst CLI.
- **Test Suite Consolidation** \
  Purged redundant copies of `diff-pages.py` and `verify-geometry.py` across course directories, reducing codebase maintenance burden by over 700 lines of duplicated code.

---

## [0.1.0] - 2026-10-27

### Added
- **Unified CLI & TUI Tooling (`lab-report`)** \
  Python-based CLI and interactive terminal UI with `build`, `watch`, `pack`, `test`, `lint`, `inspect`, and `render` commands.
- **Model Context Protocol (MCP) Server (`tools/lab_report/mcp.py`)** \
  Standard JSON-RPC 2.0 stdio server providing autonomous AI tool calling for host inspection, typography auditing, and document compilation.
- **Pure Vector SVG Diagram Generator (`tools/lab_report/diagrams.py`)** \
  Automated PlantUML compiler (`.puml` to `.svg`) adhering to publication-grade document design tokens.
- **Empirical Host Inspector (`tools/lab_report/sys_inspect.py`)** \
  Directly reads hardware specs from host CPU, RAM, OS, and Typst compiler to populate `content/tools-and-materials.typ`.
- **Deterministic Typography Linter (`tools/lab_report/linter.py`)** \
  Audits colons on bold title headers, instructional template placeholders, and layout boundaries with automatic fixing capability.
- **Strict Quality Verification Gates** \
  27-anchor coordinate geometry auditor (`verify-geometry.py`) and full SVG vector regression diff engine (`diff-pages.py`).
- **Canonical Course Renaming** \
  Renamed course directories to canonical lowercase kebab-case (`database/`, `computer-networks/`, `object-oriented-programming/`) while maintaining full backwards-compatible alias resolution.
- **Python-Agnostic Core** \
  Decoupled standalone course builders (`build.py`) from Python dependencies, allowing pure standard-library and Typst compilation.
- **Repository Governance** \
  Added AGPLv3 `LICENSE`, SWEBOK v4 `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, and `AGENTS.md`.

### Changed
- Standardized ISO B5 typography across Computer Networks and Database templates to Word 1.5 equivalent line spacing (`par(leading: 0.75em, spacing: 0.75em)`).
- Formatted numbered steps to use native Typst hanging indents (`+ *Title* \ \n Body`).
- Removed all legacy LaTeX templates and redundant nested virtual environments.
