// Shared B5 template.
// Teknik Informatika, Institut Teknologi Sumatera.

#import "./cover.typ": render-cover
#import "./inner-title.typ": render-inner-title
#import "./components.typ": summary-box, dot-line, step-figure, topic, references-list

#let tabhead = rgb("4F81BD")
#let tabband = rgb("DBE5F1")
#let tabline = rgb("95B3D7")

#let lab-report(
  session: 1,
  title: "Judul Praktikum",
  cover-title: none,
  inner-title: none,
  date: "September 2026",
  course: (code: "", name: ""),
  student: (name: "Nama Peserta Praktikum", id: "NIM"),
  class: "RA",
  assistants: ("Asisten 1", "Asisten 2"),
  members: none,
  body
) = {

  // Document metadata
  let doc-meta-title = if type(title) == str { title } else { "Laporan Praktikum" }
  set document(title: doc-meta-title, author: student.name)

  // Paper geometry & layout
  set page(
    width: 176mm,
    height: 250mm,
    margin: (
      left: 25.4mm,
      right: 25.4mm,
      top: 25.4mm,
      bottom: 25.4mm,
    ),
    header: context {
      let page-num = here().page()
      // Skip header on cover (page 1)
      if page-num > 1 {
        move(dy: -5.33pt)[
          #block(width: 100%)[
            #set text(font: "Arial", weight: "black")
            #grid(
              columns: (1fr, auto),
              align: (left + bottom, right + bottom),
              text(size: 7pt, tracking: 0.315em)[LAPORAN PRAKTIKUM],
              text(size: 8.05pt, tracking: -0.0023em)[Teknik Informatika ITERA]
            )
          ]
        ]
        place(top + left, dx: -39.36pt, dy: 48.72pt)[
          #line(length: 429.48pt, stroke: 1.2pt + black)
        ]
        place(top + left, dx: -39.36pt, dy: 51.48pt)[
          #line(length: 429.48pt, stroke: 3.0pt + black)
        ]
      }
    },
    footer: context {
      let page-num = here().page()
      if page-num > 1 {
        place(top + left, dx: 0pt, dy: -0.75pt)[
          #line(length: 100%, stroke: 0.5pt + black)
        ]
        v(8.24pt)
        align(right)[
          #set text(font: "Arial", weight: "black", size: 8pt)
          #str(page-num - 1)
        ]
      }
    }
  )

  // Typography rules
  set text(
    font: ("Garamond", "DejaVu Serif"),
    size: 12pt,
    lang: "id",
    hyphenate: true
  )

  set par(
    justify: true,
    leading: 0.75em,
    spacing: 0.75em,
    first-line-indent: 0pt
  )

  // Section headings matching Word and LaTeX
  show heading.where(level: 1): it => block(
    width: 100%,
    above: 20pt,
    below: 14pt,
    align(center)[
      #set text(font: "Garamond", size: 18pt, weight: "bold")
      #move(dx: 0.39pt)[#it.body]
    ]
  )

  show heading.where(level: 2): it => block(
    width: 100%,
    above: 16pt,
    below: 8pt,
    align(left)[
      #set text(font: "Garamond", size: 12pt, weight: "bold")
      #it.body
    ]
  )

  show heading.where(level: 3): it => block(
    width: 100%,
    above: 16pt,
    below: 10pt,
    align(center)[
      #set text(font: "Garamond", size: 16pt, weight: "bold")
      #it.body
    ]
  )

  // Code / CLI raw block styling - flush with body text
  show raw.where(block: true): it => block(
    width: 100%,
    inset: (x: 0pt, y: 4pt),
    above: 6pt,
    below: 8pt,
    it
  )
  show raw: set text(font: ("DejaVu Sans Mono", "Liberation Mono"), size: 9pt)

  // Figure & table caption styling
  show figure.caption: it => [
    #set text(font: "Garamond", size: 8pt)
    #it.supplement #context it.counter.display(it.numbering). #it.body
  ]
  show figure.where(kind: table): set figure.caption(position: top)
  set figure(gap: 4pt)

  // Table styling
  show table: set text(font: "Garamond", size: 8pt)
  show table.cell.where(y: 0): it => [
    #set text(fill: white, weight: "bold")
    #it
  ]

  // List bullet & enum styling matching Word autoformat (hanging indent)
  set list(
    marker: ([•], [–]),
    indent: 0pt,
    body-indent: 6pt,
    spacing: 0.75em
  )

  set enum(
    indent: 0pt,
    body-indent: 6pt,
    spacing: 0.75em
  )

  // Normalize legacy student schemas to one canonical NIM key.
  let student = student + (
    nim: if "nim" in student { student.nim } else if "student_id" in student { student.student_id } else { student.id },
  )

  // Page 1: Outer Cover
  render-cover(
    session: session,
    title: if cover-title != none { cover-title } else { title },
    date: date,
    course: course,
    student: student,
    class: class,
    assistants: assistants
  )
  pagebreak()

  // Page 2: Inner Title
  render-inner-title(
    title: if inner-title != none { inner-title } else { title },
    members: members,
    student: student
  )
  pagebreak()

  // Body content (Pages 3+)
  body
}
