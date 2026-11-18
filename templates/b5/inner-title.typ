// Inner title page for the B5 template.
// Coordinates follow the Word and LaTeX reference (main.tex).

#let abujudul = rgb("959595") // Teks & garis halaman judul dalam

#let render-inner-title(
  title: "Judul Praktikum",
  members: none,
  student: none
) = {
  let u = 0.354mm

  // Menyesuaikan origin ke sudut kiri atas halaman (current page.north west)
  // sama seperti TikZ overlay di LaTeX main.tex
  place(
    top + left,
    dx: -25.4mm,
    dy: -25.4mm,
    block(
      width: 176mm,
      height: 250mm,
      clip: true,
      {
      // 1. Fakultas & Tahun: Garamond 9.4pt, tracking 14.5%
      place(
        top + center,
        dy: 140.30pt,
        align(center)[
          #set text(
            font: "Garamond",
            size: 9.4pt,
            fill: abujudul,
            tracking: 0.1260em
          )
          #set par(leading: 10.39pt)
          FAKULTAS TEKNOLOGI INDUSTRI \
          ITERA 2026
        ]
      )

      // 2. Judul Laporan: Arial 900 16pt dengan garis bawah adaptif dalam stack
      place(
        top + center,
        dy: 195pt,
        align(center)[
          #stack(
            spacing: 14pt,
            block(width: 357.48pt)[
              #context {
                let measured = measure(
                  text(font: "Arial", weight: 900, size: 16pt, fill: abujudul, tracking: -0.02em, title)
                )
                let scale = calc.min(1, 357.48pt / measured.width)
                set text(
                  font: "Arial",
                  weight: 900,
                  size: 16pt * scale,
                  fill: abujudul,
                  tracking: -0.02em
                )
                set par(leading: 0.35em)
                title
              }
            ],
            line(length: 357.48pt, stroke: 0.6pt + abujudul)
          )
        ]
      )

      // 3. Anggota Kelompok & Catatan (Terkunci di y = 402.40pt dan 520.00pt)
      let member-note = if type(members) == dictionary { members.at("note", default: "") } else { "" }
      if members != none and (
        (type(members) == dictionary and members.at("list", default: ()).len() > 0) or
        (type(members) == array and members.len() > 0)
      ) {
        // Ada anggota kelompok
        let member-list = if type(members) == dictionary { members.at("list", default: ()) } else { members }

        place(
          top + left,
          dx: 0.60pt,
          dy: 402.42pt,
          block(
            width: 176mm,
            align(center)[
              #set text(font: "Garamond", size: 9.6pt, weight: "bold")
              #set par(leading: 4.88pt)
              #h(-0.68pt)Nama anggota kelompok: \
              #for m in member-list [
                #m \
              ]
            ]
          )
        )
        if member-note != "" {
          place(
            top + left,
            dx: 0.80pt,
            dy: 519.98pt,
            block(
              width: 176mm,
              align(center)[
                #set text(font: "Garamond", size: 9.6pt, weight: "bold")
                #member-note
              ]
            )
          )
        }
      } else if student != none and student.name != none and student.name != "" {
        // Laporan individu: tampilkan nama dan NIM mahasiswa
        place(
          top + left,
          dx: 0.60pt,
          dy: 402.42pt,
          block(
            width: 176mm,
            align(center)[
              #set text(font: "Garamond", size: 9.6pt, weight: "bold")
              #set par(leading: 4.88pt)
              #h(-0.68pt)Nama Mahasiswa: \
              #student.name (#student.nim)
            ]
          )
        )
        // Catatan laporan individu
        if member-note != "" {
          place(
            top + left,
            dx: 0.80pt,
            dy: 519.98pt,
            block(
              width: 176mm,
              align(center)[
                #set text(font: "Garamond", size: 9.6pt, weight: "bold")
                #member-note
              ]
            )
          )
        }
      }

      // 4. Alamat Institusi ITERA (Terkunci di y = 543.43pt dan 599.02pt)
      place(
        top + left,
        dx: 0.55pt,
        dy: 543.43pt,
        block(
          width: 176mm,
          align(center)[
            #set text(font: "Garamond", size: 9.6pt)
            #set par(leading: 4.88pt)
            ITERA Laboratory \
            #h(0.16pt)Program Studi Teknik Informatika \
            #h(0.06pt)Fakultas Teknologi Industri \
            #h(0.06pt)Institut Teknologi Sumatera
          ]
        )
      )

      place(
        top + center,
        dy: 599.02pt,
        align(center)[
          #set text(font: "Garamond", size: 9.6pt)
          #set par(leading: 4.88pt)
          #h(-0.77pt)Jl. Terusan Ryacudu, Desa Way Hui, Kecamatan Jati Agung, \
          #h(1.25pt)Lampung Selatan -- Lampung 35365 \
          #h(0.43pt)Telp. +62-721-8030188; 8030189
        ]
      )
    }
  )
)
}
