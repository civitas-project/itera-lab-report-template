#import "../../template/components.typ": step-figure, topic

== Langkah 4 - Konfigurasi Layanan DNS, HTTP Server, dan Verifikasi End-to-End

#topic[A. Konfigurasi Layanan Domain Name System (DNS)][
  Domain Name System (DNS) beroperasi pada _Application Layer_ (Layer 7) model OSI dan bertanggung jawab menerjemahkan nama domain (_Fully Qualified Domain Name_ / FQDN) yang mudah diingat manusia menjadi alamat IP numerik Layer 3 (RFC 1034 dan RFC 1035).

  Konfigurasi Server DNS dilakukan pada `Server DNS 1` (Subnet 122) dan direplikasi pada server DNS lainnya:
  + *Aktivasi Layanan DNS* \
    Pada tab _Services > DNS_, opsi _DNS Service_ dinyalakan (_On_).
  + *Pembuatan Resource Record Tipe A*
     - *Name* \
       Ditetapkan nama domain pengujian, yakni `contoh.com`.
     - *Type* \
       Dipilih _A Record_ (_Address Record_ untuk IPv4).
     - *Address* \
       Diisi dengan alamat IP server web lokal tujuan, yakni `192.168.122.4`.
     - Tombol _Add_ diklik untuk menyimpan rekaman ke dalam basis data DNS server.
  + *Sinkronisasi Parameter DNS pada Server DHCP* \
    Alamat IP server DNS (`192.168.122.4` untuk Subnet 122, `192.168.200.4` untuk Subnet 200, `192.168.75.4` untuk Subnet NIM 75, dan `192.168.140.3` untuk Subnet 140) dikonfigurasi pada parameter _DNS Server_ pada DHCP Pool masing-masing. Dengan demikian, ketika klien melakukan pembaruan sewa (_lease renewal_), klien secara otomatis memperoleh informasi server resolusi nama yang tepat.

  #step-figure("/assets/images/dns-server-config.png", [Konfigurasi DNS Service A-Record contoh.com dan Verifikasi nslookup], width: 35%)

  *Verifikasi Resolusi Nama Domain (nslookup)* \
  Pengujian resolusi nama dilakukan dari Command Prompt klien `PC0` menggunakan perintah `nslookup contoh.com`. Server DNS `192.168.122.4` berhasil merespons dengan jawaban _non-authoritative_ yang memetakan nama domain `contoh.com` secara akurat ke alamat IP `192.168.122.4`.
]

#topic[B. Konfigurasi Web Server HTTP dan Pengujian Web Browser][
  Hypertext Transfer Protocol (HTTP) merupakan protokol pertukaran berkas berbasis _request-response_ yang berjalan di atas protokol transport TCP pada port 80 (RFC 2616). 

  Layanan HTTP dikonfigurasi pada tab _Services > HTTP_:
  - Opsi _HTTP_ dan _HTTPS_ dinyalakan (_On_).
  - Berkas `index.html` disunting untuk menyajikan laman sambutan resmi Cisco Packet Tracer dengan struktur hyperlink standar: _A small page_, _Copyrights_, dan _Image page_.

  *Pengujian Akses Web melalui Web Browser*
  + *Akses Berbasis Nama Domain* \
    Pada aplikasi _Web Browser_ di `PC0`, URL `http://contoh.com` dimasukkan. Perangkat mula-mula mengirimkan kueri DNS ke `192.168.122.4` melalui UDP port 53. Setelah memperoleh balasan IP `192.168.122.4`, browser membangun jabat tangan 3 langkah (_TCP 3-Way Handshake_) pada port 80, lalu mengirimkan permintaan `GET / HTTP/1.1`. Laman web Cisco Packet Tracer berhasil dirender secara sempurna.
  + *Akses Lintas-Router secara Nirkabel* \
    Dari `Laptop4` (klien nirkabel pada Subnet 200), URL `http://192.168.140.3` diakses. Paket HTTP melintasi `AccessPoint0`, `Switch0`, ditransmisikan menyeberangi `Router0` dan `Router1` melalui tautan statis, hingga mencapai `Server DNS 4` pada Subnet 140. Server merespons dengan kode status `HTTP/1.1 200 OK` (ukuran respons 104 byte).

  #step-figure("/assets/images/browser-access-test.png", [Pengujian Akses Layanan Web HTTP melalui Web Browser PC0 dan Laptop4], width: 35%)
]

#topic[C. Uji Konektivitas End-to-End melalui Protokol ICMP (Ping)][
  Pengujian konektivitas menyeluruh dilakukan menggunakan utilitas `ping` (_Internet Control Message Protocol_ / ICMP Echo Request dan Echo Reply sesuai RFC 792) untuk memvalidasi fungsi switching Layer 2, routing Layer 3, dan jembatan nirkabel:

  #step-figure("/assets/images/ping-inter-vlan.png", [Uji Konektivitas ICMP Ping End-to-End Lintas Subnet dan Antar Router], width: 35%)

  *Analisis Hasil Uji Ping*
  - *PC0 $arrow.r$ Server DNS 1 (`192.168.122.4` - Subnet Lokal)* \
    Pengujian menghasilkan 4 paket terkirim dan 4 paket diterima ($0\%$ _loss_) dengan waktu latensi $<1" ms"$ dan nilai _Time-to-Live_ (TTL) bernilai `128`. Nilai TTL yang tidak berkurang menunjukkan transmisi berlangsung murni pada Layer 2 melalui `Switch2` tanpa melewati hop router.
  - *PC0 $arrow.r$ PC3 (`192.168.75.3` - Subnet NIM 75 Lintas-Router)* \
    Paket melintasi `Router0` (hop 1) dan `Router1` (hop 2). Paket pertama mengalami sedikit penundaan akibat resolusi ARP antar-router, diikuti 3 paket balasan dengan latensi stabil $10" ms"$. Nilai TTL pada paket balasan teramati bernilai `126`. Penurunan nilai TTL dari `128` menjadi `126` membuktikan secara empiris bahwa paket melewati tepat 2 hop router ($128 - 2 = 126$).
  - *Laptop4 $arrow.r$ Server DNS 4 (`192.168.140.3` - Lintas Nirkabel dan Antar-Router)* \
    Laptop4 yang terhubung nirkabel melalui Access Point berhasil menjangkau Server DNS 4 di Subnet 140 dengan latensi rata-rata $17" ms"$ dan nilai TTL `126`. Hal ini membuktikan integrasi penuh antara segmen nirkabel, domain switching lokal, dan perutean statis Layer 3.
]
