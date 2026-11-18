# Proof Contract - Object-Oriented Programming - Modul 5 Polymorphism

## Identitas
- **Course**<br>
object-oriented-programming
- **Modul contoh**<br>
Polymorphism
- **Sumber PDF**<br>
PDF modul praktikum PBO
- **Turunan dari contoh**<br>
contoh laprak/Polymorphism (Polimorfisme)

## Aturan kolom
- **Jenis bukti**<br>
Hanya `cli-transcript`, `dbms-result`, `program-console`, `gui-state`. `gui-state` tidak boleh berdiri sendiri sebagai bukti eksekusi program; harus dipasangkan dengan ID `program-console`.
- **Perintah/Aksi**<br>
Sebenarnya dieksekusi pada host, NIM disesuaikan, `<br>` untuk beberapa baris, tanpa perintah yang dibuat.
- **Lokasi**<br>
Relatif terhadap akar proyek; screenshot `assets/images/<nama>.png`; path Typst adalah `/assets/images/<nama>.png`; transkrip inline untuk OOP = berkas bernomor sebenarnya seperti `content/03-langkah.typ` atau `content/05-output.typ` (bukan `content/documentation/step-N.typ`, milik kursus B5); satu ID = satu artefak.
- **Kriteria lulus**<br>
Terukur, dengan token yang diharapkan dan larangan spesifik.

## Daftar bukti

| ID | Langkah/Tugas | Jenis bukti | Perintah/Aksi | Lokasi file | Kriteria lulus |
|----|---------------|-------------|---------------|-------------|----------------|
| P-01 | Overriding dasar | program-console | `javac "Percobaan 1.java"`<br>`java Percobaan1` | `assets/images/output-percobaan1.png` | Exit 0; output menampilkan suara masing-masing subclass; tanpa stack trace. |
| P-02 | Array polimorfis | program-console | `javac "Percobaan 2.java"`<br>`java Percobaan2` | `assets/images/output-percobaan2.png` | Memuat output untuk seluruh elemen array; tanpa `ClassCastException`. |
| P-03 | Overloading vs overriding | program-console | `javac "Percobaan 4.java"`<br>`java Percobaan4` | `content/05-output.typ` | Memuat hasil overload `tambah(int,int)`, `tambah(int,int,int)`, dan `tambah(double,double)`; tanpa error kompilasi. |

## Modul lain

Duplikat tabel per modul, diisi dari PDF modul tersebut pada fase 2.

Kompilasi dan eksekusi dijalankan ulang pada host sebenarnya; nama kelas dan paket mengikuti modul serta NIM dari `profile.toml`.
