## 1. Scope & input

- **Sumber kebenaran**<br>
  Modul PDF per sesi adalah referensi utama untuk studi kasus, nama paket, dan nama kelas. Setiap sesi modul memiliki PDF sendiri di folder mata kuliah.
- **Metadata master**<br>
  `profile.toml` di root adalah master nama, NIM, kelas, dan program studi. `data/student.typ` di sinkronkan otomatis dari `profile.toml` melalui `lab-report profile`; jangan edit manual. OOP menggunakan nama field `nama` dan `nim`.
- **Cakupan file**<br>
  Agent mengedit `content/01-tujuan.typ` sampai `content/07-kesimpulan.typ` dan `src/*.java` (saat ini stub). `main.typ` include semua tujuh bagian berurutan dari `01-tujuan.typ` hingga `07-kesimpulan.typ`. `build.py` dan `tests/` mengelola build dan uji template.

- **Identitas di proyek mandiri**<br>
  Proyek hasil `lab-report new` memiliki `profile.toml` di root proyek. Edit nama, NIM, dan kelas di sana; `build.py` membaca berkas ini lebih dulu sebelum jatuh ke `profile.toml` induk repo.
- **Pembacaan PDF**<br>
  Simpan PDF modul di `materials/`, lalu ekstrak teks dengan `pdftotext -layout` menjadi `materials/modul.txt`. Untuk halaman berisi gambar, gunakan `pdfimages -list` untuk mencari halamannya, `pdftoppm` untuk merender PNG, lalu baca PNG dengan kemampuan vision agen. Isi gambar adalah instruksi modul, bukan bukti verifikasi laporan; bukti harus berasal dari eksekusi nyata di host.

## 2. Peta file

- **Dikonfigurasi agent**<br>
  `content/01-tujuan.typ`, `content/02-alat-bahan.typ`, `content/03-langkah.typ`, `content/04-source-code.typ`, `content/05-output.typ`, `content/06-analisis.typ`, `content/07-kesimpulan.typ`; `src/Percobaan 1.java` sampai `src/Percobaan 4.java`.
- **Dikelola CLI/template**<br>
  `template/*` (cover, components, header-footer, lib) dan `data/*` (report, class, assistants, student) dikendalikan oleh `lab-report`; jangan dirubah tangan.
- **Aset dan output**<br>
  `assets/images/` untuk tangkapan layar output program. `dist/` dan hasil build adalah artefak yang di-gitignore.
- **Artefak final (`dist/`)**<br>
  Hasil akhir: `<nama>_<nim>_Tugas<modul>.pdf` dan `.zip`, dibuat oleh `lab-report build` / `lab-report pack` atau `build.py`. Folder ini di-gitignore.
- **Input modul (`materials/`)**<br>
  PDF materi, file `.pkt`, dan referensi yang diberikan asisten. Bukan hasil produksi dan tidak ikut di-zip.
- **Sumber kerja (`src/`)**<br>
  Kode atau konfigurasi yang dibuat mahasiswa (`*.java`, `*.sql`, `*.pkt`). Semua isi folder ini dimasukkan ke ZIP oleh `lab-report pack`.

## 3. Kontrak per bagian

- **01-tujuan**<br>
  Hasil belajar konkret yang dicapai mahasiswa setelah modul. Tanpa teks instruksi.
- **02-alat-bahan**<br>
  Diisi dari output `lab-report inspect object-oriented-programming --write` (spesifikasi host sebenarnya: CPU, memori, OS, versi Typst). Jangan perangkat keras fiktif.
- **documentation / step-*.typ**<br>
  Setiap langkah memakai heading level 2, ditulis `== Langkah N - Judul` (12pt bold rata kiri). Isi langkah memuat konfigurasi, perintah, dan bukti nyata. Tabel hasil ukur seperti RTT wajib memakai `#comparison-table(...)`, bukan `#table(...)` polos.
- **04-source-code**<br>
  Kode Java lengkap yang benar-benar dapat dikompilasi per percobaan, nama paket dan kelas sesuai modul PDF.
- **05-output**<br>
  Output sebenarnya dari menjalankan program, bukan contoh buatan.
- **06-analisis**<br>
  Diskusi konseptual yang ditautkan langsung ke studi kasus modul tersebut.
- **07-kesimpulan**<br>
  Kesimpulan teknis dari percobaan. Tidak ada teks instruksi di output akhir.
- **references.typ**<br>
  Daftar pustaka yang benar-benar dipakai. Entri referensi ditulis polos, tanpa bold atau italic.
- **Tabel perbandingan**<br>
  Gunakan `#comparison-table((lebar,...), [caption], [Header 1], [Header 2], ...)` dari `template/components.typ`. Kolom bebas sesuai modul, tetapi minimal dua kolom dan baris header wajib. Caption `Tabel N.` dibuat otomatis; lebar total tabel B5 tidak boleh melebihi 125 mm.
