# Contributing Guide & Quality Gates

Thank you for contributing to the ITERA Lab Report Template & Automation Suite. To ensure consistent typography and zero-tolerance layout precision, all contributions must adhere to the workflows and quality gates outlined below.

---

## 1. Development Environment Setup

1. Install required tools:
   * **Typst** $\ge 0.13.0$
   * **Python** $\ge 3.10$
   * **uv** package manager (`curl -LsSf https://astral.sh/uv/install.sh | sh`)
   * **Rust toolchain** (`rustup`/`cargo`; MSRV follows `crates/lab-report/Cargo.toml`, edition 2021 -- do not pin a version manually)
2. Clone repository and install dependencies:
   ```bash
   git clone <repository-url>
   cd "Template Laprak"
   uv sync
   cargo build -p lab-report
   # or: cargo install --path crates/lab-report
   ```

---

## 2. Local Quality Gates (Pre-Commit Verification)

Before submitting changes or proposing pull requests, you MUST execute the following 4 verification gates:

### Gate 1: Rust Style & Lint
Enforces Rust formatting and clippy lints across the workspace:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

### Gate 2: Rust Tests
Ensures all Rust unit tests pass:
```bash
cargo test
```

### Gate 3: Python Harness Style & Tests
Ruff checks the pytest/ruff harness under `tests/`:
```bash
uv run ruff check tests
uv run pytest
```

### Gate 4: Typography & Geometry (native binary)
Validates course documents for formatting invariants and geometric coordinate anchors within $\le 0.5\text{ pt}$ tolerance:
```bash
lab-report lint computer-networks
lab-report lint database
lab-report test computer-networks
lab-report test database
lab-report test object-oriented-programming
```
Optionally run `asep_check --mode axiomatic` for traceability audits.

---

## 3. Typographic & Formatting Invariants

When adding or editing `.typ` files in any course directory:
- **Word 1.5 Line Spacing** \
  In `lib.typ`, par spacing is strictly calibrated to `leading: 0.75em, spacing: 0.75em`. Never insert manual `#v(...)` spacers between body paragraphs.
- **Prohibition of Colons on Bold Titles** \
  Format item headers as `*Title* \ \n Description...`. Never use `*Title:*` or `*Title*:`.
- **Hanging Indent** \
  Use native Typst numbering (`+ *Title* \ \n Body`) so wrapped lines align flush under the text after the number.
- **Pure Vector Diagrams** \
  Technical diagrams must be created as PlantUML source files (`.puml`) under `<course>/assets/diagrams/` and compiled to vector SVG (`lab-report render`).
- **Python-Agnostic Core** \
  Course documents must remain compilable purely with `python3 build.py` (zero external pip packages) or Typst CLI.

---

## 4. Submitting Changes

1. Create a feature branch \
   `git checkout -b feature/your-feature-name`
2. Commit changes using clear, concise commit messages.
3. Verify that all 4 local quality gates pass.
4. Push your branch and open a Pull Request.
