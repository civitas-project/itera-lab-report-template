// Cover Page layout for PBO Lab Report with exact coordinate alignment

#let make-cover(
  modul: "X",
  judul: "JUDUL PRAKTIKUM",
  bulan: "BULAN PENGERJAAN",
  tahun: "2026",
  student: (:),
  class: (:),
  assistants: (),
) = {
  // Page 1 has no running header or footer
  set page(
    paper: "a4",
    margin: (top: 0pt, bottom: 0pt, left: 0pt, right: 0pt),
    header: none,
    footer: none,
  )

  let is-placeholder(val) = (
    type(val) == str and (val.starts-with("[") or val in ("X", "JUDUL PRAKTIKUM", "BULAN PENGERJAAN"))
  )

  // Top title lines (16pt Bold)
  place(top + left, dx: 205.43pt, dy: 72.68pt + 0.24pt)[
    #text(font: "Times New Roman", size: 16pt, weight: "bold")[LAPORAN PRAKTIKUM]
  ]
  place(top + left, dx: 138.53pt, dy: 101.84pt + 0.24pt)[
    #text(font: "Times New Roman", size: 16pt, weight: "bold")[PEMROGRAMAN BERORIENTASI OBJEK]
  ]

  // Subtitle block (12pt): centered, conditional color
  place(top + center, dy: 154.69pt + 0.37pt)[
    #text(font: "Times New Roman", size: 12pt)[MODUL #text(fill: if is-placeholder(modul) { rgb("ee0000") } else { rgb("000000") })[#modul]]
  ]
  place(top + center, dy: 178.56pt + 0.37pt)[
    #text(font: "Times New Roman", size: 12pt, fill: if is-placeholder(judul) { rgb("ee0000") } else { rgb("000000") })[#judul]
  ]

  // ITERA Logo
  place(top + left, dx: 229.025pt, dy: 217.747pt)[
    #image("../assets/images/logo-itera.png", width: 137.25pt, height: 137.25pt)
  ]

  // Block 4: Column 1 - Labels (starts at 409.25pt)
  place(top + left, dx: 139.15pt, dy: 409.25pt + 0.37pt)[
    #block[
      #set text(font: "Times New Roman", size: 12pt)
      #set par(leading: 7.92pt)
      Nama\
      NIM\
      Program Studi\
      Mata Kuliah\
      Kelas\
      Dosen Pengampu\
      Tim Asisten
    ]
  ]

  // Block 5: Column 2 - Colons (starts at 409.25pt)
  place(top + left, dx: 238.06pt, dy: 409.25pt + 0.37pt)[
    #block[
      #set text(font: "Times New Roman", size: 12pt)
      #set par(leading: 7.92pt)
      :\
      :\
      :\
      :\
      :\
      :\
      :
    ]
  ]

  let val-style(val, sz: 12pt) = {
    let is-red = (type(val) == str and val.starts-with("[") and val.ends-with("]"))
    text(font: "Times New Roman", size: sz, fill: if is-red { rgb("ee0000") } else { rgb("000000") }, val)
  }

  // Block 6: Disusun Oleh + All Values in one unbroken column
  place(top + left, dx: 252.55pt, dy: 393.38pt + 0.37pt)[
    #block[
      #set text(font: "Times New Roman", size: 12pt)
      #set par(leading: 7.92pt)
      #h(257.15pt - 252.55pt)Disusun Oleh:\
      #val-style(student.at("nama", default: "[Nama Mahasiswa]"))\
      #val-style(student.at("nim", default: "[NIM]"))\
      #val-style(class.at("program-studi", default: "Teknik Informatika"))\
      #val-style(class.at("mata-kuliah", default: "Pemrograman Berorientasi Objek"))\
      #val-style(class.at("kelas", default: "[Kelas PBO (Bukan Kelas Praktikum)]"))\
      #val-style(class.at("dosen", default: "[Nama Dosen sesuai Kelas Besar PBO]"))\
      #h(254.80pt - 252.55pt)#assistants.at(0, default: "1. Jundi Lamtara (124140190)")\
      #h(254.80pt - 252.55pt)#assistants.at(1, default: "2. Mifthahul Rezki Akbar (124140202)")
    ]
  ]

  // Bottom Institution Block (16pt Bold)
  place(top + left, dx: 204.09pt, dy: 631.84pt + 0.24pt)[
    #text(font: "Times New Roman", size: 16pt, weight: "bold")[TEKNIK INFORMATIKA]
  ]
  place(top + left, dx: 161.18pt, dy: 661.00pt + 0.24pt)[
    #text(font: "Times New Roman", size: 16pt, weight: "bold")[FAKULTAS TEKNOLOGI INDUSTRI]
  ]
  place(top + left, dx: 159.84pt, dy: 690.16pt + 0.24pt)[
    #text(font: "Times New Roman", size: 16pt, weight: "bold")[INSTITUT TEKNOLOGI SUMATERA]
  ]
  place(top + center, dy: 719.32pt + 0.24pt)[
    #set text(font: "Times New Roman", size: 16pt, weight: "bold")
    #if is-placeholder(bulan) [
      #text(fill: rgb("ee0000"))[\[#bulan.trim("[").trim("]")\]] #tahun
    ] else [
      #bulan #tahun
    ]
  ]

  pagebreak()
}
