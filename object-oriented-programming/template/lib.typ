// Main Template Library for PBO Lab Report

#import "cover.typ": make-cover
#import "header-footer.typ": running-header, running-footer
#import "components.typ": red-text, placeholder, section, subsection

#let lab-report(
  modul: "X",
  judul: "JUDUL PRAKTIKUM",
  bulan: "BULAN PENGERJAAN",
  tahun: "2026",
  student: (:),
  class: (:),
  assistants: (),
  body,
) = {
  // Set default document styles
  set document(
    title: [Laporan Praktikum Pemrograman Berorientasi Objek - Modul #modul],
    author: student.at("nama", default: "Mahasiswa"),
  )

  // 1. Generate Cover Page
  make-cover(
    modul: modul,
    judul: judul,
    bulan: bulan,
    tahun: tahun,
    student: student,
    class: class,
    assistants: assistants,
  )

  // 2. Body Page Setup (Pages 2+)
  set page(
    paper: "a4",
    margin: (top: 72.89pt, bottom: 72pt, left: 72pt, right: 72pt),
    header: running-header(),
    footer: running-footer(),
  )

  set text(font: "Times New Roman", size: 12pt, lang: "id")
  set par(leading: 7.93pt, spacing: 7.93pt, justify: false)
  set list(indent: 0pt, body-indent: 6pt, spacing: 7.93pt)
  set enum(indent: 0pt, body-indent: 6pt, spacing: 7.93pt)

  // Heading 1 styling: 14pt bold, proper spacing above/below
  show heading.where(level: 1): it => {
    set text(font: "Times New Roman", size: 14pt, weight: "bold")
    block(above: 18pt, below: 10pt)[
      #it.body
    ]
  }

  // Heading 2 styling: 12pt bold
  show heading.where(level: 2): it => {
    set text(font: "Times New Roman", size: 12pt, weight: "bold")
    block(above: 14pt, below: 8pt)[
      #it.body
    ]
  }

  // Fenced raw code block styling
  show raw.where(block: true): it => block(
    fill: rgb("f8f9fa"),
    inset: 9pt,
    radius: 3pt,
    stroke: 0.5pt + rgb("d0d7de"),
    width: 100%,
    it
  )
  show raw: set text(font: ("DejaVu Sans Mono", "Liberation Mono"), size: 8.5pt)

  body
}
