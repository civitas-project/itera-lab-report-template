# ITERA Lab Report Template & Automation Suite

> Native Rust CLI and modular Typst templates for building, linting, inspecting, and packaging ITERA Informatics lab reports locally, with zero network calls required.

---

<!-- @gov:README-SUMMARY -->
## Overview

The repository ships a native Rust CLI in `crates/lab-report` (16 source modules, 13 subcommands) plus three course report roots (`computer-networks`, `database`, `object-oriented-programming`) built on modular Typst files in `templates/b5` with per-course shims. Data flow is deterministic and local: `profile.toml` (student name, NIM, class, language) synchronizes into `data/*.typ` per course, the Typst engine compiles `main.typ` with `content/` and `template/` into `dist/<name>_<nim>_TugasN.pdf`, and `lab-report pack` bundles the PDF with course source into a submission ZIP. Quality gates (`lint`, `test`, `audit`, `asep_check`) run on the host; no network calls are required to build or verify a report.

---

<!-- @gov:README-PREREQ -->
## Target Platforms

- **Linux**<br>
  x86_64 and aarch64.
- **macOS**<br>
  Apple Silicon and Intel.
- **Windows**<br>
  x86_64, installed via `install.sh` or `install.ps1`.

## Prerequisites

- **Typst**<br>
  0.15.1 required for `typst compile` / `typst watch` used by the runner and `build.py` fallback.
- **Rust**<br>
  Edition 2021 toolchain to build `crates/lab-report`; `Cargo.toml` declares no `rust-version`, so no MSRV is asserted.
- **Python**<br>
  3.10+ only for the optional standalone `build.py` fallback.
- **uv**<br>
  Optional; absent on PATH in this environment. Python tests run with `.venv/bin/python -m pytest -n auto`.

---

<!-- @gov:README-HIGHLIGHTS -->
## Architecture Highlights

- **Memory model**<br>
  `unsafe_code = "deny"` at workspace level.
- **Concurrency**<br>
  scoped threads parallelize probes and SVG parsing.
- **Traceability**<br>
  1:1 symmetrical traceability, 19 requirements.
- **Zero-drift topology**<br>
  physical directories map 1:1 to logical bounded contexts; no unmapped directories.
- **Empirical host inspection**<br>
  `lab-report inspect` reads `lscpu`, `/proc/meminfo`, `lsblk` to populate `tools-and-materials.typ` with real hardware specs; no fabricated CPU/RAM claims.

---

<!-- @gov:README-LAYOUT -->
### Workspace Layout

```text
.
├── AGENTS.md
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.toml
├── Cargo.lock
├── GAP_REGISTER.md
├── LICENSE
├── MANUAL.md
├── SECURITY.md
├── profile.toml
├── profile.toml.example
├── pyproject.toml
├── uv.lock
├── install.sh
├── install.ps1
├── .agents/
├── .cargo/
├── computer-networks/
├── database/
├── object-oriented-programming/
├── crates/
│   └── lab-report/
├── templates/
│   └── b5/
├── tests/
└── docs/
    ├── PRD.md
    ├── TDD.md
    └── adr/
```

`contoh laprak/` and `anti-slop/` are local, untracked artifacts; they are not part of the release artifact.

### Software Configuration Items (SCI) Traceability Matrix

<!-- @gov:SCI-CLI @gov:SCI-TEMPLATE @gov:SCI-COURSE @gov:SCI-TESTS -->
| Software Configuration Item | Traceability Tag | Canonical Scope | Primary Artifacts |
| :--- | :--- | :--- | :--- |
| **Native CLI**<br>`crates/lab-report/` | hidden HTML comment | Rust toolchain: 16 modules, 13 subcommands | `crates/lab-report/Cargo.toml`, `src/*.rs` |
| **Typst Templates**<br>`templates/b5/` + course shims | hidden HTML comment | ISO B5/A4 modular templates, Garamond/Times New Roman | `templates/b5/*.typ`, `main.typ`, `template/`, `content/` |
| **Course Reports**<br>`computer-networks/`, `database/`, `object-oriented-programming/` | hidden HTML comment | Per-course report sources, data, and PDF output | `main.typ`, `data/*.typ`, `content/`, `template/`, `dist/` |
| **Verification**<br>`tests/` | hidden HTML comment | Coordinate geometry, font, env, and build-script tests | `tests/*.py`, `crates/lab-report/tests/` |

