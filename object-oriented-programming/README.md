# Object-Oriented Programming Lab Report Template (Typst)

Official Object-Oriented Programming (OOP) laboratory report template for Institut Teknologi Sumatera (ITERA) based on [Typst](https://typst.app/), reconstructed with **absolute geometric accuracy (zero-tolerance sub-point error $\le 0.09\text{ pt}$)** against the reference document `Template Praktikum PBO RD.pdf`.

Equipped with a standalone Python standard-library compiler (`python3 build.py`), automated submission ZIP packaging (`<Name>_<NIM>_Tugas<Module>.zip`), and integration with the unified `lab-report` automation suite.

---

## Workspace Layout

```text
object-oriented-programming/
├── pyproject.toml              # Project metadata & strict Ruff configuration
├── build.py                    # Cross-platform standalone builder
├── main.typ                    # Main Typst entrypoint
├── assets/images/              # High-resolution ITERA logo
├── fonts/                      # TrueType font bundle (Times New Roman, Arial Black)
├── data/                       # Student configuration & report metadata
│   ├── student.typ             # Student name and NIM
│   ├── report.typ              # Module number, title, month, and year
│   ├── class.typ               # Class code and lecturer
│   └── assistants.typ          # Lab assistants
├── template/                   # Layout architecture and typography
│   ├── lib.typ                 # Root template wrapper (ISO A4)
│   ├── cover.typ               # Geometric cover layout
│   ├── header-footer.typ       # Running header bar and footer pagination
│   └── components.typ          # Red text and placeholder components
├── content/                    # Modular report chapters
│   ├── 01-tujuan.typ           # Objectives
│   ├── 02-alat-bahan.typ       # Tools and materials
│   ├── 03-langkah.typ          # Experimental procedures
│   ├── 04-source-code.typ      # Java code listings
│   ├── 05-output.typ           # Output screenshots
│   ├── 06-analisis.typ         # Technical analysis
│   └── 07-kesimpulan.typ       # Conclusions
├── src/                        # Java source files (.java)
└── tests/                      # Declarative geometric verifications & test suites
```

---

## Quick Start

### 1. Prerequisites
- [Typst](https://github.com/typst/typst) \
  Version $\ge 0.13.0$ (`typst --version`)
- [Python](https://www.python.org/) \
  Version $\ge 3.10$ (`python3 --version`)
- [uv](https://docs.astral.sh/uv/) \
  Optional, for running full test suites (`uv --version`)

### 2. Configure Student & Report Metadata
Edit variables in `data/`:
- **`data/student.typ`**
  ```typst
  #let student-data = (
    nama: "Timothy Ronald",
    nim: "124140201",
  )
  ```
- **`data/report.typ`**
  ```typst
  #let report-data = (
    modul: "1",
    judul: "PENGENALAN BAHASA JAVA",
    bulan: "MARET",
    tahun: "2026",
  )
  ```
- **`data/class.typ`**
  ```typst
  #let class-data = (
    program-studi: "Teknik Informatika",
    mata-kuliah: "Pemrograman Berorientasi Objek",
    kelas: "RA",
    dosen: "Dr. Eng. Nama Dosen, S.T., M.T.",
  )
  ```

### 3. Add Java Source Files
Place all Java source files in `src/`:
- `src/Percobaan 1.java`
- `src/Percobaan 2.java`
- `src/Percobaan 3.java`
- `src/Percobaan 4.java`

---

## Execution Commands

### Mode A: Standalone Builder (`python3 build.py`)
```bash
# Compile report to output.pdf
python3 build.py

# Live watch mode
python3 build.py watch
```

### Mode B: Unified CLI (`uv run lab-report`)
```bash
# Compile and build report
uv run lab-report build object-oriented-programming -m 1

# Run geometric coordinate verification
uv run lab-report test object-oriented-programming

# Audit typography and quality invariants
uv run lab-report lint object-oriented-programming

# Package submission ZIP (PDF + Java sources)
uv run lab-report pack object-oriented-programming -m 1
```

---

## Submission Archive Structure (`lab-report pack`)

The generated ZIP archive is named `<Name>_<NIM>_Tugas<Module>.zip` with a flat root structure conforming to academic submission requirements:

```text
Timothy_Ronald_124140201_Tugas1.zip
├── Timothy_Ronald_124140201_Tugas1.pdf
├── Percobaan 1.java
├── Percobaan 2.java
├── Percobaan 3.java
└── Percobaan 4.java
```