- **Panjang judul**<br>
  Jika judul lebih dari 40 karakter, pecah menjadi dua baris dengan forced break pada batas kata yang alami. Jika 40 karakter atau kurang, biarkan satu baris. Template bersama tetap melakukan auto-fit.

## 4. Aturan grounding

- Nama paket, nama kelas, dan NIM mengikuti modul PDF dan `profile.toml`; tidak ada rumus NIM yang diimprovisasi.
- Spesifikasi hardware di `02-alat-bahan` harus berasal dari terminal host (`lscpu`, `free`, `uname`, `lsblk`, `typst --version`).
- Jawaban harus cocok dengan studi kasus modul, bukan deskripsi generik OOP.
- **Variabel turunan NIM**<br>
  Aturan turunan wajib berasal dari PDF modul dan dicatat di `MODULE_VARIABLES.md` pada root proyek. Jangan pakai rumus universal atau menyalin nilai dari laporan lain. Bila modul tidak menyebut aturan, tandai `ragu` di `GAP_REGISTER.md` dan minta keputusan manusia.

## 5. Aturan bukti

- Bukti harus berupa output kompilasi dan eksekusi Java autentik yang cocok dengan studi kasus modul.
- Format bukti per modul ditetapkan di `PROOF_CONTRACT.md` pada root proyek (dibuat Fase 2 dari PDF modul).
- Kontrak memakai empat jenis: `cli-transcript`, `dbms-result`, `program-console`, `gui-state`.
- Bukti GUI tidak boleh berdiri sendiri sebagai bukti eksekusi program; sertakan ID `program-console` pendamping.
- Perintah wajib dijalankan nyata dengan NIM sudah disubstitusi; kriteria lulus memuat token yang harus muncul dan larangan spesifik.
- **Sisipan gambar**<br>
  Gunakan `#step-figure("<path>", [caption], width: 100%)` untuk B5 atau `#output-figure("<path>", [caption], width: 100%)` untuk OOP. Argumen pertama adalah path string, bukan elemen `image(...)`. Nomor caption `Gambar N.` dibuat otomatis; `width` boleh di-override per gambar.
- **Crop screenshot**<br>
  Screenshot dipangkas dengan `-trim +repage -border 25` sebelum disisipkan.

## 6. Gate kualitas

- `lab-report lint object-oriented-programming`
- `lab-report test object-oriented-programming`
- `lab-report build object-oriented-programming -m <modul>`
- `lab-report pack object-oriented-programming -m <modul>`
- **Audit akhir**<br>
  Jalankan `lab-report audit` sebelum pengumpulan. Perintah ini menolak temuan Fase 6 yang masih `kurang`, `bolong`, atau `ragu` dengan keputusan `go`, menandai item `no-go` untuk konfirmasi manusia, serta membandingkan PDF di `dist/` dengan salinan di dalam ZIP. Ulangi `lab-report pack` bila PDF berubah setelah pengemasan.
- `cargo test`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `.venv/bin/python -m pytest -n auto`
- Catatan: gate tipografi resmi saat ini hanya mencakup computer-networks dan database. Placeholder OOP adalah konten starter yang sudah ada dan harus diganti dengan konten asli sebelum lint dianggap lulus.

- **Audit Fase 6**<br>
  Sebelum pengumpulan, periksa empat hal: slop (kalimat generik atau instruksi tertinggal), analisis terlewat (ID di `MODULE_VARIABLES.md` dan `PROOF_CONTRACT.md` tidak dibahas), bagian hilang (section kosong sesuai kontrak S3), dan halusinasi modul (angka tanpa sumber di modul atau `materials/`).
- **Pencatatan temuan**<br>
  Masukkan setiap temuan ke `GAP_REGISTER.md` dengan `Kategori` (`slop`, `analisis-terlewat`, `bagian-hilang`, `halusinasi-modul`) dan `Bukti` berupa lokasi file:baris. `ANALISIS_VERIFIKASI.md` dan `REVISI_SUMMARY.md` hanya ringkasan opsional, bukan berkas wajib.

## 7. Cuplikan ilustratif

- Format langkah dua baris:
  ```
  + *Judul Langkah* \
    Penjelasan teknis yang menjorok sejajar di bawah judul.
  ```
  ```
  == Langkah 1 - Judul Langkah
  ```
  Heading level 2 dipakai untuk setiap Langkah, sehingga berukuran 12pt dan rata kiri.
- Blok kode plain, tanpa konvensi baru.

#comparison-table((30mm, 80mm), [Perbandingan sesuai modul], [Aspek], [Penjelasan], [Contoh], [Isi dari modul])

Konvensi caption gambar, tabel analitis, artefak pengumpulan, dan infrastruktur agen menunggu kesepakatan terpisah.