---

<!-- @lim:README-LIMITS -->
## Known Limitations

- **No OCR**<br>
  Scanned PDF inputs use agent vision; the toolchain extracts text only from digital PDFs and Typst sources.
- **Java / MariaDB not bundled**<br>
  Runtimes are not included; authentic proof (console logs, query results) depends on host-installed tooling.
- **`uv` absent on PATH**<br>
  `uv` is optional and not installed on this host; Python verification uses `.venv/bin/python -m pytest -n auto`.
- **Release CI removed**<br>
  Release CI is intentionally removed; publishes are manual and `CHANGELOG.md` records release history.
- **`contoh laprak/` local**<br>
  Extracted `contoh laprak/` examples are local and untracked; not part of the release artifact.

---

<!-- @gov:README-QUICKSTART -->
## Quick Start

```bash
# 1. Clone repository
git clone https://github.com/civitas-project/itera-lab-report-template
cd itera-lab-report-template

# 2. Build the native CLI
cargo build -p lab-report

# 3. Scaffold a new course report
lab-report new computer-networks ~/laprak

# 4. Set student identity
lab-report profile --name "Nama" --nim "125140075" --class "RB"

# 5. Build PDF
lab-report build computer-networks -m 5

# 6. Run quality gates
lab-report audit

# 7. Package submission
lab-report pack computer-networks -m 5
```

Expected verifiable output:
```text
>> Compilation successful.
[OK] ZIP freshness OK: ...
```

---

<!-- @gov:README-CONTRIB -->
## Quality Gates

All contributions must pass automated gates with **Exit Code 0** before submitting a pull request:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
.venv/bin/python -m pytest -n auto
asep_check --mode axiomatic
```

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

<!-- @gov:README-BENCH -->
## Benchmarks

<!-- @gov:BENCH-BASELINE -->
Baseline hardware: i5-3320M, 12 GB RAM, 120 GB SSD / 111.8 GiB, Void Linux 6.18.62_1, Typst 0.15.1.

<!-- @gov:BENCH-CARGO-TEST @gov:BENCH-CLIPPY @gov:BENCH-PYTEST @gov:BENCH-GEOMETRY @gov:BENCH-LINT -->
| Operation | Traceability Tag | Workload Benchmark | Latency (p50) | Latency (p99) | Memory / Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **cargo test**<br>workspace test suite | hidden HTML comment | 28 tests | not measured | not measured | 28 passed / 0 failed |
| **cargo clippy**<br>workspace lints | hidden HTML comment | all targets, `-D warnings` | not measured | not measured | 0 warnings |
| **pytest**<br>repo Python tests | hidden HTML comment | 5 tests, `-n auto` | not measured | not measured | 5 passed |
| **lab-report test**<br>geometry anchors | hidden HTML comment | coordinate & font verification | not measured | not measured | 0.00pt deviation |

Reproduce benchmarks locally:
```bash
cargo test --workspace
```

---

<!-- @gov:README-LEGAL -->
## Governance

<!-- @gov:ARCH-OVERVIEW @gov:CONTRIB-VERIFY @gov:SEC-SLA @gov:GOV-RELEASE @gov:GOV-LICENSE -->
| Governance Document | Traceability Tag | Responsibility & Policy Enforcement |
| :--- | :--- | :--- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | | Architectural topology map, crate boundaries, and 5 Non-Negotiable Invariants |
| [CONTRIBUTING.md](CONTRIBUTING.md) | | Contributor SDLC, Dual-Branch git topology (`main`/`dev`), and 5-Gate CI |
| [SECURITY.md](SECURITY.md) | | Coordinated Vulnerability Disclosure (CVD), CVSS triage, and SLA response matrix |
| [CHANGELOG.md](CHANGELOG.md) | | Keep a Changelog v1.1.0 semantic versioning and release provenance |
| [LICENSE](LICENSE) | | Permissive open-source license and legal IP boundaries |
| [docs/PRD.md](docs/PRD.md) | | 19 requirements; `trace_manifest.json` is not generated |
