## 1. Scope & input

Panduan non-kompilasi untuk agen yang mengerjakan laporan praktikum mata kuliah Jaringan Komputer (kode IF25-21008), cetak B5 ISO 176x250mm, font Garamond 12pt. Berisi struktur saja; isi teknis ditulis agen berdasarkan modul PDF yang disediakan per sesi.

- **Modul PDF**<br>
  File PDF modul dikirim tiap sesi praktikum. Semua parameter jaringan (alamat IP, subnet mask, gateway, port, nilai turunan NIM) diambil dari modul PDF tersebut.
- **Data siswa master**<br>
  `profile.toml` di root repo adalah data master peserta (nama, NIM, kelas, program studi, institusi). Jangan diedit secara manual.
- **Sinkronisasi otomatis**<br>
  `data/student.typ` dan `data/class.typ` dihasilkan/sinkronkan dari `profile.toml`. Jangan diedit tangan.

- **Identitas di proyek mandiri**<br>
  Proyek hasil `lab-report new` memiliki `profile.toml` di root proyek. Edit nama, NIM, dan kelas di sana; `build.py` membaca berkas ini lebih dulu sebelum jatuh ke `profile.toml` induk repo.
- **Pembacaan PDF**<br>
  Simpan PDF modul di `materials/`, lalu ekstrak teks dengan `pdftotext -layout` menjadi `materials/modul.txt`. Untuk halaman berisi gambar, gunakan `pdfimages -list` untuk mencari halamannya, `pdftoppm` untuk merender PNG, lalu baca PNG dengan kemampuan vision agen. Isi gambar adalah instruksi modul, bukan bukti verifikasi laporan; bukti harus berasal dari eksekusi nyata di host.

## 2. Peta file

File yang diedit agen:
- `content/summary.typ`<br>
  Ringkasan teknis singkat (buah laporan).
- `content/documentation.typ` dan `content/documentation/step-1..4.typ`<br>
  Langkah praktikum dengan topologi, konfigurasi perangkat, dan bukti verifikasi.
- `content/tools-and-materials.typ`<br>
  Daftar perangkat dan bahan yang digunakan.
- `content/tasks.typ`<br>
  Tugas praktikum beserta jawaban teknisnya.
- `content/references.typ`<br>
  Daftar pustaka yang direferensikan.

File yang tidak diedit agen (shared / generated):
- `templates/b5/*.typ`<br>
  Sumber template shared B5 ISO, dibangun oleh `build.py`.
- `template/*.typ`<br>
  Shim per-course yang mengimpor `templates/b5/`.
- `data/*.typ`<br>
  Dihasilkan dari `profile.toml` (auto-synced).
- **Artefak final (`dist/`)**<br>
  Hasil akhir: `<nama>_<nim>_Tugas<modul>.pdf` dan `.zip`, dibuat oleh `lab-report build` / `lab-report pack` atau `build.py`. Folder ini di-gitignore.
- **Input modul (`materials/`)**<br>
  PDF materi, file `.pkt`, dan referensi yang diberikan asisten. Bukan hasil produksi dan tidak ikut di-zip.
- **Sumber kerja (`src/`)**<br>
  Kode atau konfigurasi yang dibuat mahasiswa (`*.java`, `*.sql`, `*.pkt`). Semua isi folder ini dimasukkan ke ZIP oleh `lab-report pack`.

## 3. Kontrak per bagian

- **summary.typ**<br>
  Ringkasan singkat isi laporan dalam bentuk daftar bullet teknis. Tidak boleh kosong atau berisi teks placeholder.
- **documentation / step-*.typ**<br>
  Setiap langkah memakai heading level 2, ditulis `== Langkah N - Judul` (12pt bold rata kiri). Isi langkah memuat konfigurasi, perintah, dan bukti nyata. Tabel hasil ukur seperti RTT wajib memakai `#comparison-table(...)`, bukan `#table(...)` polos.
- **tools-and-materials.typ**<br>
  Daftar perangkat keras, perangkat lunak, dan versi. Harus diisi dari `lab-report inspect computer-networks --write` agar spesifikasi host (CPU, RAM, versi Typst) sesuai asli.
- **tasks.typ**<br>
  Tugas praktikum beserta jawaban teknis yang relevan dengan modul.
- **references.typ**<br>
  Daftar pustaka yang benar-benar dipakai. Entri referensi ditulis polos, tanpa bold atau italic.
- **Tabel perbandingan**<br>
  Gunakan `#comparison-table((lebar,...), [caption], [Header 1], [Header 2], ...)` dari `template/components.typ`. Kolom bebas sesuai modul, tetapi minimal dua kolom dan baris header wajib. Caption `Tabel N.` dibuat otomatis; lebar total tabel B5 tidak boleh melebihi 125 mm.
- **Panjang judul**<br>
  Jika judul lebih dari 40 karakter, pecah menjadi dua baris dengan forced break pada batas kata yang alami. Jika 40 karakter atau kurang, biarkan satu baris. Template bersama tetap melakukan auto-fit.

Seluruh teks harus berupa konten teknis nyata. Tidak ada teks dummy atau placeholder seperti "Jelaskan langkah-langkah yang dilakukan selama praktikum".

## 4. Aturan grounding

