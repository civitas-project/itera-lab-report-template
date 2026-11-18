#import "../../template/components.typ": step-figure, topic

== Langkah 3 - Perluasan Jaringan Nirkabel melalui Access Point (IEEE 802.11 / WPA2-PSK)

#topic[A. Prinsip Kerja Layer 2 Bridging pada Access Point][
  Access Point (AP) adalah perangkat jaringan yang beroperasi pada _Data Link Layer_ (Layer 2) model OSI dan bertindak sebagai jembatan transparan (_transparent bridge_) yang mengonversi frame kabel _IEEE 802.3 Ethernet_ menjadi frame gelombang radio nirkabel _IEEE 802.11 Wi-Fi_, dan sebaliknya. 

  Pada topologi percobaan:
  - Antarmuka `Port 0` (kabel tembaga) dihubungkan ke `Switch0` pada port `FastEthernet5/1` menggunakan kabel _straight-through_.
  - Antarmuka `Port 1` (nirkabel) memancarkan sinyal Radio Frequency (RF) pada frekuensi 2.4 GHz yang membentuk _Basic Service Set_ (BSS).
  - Karena bekerja pada Layer 2 tanpa melakukan proses routing Layer 3, seluruh perangkat nirkabel yang terhubung ke AP berada di dalam domain siaran (_broadcast domain_) dan subnet yang sama dengan `Switch0`, yakni Subnet 200 (`192.168.200.0/24`).
]

#topic[B. Konfigurasi Parameter Nirkabel dan Keamanan WPA2-PSK][
  Pengaturan parameter nirkabel dilakukan pada tab _Config > Port 1_ perangkat `AccessPoint0` dengan rincian spesifikasi sebagai berikut:
  - *Port Status* \
    Dinyalakan (_On_).
  - *SSID (Service Set Identifier)* \
    Diatur menjadi `WIFI_ITERA_IF` (atau menggunakan SSID baku `Default`).
  - *Coverage Range* \
    140 meter, menjamin jangkauan sinyal mencakup seluruh area penempatan perangkat ujung nirkabel.
  - *Channel* \
    Saluran 1 pada frekuensi pembawa 2.412 GHz.
  - *Authentication Mode* \
    Dipilih standar _WPA2-PSK_ (_Wi-Fi Protected Access 2 Pre-Shared Key_).
  - *Encryption Algorithm* \
    _Advanced Encryption Standard_ (AES) berbasis Counter Mode Cipher Block Chaining Message Authentication Code Protocol (CCMP), menjamin kerahasiaan data dan integritas frame 802.11i.
  - *PSK Pass Phrase* \
    Diatur menjadi `itera2025`.

  #step-figure("/assets/images/access-point-config.png", [Konfigurasi Parameter Port Nirkabel Access Point dan Status Asosiasi RF], width: 35%)
]

#topic[C. Integrasi Perangkat Klien Nirkabel dan Akuisisi DHCP][
  Dua perangkat akhir, yakni `PC4` dan `Laptop4`, dikonfigurasi untuk terhubung ke jaringan lokal secara nirkabel:
  + *Penggantian Kartu Jaringan Fisik*
     - Perangkat dimatikan sesaat (_power off_).
     - Modul kartu jaringan kabel tembaga bawaan dilepas dari slot 0.
     - Modul nirkabel dipasang: modul `PT-HOST-NM-1W` untuk `PC4` dan modul `PT-LAPTOP-NM-1W` (kompatibel modul WPC300N) untuk `Laptop4`.
     - Perangkat dinyalakan kembali (_power on_).

  + *Proses Asosiasi dan Autentikasi Nirkabel*
     - Klien memancarkan _Probe Request_ untuk memindai keberadaan SSID.
     - Setelah menerima _Probe Response_ dari `AccessPoint0`, dilakukan jabat tangan 4 langkah (_4-Way Handshake_) WPA2-PSK untuk menurunkan kunci enkripsi dinamis _Pairwise Transient Key_ (PTK) dari passphrase `itera2025`.
     - Status koneksi diverifikasi berhasil terhubung (_Associated_) dengan kekuatan sinyal penuh (-42 dBm pada kecepatan 54 Mbps).

  + *Akuisisi Alamat IP melalui DHCP Subnet 200*
     - Begitu asosiasi Layer 2 terjalin, klien mengirimkan siaran DHCP Discover yang diteruskan oleh AP ke `Switch0` dan diterima oleh `Server DHCP 122 dan 200` pada antarmuka `FastEthernet1`.
     - Klien secara dinamis memperoleh konfigurasi pengalamatan IP yang valid:
        - *PC4* \
          Alamat IP `192.168.200.2`, Subnet Mask `255.255.255.0`, Gateway `192.168.200.1`, DNS Server `192.168.200.4`.
        - *Laptop4* \
          Alamat IP `192.168.200.3`, Subnet Mask `255.255.255.0`, Gateway `192.168.200.1`, DNS Server `192.168.200.4`.

  #step-figure("/assets/images/wireless-connection-pc4.png", [Verifikasi Alokasi IP Nirkabel Dinamis pada Laptop4 dan PC4 melalui Command Prompt])
]
