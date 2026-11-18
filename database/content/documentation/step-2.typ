#import "../../template/lib.typ": tabhead, tabband, tabline
#import "../../template/components.typ": topic

#topic[B. Langkah Kedua Praktikum][
  Tahapan eksekusi skema basis data dan pengujian integritas relasional. Hasil pembuatan atribut disajikan dalam tabel data terstruktur berikut:

  #align(center)[
    #figure(
      caption: [Contoh format tabel praktikum],
      supplement: [Tabel],
      table(
        columns: (15mm, 35mm, 35mm, 35mm),
        align: (center + horizon, left + horizon, left + horizon, left + horizon),
        stroke: 0.5pt + tabline,
        fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },
        table.header(
          [No], [Judul Kolom 1], [Judul Kolom 2], [Judul Kolom 3]
        ),
        [1], [Data 1], [Data 2], [Data 3],
        [2], [Data 4], [Data 5], [Data 6]
      )
    )
  ]
]
