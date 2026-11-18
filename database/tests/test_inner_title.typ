#import "../template/inner-title.typ": render-inner-title

#set page(width: 176mm, height: 250mm, margin: (x: 25.4mm, y: 25.4mm))
#render-inner-title(
  title: "Judul Praktikum",
  members: (
    list: ("Anggota 1 (12345)", "Anggota 2 (67890)"),
    note: "Jika tugas individu cukup tulis nama dan nim saja"
  )
)
