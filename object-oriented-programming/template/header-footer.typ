// Header and Footer definitions matching Template Praktikum PBO RD.pdf exactly

#let running-header() = context {
  if counter(page).get().first() > 1 {
    place(top + left, dx: 0pt, dy: 37.13pt)[
      #text(font: "Arial", weight: "black", size: 9pt)[LAPORAN PRAKTIKUM]
    ]
    place(top + left, dx: 409.37pt - 72.00pt, dy: 36.00pt)[
      #text(font: "Arial", weight: "black", size: 9pt)[Teknik Informatika ITERA]
    ]
    place(top + left, dx: 36.00pt - 72.00pt, dy: 50.50pt)[
      #line(length: 546.33pt - 36.00pt, stroke: 4.5pt + black)
    ]
  }
}

#let running-footer() = context {
  if counter(page).get().first() > 1 {
    place(top + left, dx: 72.375pt - 72.00pt, dy: 0pt)[
      #line(length: 523.475pt - 72.375pt, stroke: 0.5pt + black)
    ]
    place(top + left, dx: 516.63pt - 72.00pt, dy: 2.60pt)[
      #text(font: "Arial", weight: "black", size: 10pt)[#counter(page).display("1")]
    ]
  }
}
