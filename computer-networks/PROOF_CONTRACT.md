# Proof Contract - Computer Networks - Modul 5

## Identitas
- **Kursus**<br>
computer-networks
- **Modul/sesi**<br>
5
- **Sumber PDF**<br>
PDF modul praktikum Computer Networks sesi 5 (contoh, bukan path inventif)
- **Diekstrak dari**<br>
contoh laprak/Simulasi Cisco ...

## Aturan kolom
- **Jenis bukti**<br>
Hanya `cli-transcript`, `dbms-result`, `program-console`, `gui-state`. `gui-state` tidak boleh berdiri sendiri untuk konektivitas/routing; harus dipasangkan dengan ID `cli-transcript`.
- **Perintah/Aksi**<br>
Harus sebenarnya dieksekusi pada host, NIM sudah disubstitusi; baris banyak dipisahkan `<br>`; dilarang "lihat modul" atau perintah buatan.
- **Lokasi**<br>
Path relatif terhadap root proyek; screenshot `assets/images/<nama>.png` (dalam Typst: `/assets/images/<nama>.png`); transkrip inline = berkas langkah B5 `content/documentation/step-N.typ`; satu ID = satu artefak.
- **Kriteria lulus**<br>
Terukur, harus mengandung token yang diharapkan dan larangan spesifik, bukan "output benar".

## Daftar bukti
| ID | Langkah/Tugas | Jenis bukti | Perintah/Aksi | Lokasi file | Kriteria lulus |
|---|---|---|---|---|---|
| P-01 | Verifikasi routing OSPF | cli-transcript | `show ip route`<br>`show ip ospf neighbor` | assets/images/verifikasi-ospf-ping.png | Memuat `Codes: L - local` dan baris `Neighbor ID`; kanvas topologi saja tidak valid. |
| P-02 | Konektivitas lintas subnet | cli-transcript | `ping <alamat tujuan sesuai modul>` | assets/images/ping-inter-vlan.png | Memuat `Reply from ...` dan `Ping statistics`; RTO pada paket pertama dibolehkan jika resolusi ARP belum selesai. |
| P-03 | Alokasi DHCP | cli-transcript | `show ip dhcp binding` | assets/images/dhcp-server-config.png | Memuat alamat IP hasil alokasi sesuai NIM modul; bukan `xxxxxxxxx` atau placeholder. |

## Modul lain
Duplikasi blok tabel ini untuk modul lain dan isi dari PDF modul tersebut pada fase 2.

Catatan akhir: Perintah dijalankan ulang pada host sebenarnya sebelum laporan final; nilai turunan NIM disubstitusi dari `profile.toml`.
