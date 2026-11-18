#import "../template/lib.typ": tabhead, tabband, tabline
#import "../template/components.typ": step-figure, topic

=== Tugas Praktikum

Bagian ini menyajikan pembahasan komprehensif atas tugas mandiri Praktikum Modul 5 yang mencakup analisis komparatif arsitektural antara Server Domain Name System (DNS) dan Server Hypertext Transfer Protocol (HTTP), perincian alur kerja operasional, serta sintesis evaluasi teknis implementasi seluruh modul latihan terintegrasi.

#topic[1. Analisis Komparatif Arsitektural antara Server DNS dan Server HTTP][
  Meskipun keduanya beroperasi pada _Application Layer_ (Layer 7) model referensi OSI dan menjadi fondasi utama interaksi Internet modern, Server DNS dan Server HTTP mengemban tanggung jawab komputasi, mekanisme transmisi, serta model pertukaran data yang sangat berbeda secara fundamental.

  #set enum(numbering: "A.")
  + *Domain Name System (DNS Server)* \
    DNS berfungsi sebagai basis data terdistribusi (_distributed hierarchical database_) yang memetakan nama domain hierarkis (_Fully Qualified Domain Name_ / FQDN) ke alamat numerik IP (RFC 1034 dan RFC 1035). DNS beroperasi terutama di atas protokol transport UDP (_User Datagram Protocol_) pada _well-known port_ 53 guna meminimalkan _overhead_ jabat tangan (_handshake_) serta memastikan latensi kueri yang ultra-rendah. Koneksi TCP port 53 hanya digunakan apabila ukuran muatan (_payload_) data melampaui batas standar 512 byte (misalnya pada _zone transfer_ DNS AXFR/IXFR atau respons DNSSEC). Karakteristik DNS bersifat _stateless_, di mana setiap kueri diperlakukan secara atomik dan independen.

  + *Hypertext Transfer Protocol (HTTP Server)* \
    HTTP berfungsi sebagai protokol pertukaran dan penyajian berkas hiperteks (_hypertext markup_ dan multimedia) dalam arsitektur klien-server (RFC 2616 dan RFC 9110). HTTP beroperasi di atas protokol transport TCP (_Transmission Control Protocol_) pada port 80 (atau port 443 pada varian terenkripsi TLS/HTTPS). HTTP mensyaratkan pembentukan koneksi andal melalui jabat tangan 3 langkah (_TCP 3-Way Handshake: SYN, SYN-ACK, ACK_) sebelum payload aplikasi dapat dipertukarkan, menjamin integritas transmisi data tanpa kehilangan paket (_lossless transmission_).

  Tabel berikut menyajikan perbandingan komparatif parameter teknis antara Server DNS dan Server HTTP:

  #align(center)[
    #figure(
      caption: [Analisis Komparasi Parameter Teknis Server DNS vs Server HTTP],
      table(
        columns: (28mm, 48mm, 48mm),
        align: (left, left, left),
        fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },
        stroke: 0.5pt + tabline,
        [Parameter], [Server DNS], [Server HTTP],
        [Fungsi Utama], [Resolusi nama host/domain ke alamat IP numerik], [Penyajian dan transfer berkas hiperteks/web ke klien],
        [Lapisan OSI], [Application Layer (Layer 7)], [Application Layer (Layer 7)],
        [Protokol Transport], [UDP (utama) dan TCP (transaksi besar/zone)], [TCP secara eksklusif (berorientasi koneksi)],
        [Nomor Port Standar], [Port 53 (UDP/TCP)], [Port 80 (HTTP) dan Port 443 (HTTPS)],
        [Model Komunikasi], [Kueri-respons atomik (_query-response_)], [_Client-server request-response_ (GET, POST)],
        [Format Pesan Data], [Struktur biner terformat (Header, Question, RR)], [Teks berbasis ASCII/MIME (Headers) + Payload],
        [Overhead Latensi], [Sangat rendah (tanpa jabat tangan transport)], [Lebih tinggi (butuh 3-way handshake TCP)],
        [Mekanisme Cache], [Berdasarkan parameter TTL (_Time-to-Live_) RR], [Header HTTP (`Cache-Control`, `ETag`, `Expires`)],
        [Struktur Basis Data], [Tipe Record: A, AAAA, CNAME, MX, NS, PTR], [Sistem berkas dokumen (_web root_, HTML, assets)],
        [Standar Regulasi], [IETF RFC 1034 dan RFC 1035], [IETF RFC 2616 dan RFC 9110]
      )
    )
  ]

  #enum(numbering: "A.", start: 3)[
    *Sinergi Operasional dalam Satu Transaksi Penjelajahan Web* \
    Ketika pengguna mengetikkan `http://contoh.com` pada peramban web klien:
    #enum(numbering: "1.")[
      *Fase 1 (Resolusi DNS)* \
      Klien mengecek cache lokal. Jika nihil, klien mengirimkan paket kueri UDP ke server DNS lokal (`192.168.122.4`) menanyakan `A Record` untuk `contoh.com`. Server DNS membalas dengan muatan alamat IP `192.168.122.4`.
    ][
      *Fase 2 (Inisiasi TCP HTTP)* \
      Klien mengirimkan segmen `SYN` ke alamat `192.168.122.4:80`. Server web membalas dengan `SYN-ACK`, dan klien menyelesaikannya dengan `ACK`.
    ][
      *Fase 3 (Pengambilan Sumber Daya)* \
      Klien mengirimkan `GET / HTTP/1.1`. Server HTTP memproses permintaan dan mengirimkan balasan `HTTP/1.1 200 OK` yang memuat berkas `index.html`. Browser merender dokumen tersebut ke layar pengguna.
    ]
  ]
]