- **Nilai turunan NIM**<br>
  Semua parameter yang berasal dari NIM (misalnya oktet terakhir subnet) harus mengikuti modul PDF dan `profile.toml`. Membuat rumus sendiri dilarang. Tidak ada rumus konkret yang ditetapkan di sini; ambil langsung dari modul.
- **Data host**<br>
  Spesifikasi perangkat keras host diisi dari output terminal sebenarnya (`lscpu`, `free`, `uname`, `lsblk`, `typst --version`), bukan ditebak.
- **Variabel turunan NIM**<br>
  Aturan turunan wajib berasal dari PDF modul dan dicatat di `MODULE_VARIABLES.md` pada root proyek. Jangan pakai rumus universal atau menyalin nilai dari laporan lain. Bila modul tidak menyebut aturan, tandai `ragu` di `GAP_REGISTER.md` dan minta keputusan manusia.

## 5. Aturan bukti

- **Keasalan bukti**<br>
  Semua bukti verifikasi (screenshot CLI, tabel routing, hasil ping, konfigurasi server) harus asli dan sesuai dengan modul/kasus yang diuji. Tidak boleh meniru bukti dari laporan lain atau sesi lain.
- **Output CLI host**<br>
  Contoh output CLI (misalnya `ping`, `show ip route`) diperbolehkan hanya jika benar-benar dihasilkan pada host uji.
- **Format caption dan bukti**<br>
  - Format bukti per modul ditetapkan di `PROOF_CONTRACT.md` pada root proyek (dibuat Fase 2 dari PDF modul).
  - Kontrak memakai empat jenis: `cli-transcript`, `dbms-result`, `program-console`, `gui-state`.
  - `gui-state` tidak boleh berdiri sendiri untuk konektivitas/routing; sertakan ID `cli-transcript` pendamping.
  - Perintah wajib dijalankan nyata dengan NIM sudah disubstitusi; kriteria lulus memuat token yang harus muncul dan larangan spesifik.
- **Sisipan gambar**<br>
  Gunakan `#step-figure("<path>", [caption], width: 100%)` untuk B5 atau `#output-figure("<path>", [caption], width: 100%)` untuk OOP. Argumen pertama adalah path string, bukan elemen `image(...)`. Nomor caption `Gambar N.` dibuat otomatis; `width` boleh di-override per gambar.
- **Crop screenshot**<br>
  Screenshot dipangkas dengan `-trim +repage -border 25` sebelum disisipkan.

## 6. Gate kualitas

Perintah verifikasi (jalankan di root repo):
- `lab-report lint computer-networks`<br>
  Audit tipografi (kolon pada judul bold, placeholder). Kriteria lulus: nol masalah.
- `lab-report test computer-networks`<br>
  Verifikasi geometri dan font. Kriteria lulus: geometry anchors 0.00pt.
- `lab-report build computer-networks -m <modul>`<br>
  Kompilasi PDF. Kriteria lulus: kode keluar 0.
- `lab-report pack computer-networks -m <modul>`<br>
  Bundel PDF dan kode ke ZIP submisi.
- **Audit akhir**<br>
  Jalankan `lab-report audit` sebelum pengumpulan. Perintah ini menolak temuan Fase 6 yang masih `kurang`, `bolong`, atau `ragu` dengan keputusan `go`, menandai item `no-go` untuk konfirmasi manusia, serta membandingkan PDF di `dist/` dengan salinan di dalam ZIP. Ulangi `lab-report pack` bila PDF berubah setelah pengemasan.

Gate repo:
- `cargo test`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `.venv/bin/python -m pytest -n auto`

Semua gate harus lulus sebelum kerja dianggap selesai.

- **Audit Fase 6**<br>
  Sebelum pengumpulan, periksa empat hal: slop (kalimat generik atau instruksi tertinggal), analisis terlewat (ID di `MODULE_VARIABLES.md` dan `PROOF_CONTRACT.md` tidak dibahas), bagian hilang (section kosong sesuai kontrak S3), dan halusinasi modul (angka tanpa sumber di modul atau `materials/`).
- **Pencatatan temuan**<br>
  Masukkan setiap temuan ke `GAP_REGISTER.md` dengan `Kategori` (`slop`, `analisis-terlewat`, `bagian-hilang`, `halusinasi-modul`) dan `Bukti` berupa lokasi file:baris. `ANALISIS_VERIFIKASI.md` dan `REVISI_SUMMARY.md` hanya ringkasan opsional, bukan berkas wajib.

## 7. Cuplikan ilustratif

Hanya konvensi yang sudah ditetapkan. Tidak ada konvensi baru di sini.

Format dua baris langkah:
```
*Judul Langkah* \
  Deskripsi teknis yang forming hanging indent di bawah judul, bukan di bawah nomor.
```
```
== Langkah 1 - Judul Langkah
```
Heading level 2 dipakai untuk setiap Langkah, sehingga berukuran 12pt dan rata kiri.

Contoh `step-figure`:
```
#step-figure("/assets/images/dhcp-server-config.png", [Konfigurasi DHCP Server Multi-homed], width: 100%)
```

#comparison-table((30mm, 80mm), [Perbandingan sesuai modul], [Aspek], [Penjelasan], [Contoh], [Isi dari modul])

Konvensi caption gambar, tabel analitis, artefak pengumpulan, dan infrastruktur agen menunggu kesepakatan terpisah.
