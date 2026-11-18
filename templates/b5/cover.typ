// Outer cover page for the B5 template.
// Coordinates follow the Word and LaTeX reference (main.tex).

#let abuabu = rgb("D6D6D6")     // Kotak judul sampul
#let abubanner = rgb("EAEAEA") // Banner judul sampul

#let render-cover(
  session: 1,
  title: "Judul Praktikum",
  date: "September 2026",
  course: (code: "", name: ""),
  student: (name: "Nama Peserta Praktikum", id: "NIM"),
  class: "RA/RB/RC/RD/RE/RF/RG/RH",
  assistants: ("Nama Asisten Praktikum 1", "Nama Asisten Praktikum 2")
) = {

  // Koordinat skala 0.354mm persis dengan x=0.354mm, y=-0.354mm di LaTeX TikZ
  let u = 0.354mm

  place(
    top + left,
    dx: -25.4mm,
    dy: -25.4mm,
    block(
      width: 176mm,
      height: 250mm,
      clip: true,
      {
      // 1. Teks kiri atas: Cambria 14pt
      place(
        top + left,
        dx: 49.57pt,
        dy: 46.01pt,
        text(font: ("Cambria", "Garamond"), size: 14pt)[Laporan Praktikum]
      )

      // 2. Kotak "Pertemuan": Abu-abu (378.5 to 448.6 x 47.8 to 128.3)
      place(
        top + left,
        dx: 379.80pt,
        dy: 47.96pt,
        rect(
          width: 70.34pt,
          height: 80.78pt,
          fill: abuabu,
          stroke: none,
        )
      )

      place(
        top + left,
        dx: 379.88pt,
        dy: 51.37pt,
        box(
          width: 70.34pt,
          align(center)[#text(font: "Arial", weight: 900, size: 10pt)[Pertemuan]]
        )
      )

      place(
        top + left,
        dx: 379.80pt,
        dy: 83.11pt,
        box(
          width: 70.34pt,
          align(center)[#text(font: "Arial", weight: 900, size: 52pt, fill: white, top-edge: "ascender", bottom-edge: "descender")[#str(session)]]
        )
      )

      // 3. Kode mata kuliah & Garis
      place(
        top + left,
        dx: 48.97pt,
        dy: 149.34pt,
        text(font: "Garamond", size: 12.3pt)[#course.code: #course.name]
      )


      place(
        top + left,
        dx: 45.56pt,
        dy: 163.06pt,
        line(length: 450.95pt - 45.56pt, stroke: 0.5pt + black)
      )

      place(
        top + left,
        dx: 48.97pt,
        dy: 171.57pt,
        text(font: "Garamond", size: 18.5pt)[Laporan Praktikum/Lab]
      )

      // 4. Banner Judul: Abu-abu (15.1 to 479.7 x 236.7 to 333.9)
      place(
        top + left,
        dx: 15.15pt,
        dy: 237.52pt,
        rect(
          width: 466.21pt,
          height: 97.54pt,
          fill: abubanner,
          stroke: none
        )
      )

      place(
        top + left,
        dx: 0.29pt,
        dy: 261.3pt,
        block(
          width: 499.72pt,
          align(center)[
            #context {
              let measured = measure(
                text(font: ("Bernard MT", "Arial"), size: 24pt, weight: "bold", title)
              )
              let scale = calc.min(1, 466.21pt / measured.width)
              set text(
                font: ("Bernard MT", "Arial"),
                size: 24pt * scale,
                weight: "bold"
              )
              set par(leading: 0.35em)
              title
            }
          ]
        )
      )

      // 5. Identitas peserta (rata kanan di x = 451.15pt)
      place(
        top + left,
        dx: 0pt,
        dy: 380.63pt,
        block(
          width: 451.15pt,
          align(right)[
            #set text(font: "Garamond", size: 14pt, weight: "bold")
            #set par(leading: 6.40pt)
            #student.name \
            #class#h(-0.20pt) \
            #student.nim#h(0.48pt)
          ]
        )
      )

      // Tim Asisten (rata kanan di x = 451.15pt)
      place(
        top + left,
        dx: 0pt,
        dy: 446.29pt,
        block(
          width: 451.15pt,
          align(right)[
            #set text(font: "Garamond", size: 14pt, weight: "bold")
            #set par(leading: 6.40pt)
            #underline[Tim Asisten]#h(0.32pt) \
            #assistants.at(0, default: "") \
            #assistants.at(1, default: "")
          ]
        )
      )

      // Institusi (rata kanan di x = 451.15pt)
      place(
        top + left,
        dx: 0pt,
        dy: 548.04pt,
        block(
          width: 451.15pt,
          align(right)[
            #set text(font: "Garamond", size: 14pt, weight: "bold")
            #set par(leading: 6.19pt)
            Teknik Informatika#h(0.15pt) \
            Fakultas Teknologi Industri \
            Institut Teknologi Sumatera \
            #date#h(0.24pt)
          ]
        )
      )

      // 6. Tiga Logo Institusi (rata bawah di baseline y = 649.4 * u)
      place(
        top + left,
        dx: 42.67pt,
        dy: 584.67pt,
        image("/assets/images/logo-itera.png", width: 57.6pt)
      )

      place(
        top + left,
        dx: 122.67pt,
        dy: 616.00pt,
        image("/assets/images/logo-fti.png", width: 114.3pt)
      )

      place(
        top + left,
        dx: 256.67pt,
        dy: 612.67pt,
        image("/assets/images/logo-if.png", width: 156.9pt)
      )
    }
  )
)
}
