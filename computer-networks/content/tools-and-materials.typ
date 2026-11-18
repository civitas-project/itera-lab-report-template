#import "../template/lib.typ": tabhead, tabband, tabline

=== Alat dan Bahan

Praktikum Jaringan Komputer Modul 5 dilaksanakan menggunakan seperangkat fasilitas komputasi keras (_hardware_) dan lunak (_software_) dengan spesifikasi baku berikut:

#align(center)[
  #figure(
    caption: [Spesifikasi Alat dan Bahan Praktikum],
    table(
      columns: (30mm, 42mm, 52mm),
      align: (left, left, left),
      fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },
      stroke: 0.5pt + tabline,
      [Kategori], [Komponen / Perangkat], [Spesifikasi Teknis],
      [Hardware Host], [Unit Komputer Praktikan], [Lenovo ThinkPad T430 (Intel Core i5-3320M @ 2.60GHz, RAM 12 GB, Dual SSD 256 GB + 120 GB)],
      [Sistem Operasi], [Host Operating System], [Void Linux x86_64 (Kernel 6.18.56) / KDE Plasma (Wayland)],
      [Simulasi], [Cisco Packet Tracer], [Versi 9.0.1 (via Distrobox Debian 12 Bookworm)],
      [Dokumentasi], [Typst Typesetting Engine], [Typst CLI v0.15.1 (Modular B5 ISO Template)],
      [Perangkat Jaringan], [Router Modular Simulasi], [2 unit Router-PT (FastEthernet, Static Routing)],
      [Perangkat Jaringan], [Switch Jaringan L2], [4 unit Switch-PT (Layer 2 Switching)],
      [Server Jaringan], [Server Multi-Fungsi], [6 unit Server-PT (2 DHCP Server, 4 DNS & HTTP Server)],
      [Perangkat Nirkabel], [Access Point & Wireless Card], [1 unit AccessPoint-PT & Modul WPC300N (WPA2-PSK)],
      [End-Device Klien], [Workstation & Mobile Unit], [5 unit PC-PT dan 5 unit Laptop-PT],
      [Media Transmisi], [Kabel Virtual & Radio RF], [Copper Straight-Through, Cross-Over, RF 2.4 GHz]
    )
  )
]

