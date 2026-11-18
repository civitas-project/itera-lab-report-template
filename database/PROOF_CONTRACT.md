# Proof Contract - Database Systems - Modul DDL

## Identitas
- **Mata kuliah**<br>
  database
- **Modul contoh**<br>
  DDL Tingkat Lanjut (Modul 5)
- **Sumber PDF**<br>
  PDF modul praktikum Database Systems
- **Dasar contoh**<br>
  `contoh laprak/SQL - DDL Tingkat Lanjut`

## Aturan kolom
- **Jenis bukti**<br>
  Hanya `cli-transcript`, `dbms-result`, `program-console`, `gui-state`. `gui-state` tidak boleh berdiri sendiri sebagai bukti eksekusi SQL; harus dipasangkan dengan ID `dbms-result`.
- **Perintah/Aksi**<br>
  Sebenarnya dieksekusi pada host, NIM disubstitusi, gunakan `<br>` untuk beberapa baris, jangan mencipta perintah.
- **Lokasi**<br>
  Relatif terhadap root proyek; screenshot `assets/images/<nama>.png` (Typst: `/assets/images/<nama>.png`); transkrip inline B5 di `content/documentation/step-N.typ`; satu ID = satu artefak.
- **Kriteria lulus**<br>
  Terukur, mencakup token yang diharapkan dan larangan spesifik.

## Daftar bukti
| ID | Langkah/Tugas | Jenis bukti | Perintah/Aksi | Lokasi file | Kriteria lulus |
|---|---|---|---|---|---|
| P-01 | Struktur tabel hasil DDL | `dbms-result` | `SHOW CREATE TABLE mahasiswa;` | `content/documentation/step-1.typ` | Memuat `CREATE TABLE` dan `PRIMARY KEY`; field/NIM mengikuti modul, bukan placeholder. |
| P-02 | Simulasi pelanggaran foreign key | `dbms-result` | `INSERT INTO ...` (nilai FK tidak ada di tabel induk, sesuai modul) | `content/documentation/step-2.typ` | Memuat kode error `1451` atau `1452`; bukan pesan sukses palsu. |
| P-03 | Katalog constraint | `dbms-result` | `SELECT CONSTRAINT_NAME, CONSTRAINT_TYPE FROM information_schema.TABLE_CONSTRAINTS WHERE TABLE_SCHEMA = '<nama skema>';` | `content/documentation/step-3.typ` | Memuat nama constraint dan tipe `FOREIGN KEY` / `UNIQUE` / `PRIMARY KEY` yang sesuai modul. |

## Modul lain
| ID | Langkah/Tugas | Jenis bukti | Perintah/Aksi | Lokasi file | Kriteria lulus |
|---|---|---|---|---|---|

Diisi dari PDF modul masing-masing pada fase 2.

Catatan akhir: Perintah dijalankan ulang pada DBMS host sebenarnya; nama skema, NIM, dan nilai lain disubstitusi dari `profile.toml` dan modul.
