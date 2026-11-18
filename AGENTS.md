# Repository Rules & Invariants: ITERA Lab Reports

This repository contains modular Typst templates and CLI/TUI tooling for Informatics Engineering laboratory reports at Institut Teknologi Sumatera (ITERA).

All agents, whether generating new reports, modifying existing content, or auditing layouts, MUST strictly abide by the following non-negotiable rules.

---

## 1. Absolute Typography & Formatting Invariants

1. **Uniform 1.5 Line Spacing (No Extra Paragraph Gaps)**
   - Computer Networks (`computer-networks`) and Database (`database`) templates use ISO B5 with Garamond 12pt.

   - Par spacing must be strictly set to:
     ```typst
     set par(justify: true, leading: 0.75em, spacing: 0.75em, first-line-indent: 0pt)
     set list(indent: 0pt, body-indent: 6pt, spacing: 0.75em)
     set enum(indent: 0pt, body-indent: 6pt, spacing: 0.75em)
     ```
   - **Never** add manual vertical gaps (`#v(...)` or double blank lines) between body paragraphs. Consecutive paragraphs must flow with a single newline.

2. **Strict Prohibition of Colons on Bold Titles**
   - In step documentation, lists, and task breakdowns, bold title headers must NEVER be followed by a colon (`:`).
   - Format: `*Judul Item* \ \n Penjelasan...`
   - **Banned** \
     `*Judul Item:* Penjelasan...` or `*Judul Item*: Penjelasan...`

3. **Two-Line Item Format & Hanging Indents**
   - For numbered steps or sub-items:
     ```typst
     + *Judul Langkah* \
       Penjelasan teknis yang secara otomatis berposisi menjorok sejajar di bawah teks judul (hanging indent), bukan di bawah angka nomor.
     ```
   - For subtopic sections: use `#topic[A. Judul][Isi...]` with Word-standard hanging indent (tab stop 18pt).

4. **Zero Placeholder Anomaly Text**
   - Never leave template instructions in output documents (e.g. `Jelaskan Langkah-langkah yang dilakukan selama praktikum`).
   - If an instructional placeholder exists, remove it immediately.

5. **Adaptive Cover & Inner Title Stacks**
   - Banner title (Page 1): `Bernard MT`, `size: 22pt`, `par(leading: 0.65em, justify: false)`.
   - Inner title (Page 2): `Arial 900`, `size: 23.5pt`, `par(leading: 0.55em)`.
   - The horizontal separator rule must be placed inside `#stack(spacing: 18pt, [ #title ], line(...))` so that multi-line titles expand naturally without creating empty phantom gaps.

6. **Table & Figure Width Constraints**
   - Tables must fit within the B5 ISO printable textwidth: total table width $\le 125\text{ mm}$.
   - Figures must use tight-crop screenshots (`-trim +repage -border 25`) and be inserted via `#step-figure("<path>", [caption], width: 100%)` (B5) or `#output-figure("<path>", [caption], width: 100%)` (OOP); both accept a path string and auto-number captions as `Gambar N.`. Per-figure `width` overrides are allowed.

---

## 2. Grounding & Zero-Hallucination Contracts

1. **Real Host Hardware Specifications**
   - In `content/tools-and-materials.typ`, hardware specs must reflect empirical terminal outputs from the host machine (`lscpu`, `free`, `uname`, `lsblk`, `typst --version`).
   - **Never** fabricate hardware specs (e.g. "Intel Core i7 16GB RAM" on an AMD Ryzen machine).

2. **NIM & Module Formula Fidelity**
   - Derive IP addressing and student variables directly from the official module guidelines and the student's NIM (from `profile.toml` or `data/student.typ`).
   - **Never** invent mathematical modulo/offset formulas unless explicitly requested by the module.

3. **Visual Verification Authenticity**
   - Network / CLI verification figures MUST display authentic CLI command outputs (e.g. ping replies `Reply from ...`, `show ip route`, SQL execution results, or Java console logs).
   - Never supply an empty topology canvas as proof of connectivity or routing.

---

## 3. Tooling & Execution

- Use the unified `lab-report` native CLI binary:
  - `lab-report build <course> -m <module>` \
    Compile document to PDF (or SVG with `--svg`). Auto-detects course when executed inside course folder.
  - `lab-report watch <course> -m <module>` \
    Live compilation watcher.
  - `lab-report new <course> [dest]` \
    Scaffold a new lab report starter project with automated `.agent-context.md`.
  - `lab-report test <course>` \
    Run declarative coordinate geometry and font verification.
  - `lab-report lint <course> [--fix] [--dry-run]` \
    Audit typography rules (colons on bold headers, placeholders).
  - `lab-report inspect <course> [--write]` \
    Inspect empirical host specs and populate tools-and-materials.typ.
  - `lab-report render <course> [-p file]` \
    Compile PlantUML (.puml) diagrams into pure vector SVG assets.
  - `lab-report pack <course> -m <module>` \
    Bundle PDF and code into a submission ZIP.
  - `lab-report mcp` \
    Launch standard Model Context Protocol (MCP) JSON-RPC 2.0 server for AI agents.
  - `lab-report init` \
    Launch interactive bilingual setup wizard (English default / Bahasa Indonesia).
  - `lab-report profile [--show] [--name <str>] [--nim <str>] [--class <str>] [--lang <en|id>]` \
    Inspect or mutate student metadata.
- For deep procedures, consult the specialized skill \
  `itera-lab-report-standard`.

