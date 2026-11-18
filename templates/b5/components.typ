// Content and helper components for the B5 template.
// Layout follows the Word and LaTeX reference (main.tex).

#let abugelap = rgb("808080")

// 1. Kotak Rangkuman (Word Heading2b)
#let summary-box(body) = {
  block(
    width: 100%,
    stroke: 0.5pt + rgb("95B3D7"),
    inset: (
      left: 12pt,
      right: 12pt,
      top: 12pt,
      bottom: 14pt
    ),
    outset: 0pt,
    radius: 0pt,
    [
      #set text(font: "Arial")
      #block(below: 8pt)[
        #text(size: 11pt, weight: "bold", style: "italic")[Rangkuman praktikum]
      ]
      #block(below: 8pt)[
        #text(size: 9.5pt, weight: "bold", style: "italic")[Poin-poin utama yang anda ketahui selama praktikum adalah:]
      ]

      #set text(font: "Garamond", size: 8.5pt)
      #set par(justify: true, leading: 5.5pt, first-line-indent: 0pt)
      #set list(
        marker: text(font: "Arial", size: 5pt, baseline: 1pt)[■],
        indent: 2pt,
        body-indent: 6pt,
        spacing: 7pt
      )
      #body
    ]
  )
}

// Helper baris titik-titik untuk rangkuman
#let dot-line = move(dx: -0.05pt, dy: 0.06pt)[#box(width: 88mm)[#repeat[#text(baseline: 0.28em)[.]#h(1.8pt)]]]

// 2. Gambar Dokumentasi dengan fallback otomatis (default width: 100%)
#let step-figure(image-path, caption, width: 100%, height: auto) = {
  align(center)[
    #show figure.caption: it => [
      #set text(font: "Garamond", size: 8pt)
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
            stroke: 0.5pt + abugelap,
            align(center + horizon)[
              #set text(font: "Arial", size: 9pt, fill: abugelap)
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

// Comparison table with automatic "Tabel N." caption and the B5 table palette.
// columns: column widths, e.g. (22mm, 30mm, 35mm, 30mm); caption: description text.
// Supply at least a header row plus data rows as trailing content cells.
#let comparison-table(columns, caption, ..cells) = {
  let tabhead = rgb("4F81BD")
  let tabband = rgb("DBE5F1")
  let tabline = rgb("95B3D7")
  align(center)[
    #show figure.caption: it => [
      #set text(font: "Garamond", size: 8pt)
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

#let _parse-topic-title(t) = {
  if type(t) == str {
    let m = t.match(regex("^([A-Z0-9]+\.)\s*(.*)$"))
    if m != none {
      (m.captures.at(0), m.captures.at(1))
    } else {
      ("", t)
    }
  } else if type(t) == content {
    if t.func() == enum.item {
      let num = if t.has("number") and t.number != none { str(t.number) } else { "1" }
      (num + ".", t.body)
    } else if t.has("children") {
      let ch = t.children
      if ch.len() > 0 and ch.at(0).has("text") {
        let txt = ch.at(0).text
        let m = txt.match(regex("^([A-Z0-9]+\.)\s*(.*)$"))
        if m != none {
          let p = m.captures.at(0)
          let rest-first = m.captures.at(1)
          let remaining = ch.slice(1)
          let rest-content = if rest-first != "" { (rest-first, ..remaining).sum() } else { remaining.sum() }
          (p, rest-content)
        } else {
          ("", t)
        }
      } else {
        ("", t)
      }
    } else if t.has("text") {
      let m = t.text.match(regex("^([A-Z0-9]+\.)\s*(.*)$"))
      if m != none {
        (m.captures.at(0), m.captures.at(1))
      } else {
        ("", t)
      }
    } else {
      ("", t)
    }
  } else {
    ("", t)
  }
}

// 3. Subjudul Topik Kecil di dalam Langkah Dokumentasi (Word Autoformat Alignment)
#let topic(title, ..args) = {
  let body = if args.pos().len() > 0 { args.pos().at(0) } else { none }
  let (pfx, clean-title) = _parse-topic-title(title)
  block(
    width: 100%,
    above: 14pt,
    below: 8pt,
    if pfx != "" {
      grid(
        columns: (36pt, 1fr),
        align: (left + top, left + top),
        text(font: "Garamond", weight: "bold", size: 12pt)[#pfx],
        text(font: "Garamond", weight: "bold", size: 12pt)[#clean-title]
      )
    } else {
      text(font: "Garamond", weight: "bold", size: 12pt)[#title]
    }
  )
  if body != none {
    pad(left: if pfx != "" { 36pt } else { 0pt })[#body]
  }
}


// 4. Daftar Referensi dengan Hanging Indent 22mm
#let references-list(items, note: none) = {
  v(1.25pt)
  text(
    font: "Arial",
    weight: 900,
    size: 15pt,
    fill: abugelap
  )[Referensi]
  v(16.44pt)

  set text(font: "Garamond", size: 11pt)
  set par(
    leading: 5.8pt,
    hanging-indent: 22mm,
    first-line-indent: 0pt,
    justify: true
  )

  for item in items [
    #par[#item]
    #v(16.44pt)
  ]

  if note != none [
    #set par(hanging-indent: 0pt, justify: false, leading: 5.75pt)
    #note
  ]
}
