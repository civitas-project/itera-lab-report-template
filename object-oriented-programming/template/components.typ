// Components and styling helpers for PBO Lab Report

#let red-text(content) = text(fill: rgb("ee0000"), content)

#let placeholder(content) = red-text([\[#content\]])

// Numbered section heading with hanging indent (Word default 0.5 in = 36pt = 1.27cm).
// number: e.g. "1." or "5."
// title: heading text
// body: section content, indented to align flush with title text
#let section(number, title, body) = {
  block(above: 18pt, below: 10pt,
    grid(
      columns: (36pt, 1fr),
      gutter: 0pt,
      [#set text(font: "Times New Roman", size: 14pt, weight: "bold")
       #number],
      [#set text(font: "Times New Roman", size: 14pt, weight: "bold")
       #title],
    )
  )
  pad(left: 36pt, body)
}

// Sub-section heading with hanging indent (Word default 0.5 in = 36pt).
// number: e.g. "6.1"
// title: sub-heading text
// body: section content, indented to align flush with title text
#let subsection(number, title, body) = {
  block(above: 14pt, below: 8pt,
    grid(
      columns: (36pt, 1fr),
      gutter: 0pt,
      [#set text(font: "Times New Roman", size: 12pt, weight: "bold")
       #number],
      [#set text(font: "Times New Roman", size: 12pt, weight: "bold")
       #title],
    )
  )
  pad(left: 36pt, body)
}

// Numbered figure with automatic "Gambar N." caption (local PBO equivalent of step-figure).
// image-path: path string to the screenshot; caption: description text.
#let output-figure(image-path, caption, width: 100%, height: auto) = {
  align(center)[
    #show figure.caption: it => [
      #set text(font: "Times New Roman", size: 8pt)
      #it.supplement #context it.counter.display(it.numbering). #it.body
    ]
    #figure(
      caption: caption,
      supplement: [Gambar],
      gap: 6pt,
      {
        if image-path == none or image-path == "" or image-path == "placeholder" {
          rect(
            width: width,
            height: 50mm,
            stroke: 0.5pt + rgb("808080"),
            align(center + horizon)[
              #set text(font: "Times New Roman", size: 9pt, fill: rgb("808080"))
              Gambar dokumentasi \
              belum tersedia
            ]
          )
        } else {
          image(image-path, width: width, height: height, fit: "contain")
        }
      }
    )
  ]
}

// Comparison table with automatic "Tabel N." caption and the shared B5 palette.
// columns: column widths, e.g. (30mm, 60mm); caption: description text.
// Supply at least a header row plus data rows as trailing content cells.
#let comparison-table(columns, caption, ..cells) = {
  let tabhead = rgb("4F81BD")
  let tabband = rgb("DBE5F1")
  let tabline = rgb("95B3D7")
  align(center)[
    #show figure.caption: it => [
      #set text(font: "Times New Roman", size: 8pt)
      #it.supplement #context it.counter.display(it.numbering). #it.body
    ]
    #figure(
      caption: caption,
      supplement: [Tabel],
      gap: 6pt,
      table(
        columns: columns,
        align: (x, y) => if y == 0 { center } else { left },
        fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },
        stroke: 0.5pt + tabline,
        ..cells
      )
    )
  ]
}
