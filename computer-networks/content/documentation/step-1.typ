#import "../../template/components.typ": step-figure, topic

== Langkah 1 - Perancangan Topologi Jaringan Terintegrasi dan Multi-homed Server DHCP

#topic[A. Perancangan dan Implementasi Topologi Fisik][
  Topologi jaringan dirancang di atas lembar kerja Cisco Packet Tracer untuk mengintegrasikan empat segmen Local Area Network (LAN) yang saling terisolasi pada Layer 2 dan dihubungkan oleh sepasang Router Layer 3. Perangkat yang digunakan meliputi:
  - 2 unit Router (model `Router-PT`) sebagai gerbang komunikasi antar-subnet (_inter-subnet routing_).
  - 4 unit Switch (model `Switch-PT`) yang masing-masing melayani satu subnet independen.
  - 1 unit Access Point (model `AccessPoint-PT`) sebagai jembatan nirkabel pada segmen Subnet 200.
  - 2 unit Server DHCP multi-homed (model `Server-PT`) yang masing-masing terhubung ke dua switch sekaligus.
  - 4 unit Server DNS dan HTTP (model `Server-PT`) yang ditempatkan pada masing-masing subnet.
  - 5 unit PC (model `PC-PT`) dan 5 unit Laptop (model `Laptop-PT`) sebagai perangkat akhir (_end-device_).

  Skema pengalamatan IP kelas C terbagi ke dalam lima domain siaran (_broadcast domain_):
  + *Subnet 122* \
    `192.168.122.0/24`, melayani `Switch2`, `PC0`, `Laptop0`, `Server DNS 1`, dan antarmuka `FastEthernet0` pada `Server DHCP 122 dan 200`.
  + *Subnet 200* \
    `192.168.200.0/24`, melayani `Switch0`, `PC2`, `Laptop2`, `PC4`, `Laptop4`, `AccessPoint0`, `Server DNS 2`, dan antarmuka `FastEthernet1` pada `Server DHCP 122 dan 200`.
  + *Subnet NIM 75* \
    `192.168.75.0/24` (mengacu pada dua digit terakhir NIM mahasiswa `125140075`), melayani `Switch1`, `PC3`, `Laptop3`, `Server DNS 3`, dan antarmuka `FastEthernet0` pada `Server DHCP nim dan 140`.
  + *Subnet 140* \
    `192.168.140.0/24`, melayani `Switch3`, `PC1`, `Laptop1`, `Server DNS 4`, dan antarmuka `FastEthernet1` pada `Server DHCP nim dan 140`.
  + *Tautan Antar-Router (Inter-Router Link)* \
    `192.168.10.0/24`, menghubungkan antarmuka `FastEthernet4/0` pada `Router0` (`192.168.10.1/24`) dengan `FastEthernet4/0` pada `Router1` (`192.168.10.2/24`) menggunakan kabel _copper cross-over_.

  #step-figure("/assets/images/topologi-modul5.png", [Topologi Terintegrasi Modul 5 pada Lembar Kerja Cisco Packet Tracer], width: 35%)
]

#topic[B. Konfigurasi Perangkat Keras dan Multi-homing Server DHCP][
  Secara baku (_default_), modul sasis `Server-PT` hanya menyediakan satu buah antarmuka kabel (`FastEthernet0`). Untuk memungkinkan sebuah server DHCP melayani dua subnet fisik yang berbeda tanpa memerlukan _DHCP Relay Agent_ (IP Helper Address) pada router, server DHCP dikonfigurasi sebagai perangkat _multi-homed_:
  - Server dimatikan sesaat (_power off_), lalu modul ekspansi `PT-HOST-NM-1CFE` dipasang pada slot 1.
  - Setelah server dinyalakan kembali, antarmuka kedua (`FastEthernet1`) terbentuk dan siap disambungkan ke switch pada subnet kedua.

  Penetapan alamat IP statis pada kedua antarmuka Server DHCP dilakukan sebagai berikut:
  - *Server DHCP 122 dan 200*
    - *Antarmuka FastEthernet0 (terhubung ke Switch2 Subnet 122)* \
      Alamat IP `192.168.122.12`, Subnet Mask `255.255.255.0`, Default Gateway `192.168.122.1`, DNS Server `192.168.122.4`.
    - *Antarmuka FastEthernet1 (terhubung ke Switch0 Subnet 200)* \
      Alamat IP `192.168.200.13`, Subnet Mask `255.255.255.0`, Default Gateway `192.168.200.1`, DNS Server `192.168.200.4`.
  - *Server DHCP nim dan 140*
    - *Antarmuka FastEthernet0 (terhubung ke Switch1 Subnet NIM 75)* \
      Alamat IP `192.168.75.12`, Subnet Mask `255.255.255.0`, Default Gateway `192.168.75.1`, DNS Server `192.168.75.4`.
    - *Antarmuka FastEthernet1 (terhubung ke Switch3 Subnet 140)* \
      Alamat IP `192.168.140.11`, Subnet Mask `255.255.255.0`, Default Gateway `192.168.140.1`, DNS Server `192.168.140.3`.
]

#topic[C. Konfigurasi Layanan Pool DHCP dan Analisis Protokol DORA][
  Layanan DHCP diaktifkan pada tab _Services > DHCP_ untuk masing-masing antarmuka server. Kolam alamat (_address pool_) dikonfigurasi dengan nama `serverPool` dengan batas rentang alamat dinamis dimulai dari host `.2` hingga host `.254`:
  - *Antarmuka FastEthernet0 (Subnet 122)* \
    Default Gateway `192.168.122.1`, DNS Server `192.168.122.4`, Start IP `192.168.122.2`, Subnet Mask `255.255.255.0`, Max Users `254`.
  - *Antarmuka FastEthernet1 (Subnet 200)* \
    Default Gateway `192.168.200.1`, DNS Server `192.168.200.4`, Start IP `192.168.200.2`, Subnet Mask `255.255.255.0`, Max Users `254`.
  - *Antarmuka FastEthernet0 (Subnet NIM 75)* \
    Default Gateway `192.168.75.1`, DNS Server `192.168.75.4`, Start IP `192.168.75.2`, Subnet Mask `255.255.255.0`, Max Users `254`.
  - *Antarmuka FastEthernet1 (Subnet 140)* \
    Default Gateway `192.168.140.1`, DNS Server `192.168.140.3`, Start IP `192.168.140.2`, Subnet Mask `255.255.255.0`, Max Users `254`.

  #step-figure("/assets/images/dhcp-server-config.png", [Konfigurasi Layanan Multi-homed DHCP Server pada Subnet 122, 200, NIM 75, dan 140])

  *Mekanisme Pertukaran Pesan DORA (RFC 2131)*
  + *DHCP Discover* \
    Klien mengirimkan paket siaran (_broadcast_) Layer 2 (`FF:FF:FF:FF:FF:FF`) dan Layer 3 (`255.255.255.255`) melalui port UDP 67 untuk menemukan keberadaan server DHCP yang aktif dalam domain siaran.
  + *DHCP Offer* \
    Server DHCP merespons dengan paket penawaran yang memuat alamat IP yang belum terpakai, subnet mask, waktu sewa (_lease time_), dan alamat IP server penawar.
  + *DHCP Request* \
    Klien memilih penawaran tersebut dan memancarkan paket siaran permintaan resmi untuk mengonfirmasi peminjaman konfigurasi IP terkait.
  + *DHCP Acknowledgment (ACK)* \
    Server DHCP mencatat pemetaan MAC address klien dalam tabel sewa (_lease table_) dan mengirimkan konfirmasi final yang memuat parameter Default Gateway dan DNS Server.
]
