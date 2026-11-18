#import "../template/components.typ": summary-box

#align(center)[
  #set text(font: "Garamond", size: 16pt, weight: "bold")
  #move(dx: 0.05pt)[HTTP, Server DHCP, Server DNS, Acces Point dan Routing]
]

#summary-box[
  - Layanan DHCP (_Dynamic Host Configuration Protocol_) mengotomatisasi pendistribusian konfigurasi IP, subnet mask, default gateway, dan DNS server kepada klien melalui siklus empat tahap DORA (_Discover, Offer, Request, Acknowledgment_) pada port UDP 67/68, meniadakan risiko duplikasi IP pada jaringan multi-segmen.
  - Implementasi Server DHCP _multi-homed_ (antarmuka ganda FastEthernet0 dan FastEthernet1) memungkinkan satu unit server fisik melayani alokasi IP dinamis bagi dua subnet yang berbeda secara terisolasi melalui definisi _pool_ terpisah.
  - Routing statis pada Router Layer 3 menghubungkan seluruh segmen subnet yang berbeda melalui tabel rute eksplisit yang mendefinisikan jaringan tujuan, subnet mask, dan alamat router perantara (_next-hop_).
  - _Access Point_ (AP) menjembatani segmen kabel ke jaringan nirkabel (_Wireless LAN_) berbasis standar IEEE 802.11 (2.4 GHz) dengan proteksi enkripsi WPA2-PSK, memungkinkan workstation dan laptop nirkabel terhubung serta memperoleh konfigurasi DHCP secara mulus.
  - Layanan DNS (_Domain Name System_) memetakan nama domain ke alamat IP server web melalui _Resource Record_ bertipe _A Record_ pada port UDP 53, sehingga perangkat klien dapat mengakses layanan web HTTP (port TCP 80) menggunakan URL domain intuitif melalui web browser.
]