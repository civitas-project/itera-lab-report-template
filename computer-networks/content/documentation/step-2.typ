#import "../../template/components.typ": step-figure, topic

== Langkah 2 - Konfigurasi Antarmuka Router dan Routing Statis Antar-Subnet

#topic[A. Konfigurasi Antarmuka Fisik Router Layer 3][
  Router berfungsi sebagai pembatas domain siaran (_broadcast domain_) sekaligus pengarah paket (_packet forwarder_) antar-jaringan yang berbeda subnet. Kedua router (`Router0` dan `Router1`) dikonfigurasi melalui Command Line Interface (CLI) Cisco IOS.

  + *Konfigurasi Antarmuka pada Router0* \
    Antarmuka `FastEthernet0/0` terhubung ke `Switch2` (Subnet 122), antarmuka `FastEthernet1/0` terhubung ke `Switch0` (Subnet 200), dan antarmuka `FastEthernet4/0` terhubung ke `Router1` sebagai tautan transmisi data antar-router (_backbone link_):
    ```text
    Router0> enable
    Router0# configure terminal
    Router0(config)# interface FastEthernet0/0
    Router0(config-if)# ip address 192.168.122.1 255.255.255.0
    Router0(config-if)# no shutdown
    Router0(config-if)# exit

    Router0(config)# interface FastEthernet1/0
    Router0(config-if)# ip address 192.168.200.1 255.255.255.0
    Router0(config-if)# no shutdown
    Router0(config-if)# exit

    Router0(config)# interface FastEthernet4/0
    Router0(config-if)# ip address 192.168.10.1 255.255.255.0
    Router0(config-if)# no shutdown
    Router0(config-if)# exit
    ```

  + *Konfigurasi Antarmuka pada Router1* \
    Antarmuka `FastEthernet0/0` melayani `Switch3` (Subnet 140), antarmuka `FastEthernet1/0` melayani `Switch1` (Subnet NIM 75), dan antarmuka `FastEthernet4/0` menjadi ujung tautan menuju `Router0`:
    ```text
    Router1> enable
    Router1# configure terminal
    Router1(config)# interface FastEthernet0/0
    Router1(config-if)# ip address 192.168.140.1 255.255.255.0
    Router1(config-if)# no shutdown
    Router1(config-if)# exit

    Router1(config)# interface FastEthernet1/0
    Router1(config-if)# ip address 192.168.75.1 255.255.255.0
    Router1(config-if)# no shutdown
    Router1(config-if)# exit

    Router1(config)# interface FastEthernet4/0
    Router1(config-if)# ip address 192.168.10.2 255.255.255.0
    Router1(config-if)# no shutdown
    Router1(config-if)# exit
    ```
]

#topic[B. Konfigurasi Routing Statis Antar-Router][
  Tanpa adanya rute statis atau protokol routing dinamis, sebuah router hanya memiliki pengetahuan tentang subnet yang terhubung langsung (_directly connected networks_). Oleh sebab itu, agar host pada sisi `Router0` dapat menjangkau host pada sisi `Router1`, entri routing statis wajib ditambahkan ke dalam tabel rute dengan sintaks `ip route [tujuan jaringan] [subnet mask] [alamat IP next-hop]`:

  + *Penambahan Rute Statis pada Router0* \
    Untuk menjangkau Subnet 140 dan Subnet NIM 75 yang berada di balik `Router1`, `Router0` mengarahkan trafik melalui alamat IP antarmuka tetangga (_next-hop_) yaitu `192.168.10.2`:
    ```text
    Router0(config)# ip route 192.168.140.0 255.255.255.0 192.168.10.2
    Router0(config)# ip route 192.168.75.0 255.255.255.0 192.168.10.2
    Router0(config)# end
    Router0# write memory
    ```

  + *Penambahan Rute Statis pada Router1* \
    Sebaliknya, untuk mengembalikan paket data menuju Subnet 122 dan Subnet 200, `Router1` mendefinisikan _next-hop_ menuju alamat `192.168.10.1` (`Router0`):
    ```text
    Router1(config)# ip route 192.168.122.0 255.255.255.0 192.168.10.1
    Router1(config)# ip route 192.168.200.0 255.255.255.0 192.168.10.1
    Router1(config)# end
    Router1# write memory
    ```
]

#topic[C. Inspeksi dan Analisis Tabel Rute (Routing Table)][
  Verifikasi rute yang telah terpasang diperiksa menggunakan perintah `show ip route` pada mode _privileged EXEC_ atau melalui alat inspeksi (_Inspect Tool_ / kaca pembesar) pada antarmuka grafis Cisco Packet Tracer:

  #step-figure("/assets/images/routing-table-inspect.png", [Inspeksi Tabel Routing Statis pada Antarmuka CLI Router0 dan Router1], width: 35%)

  *Karakteristik Teknis Tabel Routing*
  - *Kode C (Connected)* \
    Menunjukkan jaringan lokal yang terhubung secara fisik pada port router. Memiliki _Administrative Distance_ (AD) bernilai `0`, menandakan tingkat kepercayaan tertinggi (_most trusted_).
  - *Kode S (Static)* \
    Menunjukkan rute statis yang didefinisikan secara manual oleh administrator. Memiliki nilai AD baku `1` dan metrik hop `0` (`[1/0]`), yang berarti rute tersebut akan selalu diprioritaskan sebelum protokol routing dinamis seperti OSPF (AD 110) atau RIP (AD 120).
  - Tautan interkoneksi `192.168.10.0/24` yang berstatus _up/up_ menjamin keberadaan rute statis di dalam tabel routing aktif, karena Cisco IOS secara otomatis menghapus rute statis apabila antarmuka _next-hop_ mengalami kegagalan transmisi (_link down_).
]
