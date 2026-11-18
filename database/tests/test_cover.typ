#import "../template/cover.typ": render-cover

#set page(width: 176mm, height: 250mm, margin: 0pt)
#render-cover(
  session: 1,
  title: "Judul Praktikum",
  date: "September 2026",
  student: (name: "Nama Peserta Praktikum", id: "NIM"),
  class: "RA/RB/RC/RD/RE/RF/RG/RH",
  assistants: ("Nama Asisten Praktikum 1", "Nama Asisten Praktikum 2")
)
