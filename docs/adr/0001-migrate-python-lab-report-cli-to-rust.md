# ADR-0001: Migrate Python Lab Report CLI to Rust

- **Status:** Accepted
- **Date:** 2026-10-31
- **Deciders:** Technical Architecture Board
- **Traceability:** @adr:0001

---

## 1. Context & Problem Statement
The current lab report toolchain (`tools/lab_report`) relies on Python 3.10+ and external virtual environments (`uv`, `pytest`, `pymupdf`, `rich`, `pillow`, `fonttools`). While flexible for rapid prototyping, this architecture incurs significant operational liabilities:
1. Host dependencies: Users must install Python, `uv`, and system libraries (C compilers, libmupdf) on disparate Linux distributions (Void, Arch, Ubuntu, Debian).
2. Cold startup latency: Python interpreter initialization and PyMuPDF dynamic loading take 300 to 500 ms per invocation, degrading interactive CLI and TUI responsiveness.
3. Distribution complexity: Packaging standalone executables across multiple host architectures requires PyInstaller or wheel bundles that exceed 50 MB and fail on musl-based distributions.

The system requires a native, single-binary executable with sub-10 ms startup time, zero external runtime dependencies, deterministic memory safety, and cross-platform compilation targets.

---

## 2. Decision Drivers
- Single standalone binary distribution with zero host runtime requirements
- Strict sub-10 ms execution latency for build, pack, and lint operations
- Deterministic memory safety without garbage collection overhead
- Rich CLI ecosystem (declarative argument parsing, terminal styling, JSON-RPC for MCP)
- Seamless integration with Typst CLI and standard system calls

---

## 3. Considered Alternatives
- **Alternative 1: Retain Python + uv toolchain**<br>
  Rejected because it imposes Python runtime dependencies on end users, exhibits slow cold-start performance, and creates packaging friction across diverse student host machines.
- **Alternative 2: Migrate to Dart CLI**<br>
  Rejected because Dart native compilation produces larger binaries (20+ MB), requires the Dart runtime SDK for development, and possesses a smaller ecosystem for systems-level PDF and hardware probing compared to Rust.
- **Alternative 3: Migrate to Go**<br>
  Rejected because Go lacks zero-cost abstractions, lacks strict compile-time invariant checking equivalent to Rust's type system, and relies on garbage collection.
- **Alternative 4: Migrate to Rust (Chosen)**<br>
  Selected because Cargo produces standalone statically linked binaries (musl/glibc), achieves sub-5 ms startup latency, guarantees memory safety without garbage collection, and enforces compile-time contract verification.

---

## 4. Decision Outcome
Chosen option: **Migrate Python Lab Report CLI to Rust**, because Rust produces a single statically linked binary with zero external runtime dependencies, sub-5 ms execution latency, and deterministic memory safety.

---

## 5. Pros and Cons of the Options

### Positive Consequences
- Distribution: A single self-contained binary (`lab-report`) can be distributed directly to students via GitHub Releases without requiring Python, uv, or system pip packages.
- Performance: Cold startup time drops from ~400 ms to <5 ms.
- Reliability: Strong static typing and pattern matching eliminate runtime `NoneType` and type coercion errors.
- Footprint: Memory consumption during PDF inspection and ZIP packaging drops from ~80 MB (Python + PyMuPDF) to <10 MB.
- Architecture: 1:1 symmetrical traceability tags (`@req`, `@impl`, `@test`) integrate directly with `asep_check`.

### Negative Consequences / Trade-offs
- Compilation time: Cargo release builds require 30 to 60 seconds compared to instantaneous Python script execution during development.
- Development velocity: Writing strict Rust types and ownership lifecycles requires higher engineering discipline than dynamic Python scripting.