#topic[2. Perincian Langkah Kerja dan Analisis Teknis Seluruh Latihan Praktikum][
  Implementasi menyeluruh praktikum ini mengintegrasikan empat pilar utama rekayasa jaringan komputer: alokasi pengalamatan dinamis (DHCP), perutean antar-jaringan (Routing Statis), perluasan media transmisi (Access Point Nirkabel), dan layanan aplikasi terdistribusi (DNS & HTTP).

  #set enum(numbering: "A.")
  + *Latihan Server DHCP Multi-homed*
    - *Langkah Operasional* \
      Pemasangan modul ekspansi antarmuka `PT-HOST-NM-1CFE` pada slot 1 Server DHCP, konfigurasi alamat IP statis pada port `FastEthernet0` dan `FastEthernet1`, serta aktivasi pool alamat dinamis `serverPool` dengan batas alokasi host `.2` s.d. `.254`.
    - *Analisis Teknis* \
      Sifat siaran (_broadcast_) paket `DHCP Discover` (MAC `FF:FF:FF:FF:FF:FF`) dibatasi secara fisik oleh router pada Layer 3. Tanpa konfigurasi _multi-homing_ (dua kartu antarmuka fisik yang masing-masing tertancap ke switch subnet berbeda), sebuah server DHCP tunggal tidak akan pernah menerima paket Discover dari subnet kedua tanpa bantuan konfigurasi `ip helper-address` (DHCP Relay Agent) pada interface router. Desain multi-homing ini mengeliminasi ketergantungan relay agent sekaligus memelihara isolasi domain siaran.

  + *Latihan Routing Statis Antar-Subnet*
    - *Langkah Operasional* \
      Penetapan IP gateway pada interface router (`Router0` dan `Router1`), penghubungan tautan inter-router `192.168.10.0/24` menggunakan kabel _copper cross-over_ pada `FastEthernet4/0`, dan penambahan rute statis dua arah (`ip route [dest_net] [mask] [next_hop]`).
    - *Analisis Teknis* \
      Routing statis menjamin efisiensi pemrosesan router karena tidak mengonsumsi bandwidth transmisi untuk pertukaran pesan periodik seperti pada protokol dinamis (RIP/OSPF). Nilai _Administrative Distance_ bernilai `1` memberikan prediktabilitas mutlak atas jalur paket data. Pengujian tabel rute (`show ip route`) membuktikan bahwa rute statis berkode `S` hanya akan muncul aktif apabila antarmuka keluar (_egress interface_) berstatus _up/up_.

  + *Latihan Konektivitas Access Point Nirkabel*
    - *Langkah Operasional* \
      Pemasangan `AccessPoint0` pada `Switch0` (Subnet 200), pengaturan Port 1 dengan SSID `WIFI_ITERA_IF` dan proteksi `WPA2-PSK` (passphrase: `itera2025`), serta penggantian NIC end-device `PC4` dan `Laptop4` menggunakan modul nirkabel `PT-HOST-NM-1W` / `PT-LAPTOP-NM-1W`.
    - *Analisis Teknis* \
      Access Point berfungsi sebagai jembatan _Layer 2 Media Converter_ murni. Frame 802.11 yang dipancarkan melalui udara didekapsulasi oleh AP dan diubah menjadi frame 802.3 Ethernet untuk dialirkan ke port switch. Enkripsi AES berbasis WPA2-PSK menjamin keamanan data pada medium terbuka (_air interface_), sementara integrasi ke Subnet 200 memungkinkan klien nirkabel memperoleh konfigurasi IP otomatis dari server DHCP tanpa batasan fisik kabel.

  + *Latihan Server DNS dan Layanan Web HTTP*
    - *Langkah Operasional* \
      Pengaktifan _DNS Service_ pada server, penambahan A Record `contoh.com` mengarah ke alamat IP server lokal, konfigurasi DNS Server IP pada DHCP pool, pengaktifan layanan HTTP port 80, dan penyuntingan konten laman web `index.html`.
    - *Analisis Teknis* \
      Sinergi antara DHCP, DNS, dan HTTP menyempurnakan otomatisasi jaringan. Klien tidak perlu menghafal alamat IP numerik server, melainkan cukup memanggil nama domain `contoh.com`. Pembaruan otomatis parameter DNS via DHCP menjamin seluruh klien dalam subnet langsung mengadopsi resolver lokal yang valid.

  *Matriks Evaluasi Hasil Pengujian Konektivitas End-to-End* \
  Pengujian konektivitas menyeluruh memvalidasi keberhasilan komunikasi lintas-arsitektur:

  #align(center)[
    #figure(
      caption: [Matriks Hasil Uji Konektivitas Protokol ICMP, DNS, dan HTTP],
      table(
        columns: (20mm, 20mm, 35mm, 25mm, 24mm),
        align: (center, center, center, center, center),
        fill: (x, y) => if y == 0 { tabhead } else if calc.even(y) { tabband } else { none },
        stroke: 0.5pt + tabline,
        [Klien], [Tujuan], [Parameter Target], [Status Uji], [Keterangan],
        [PC0], [Server DNS 1], [Ping 192.168.122.4], [Success (4/4)], [Latensi \<1 ms, TTL 128],
        [PC0], [PC3 (NIM)], [Ping 192.168.75.3], [Success (4/4)], [Latensi 10 ms, TTL 126],
        [Laptop4], [Server DNS 4], [Ping 192.168.140.3], [Success (4/4)], [Nirkabel ke Subnet 140],
        [PC0], [Server DNS 1], [nslookup contoh.com], [Resolved], [A Record 192.168.122.4],
        [PC0], [Server DNS 1], [http://contoh.com], [200 OK], [Render HTML Sukses],
        [Laptop4], [Server DNS 4], [http://192.168.140.3], [200 OK], [Akses Web Nirkabel Cross-Router]
      )
    )
  ]
]
