# Referensi Agen Praktikum Basis Data IF22-21010

## 1. Scope & input

- **Modul PDF**<br>
  Satu modul PDF per sesi praktikum sebagai referensi tunggal konten dan tugas.
- **profile.toml**<br>
  Master metadata peserta: nama, NIM, kelas, bahasa. Sumber kebenaran untuk data laporan.
- **data/student.typ dan data/class.typ**<br>
  Disinkronisasi otomatis dari profile.toml. Jangan diedit secara manual.
- **Batasan agen**<br>
  Isi konten sesuai modul dan NIM terdaftar. Jangan timpa data yang sudah ada.

- **Identitas di proyek mandiri**<br>
  Proyek hasil `lab-report new` memiliki `profile.toml` di root proyek. Edit nama, NIM, dan kelas di sana; `build.py` membaca berkas ini lebih dulu sebelum jatuh ke `profile.toml` induk repo.
- **Pembacaan PDF**<br>
  Simpan PDF modul di `materials/`, lalu ekstrak teks dengan `pdftotext -layout` menjadi `materials/modul.txt`. Untuk halaman berisi gambar, gunakan `pdfimages -list` untuk mencari halamannya, `pdftoppm` untuk merender PNG, lalu baca PNG dengan kemampuan vision agen. Isi gambar adalah instruksi modul, bukan bukti verifikasi laporan; bukti harus berasal dari eksekusi nyata di host.

## 2. Peta file

- `content/**`<br>
  Diedit oleh agen: ringkasan, langkah dokumentasi, alat dan bahan, tugas, referensi.
- `templates/b5/`<br>
  Template bersama B5 ISO 176x250mm Garamond 12pt. Tidak diubah agen.
- `template/*`<br>
  Shims lokal (cover, inner-title, components, lib). Dikelola bersama, tidak diubah agen.
- `data/*`<br>
  Dihasilkan dan disinkronisasi; artefak hasil build gitignored.
- `build.py`<br>
  Tahapan build bersama template; tidak diubah agen.
- **Artefak final (`dist/`)**<br>
  Hasil akhir: `<nama>_<nim>_Tugas<modul>.pdf` dan `.zip`, dibuat oleh `lab-report build` / `lab-report pack` atau `build.py`. Folder ini di-gitignore.
- **Input modul (`materials/`)**<br>
  PDF materi, file `.pkt`, dan referensi yang diberikan asisten. Bukan hasil produksi dan tidak ikut di-zip.
- **Sumber kerja (`src/`)**<br>
  Kode atau konfigurasi yang dibuat mahasiswa (`*.java`, `*.sql`, `*.pkt`). Semua isi folder ini dimasukkan ke ZIP oleh `lab-report pack`.

## 3. Kontrak per bagian

- `content/summary.typ`<br>
  Ringkasan materi pokok modul, bahasa Indonesia, bebas instruksi template atau teks placeholder.
- **documentation / step-*.typ**<br>
  Setiap langkah memakai heading level 2, ditulis `== Langkah N - Judul` (12pt bold rata kiri). Isi langkah memuat konfigurasi, perintah, dan bukti nyata. Tabel hasil ukur seperti RTT wajib memakai `#comparison-table(...)`, bukan `#table(...)` polos.
- `content/tools-and-materials.typ`<br>
  Diisi dari `lab-report inspect database --write`. Spesifikasi hardware asli host: lscpu, free, uname, lsblk, typst --version. Jangan disusukan.
- `content/tasks.typ`<br>
  Jawab pertanyaan aktual modul dengan SQL nyata, bukan teks instruksi.
- **references.typ**<br>
  Daftar pustaka yang benar-benar dipakai. Entri referensi ditulis polos, tanpa bold atau italic.
- Kode SQL asli, valid, dan diberi highlighting sintaks.
- **Tabel perbandingan**<br>
  Gunakan `#comparison-table((lebar,...), [caption], [Header 1], [Header 2], ...)` dari `template/components.typ`. Kolom bebas sesuai modul, tetapi minimal dua kolom dan baris header wajib. Caption `Tabel N.` dibuat otomatis; lebar total tabel B5 tidak boleh melebihi 125 mm.
- **Panjang judul**<br>
  Jika judul lebih dari 40 karakter, pecah menjadi dua baris dengan forced break pada batas kata yang alami. Jika 40 karakter atau kurang, biarkan satu baris. Template bersama tetap melakukan auto-fit.

## 4. Aturan grounding

- Nilai NIM dan parameter mengikuti PDF modul dan profile.toml. Jangan salin nilai dari contoh lain.
- Jangan buat rumus modulo/offset kecuali modul memintanya secara eksplisit.
- Jangan tulis rumus konkret di berkas ini. Format rumus menunggu kesepakatan terpisah.
- **Variabel turunan NIM**<br>
  Aturan turunan wajib berasal dari PDF modul dan dicatat di `MODULE_VARIABLES.md` pada root proyek. Jangan pakai rumus universal atau menyalin nilai dari laporan lain. Bila modul tidak menyebut aturan, tandai `ragu` di `GAP_REGISTER.md` dan minta keputusan manusia.

## 5. Aturan bukti

- Bukti harus asli dan cocok modul atau kasus: output CLI asli, hasil SQL, log konsol Java, show ip route.
- Jangan gunakan kanvas topologi kosong sebagai bukti konektivitas atau routing.
- Format bukti per modul ditetapkan di `PROOF_CONTRACT.md` pada root proyek (dibuat Fase 2 dari PDF modul).
- Kontrak memakai empat jenis: `cli-transcript`, `dbms-result`, `program-console`, `gui-state`.
- `gui-state` tidak boleh berdiri sendiri untuk konektivitas/routing; sertakan ID `cli-transcript` pendamping.
- Perintah wajib dijalankan nyata dengan NIM sudah disubstitusi; kriteria lulus memuat token yang harus muncul dan larangan spesifik.
- **Sisipan gambar**<br>
  Gunakan `#step-figure("<path>", [caption], width: 100%)` untuk B5 atau `#output-figure("<path>", [caption], width: 100%)` untuk OOP. Argumen pertama adalah path string, bukan elemen `image(...)`. Nomor caption `Gambar N.` dibuat otomatis; `width` boleh di-override per gambar.
- **Crop screenshot**<br>
  Screenshot dipangkas dengan `-trim +repage -border 25` sebelum disisipkan.

## 6. Gate kualitas

Perintah verifikasi:

- `lab-report lint database`
- `lab-report test database`
- `lab-report build database -m <modul>`
- `lab-report pack database -m <modul>`
- **Audit akhir**<br>
  Jalankan `lab-report audit` sebelum pengumpulan. Perintah ini menolak temuan Fase 6 yang masih `kurang`, `bolong`, atau `ragu` dengan keputusan `go`, menandai item `no-go` untuk konfirmasi manusia, serta membandingkan PDF di `dist/` dengan salinan di dalam ZIP. Ulangi `lab-report pack` bila PDF berubah setelah pengemasan.
- `cargo test`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `.venv/bin/python -m pytest -n auto`

Kriteria lulus: nol masalah lint, geometri 0.00pt, build keluar kode 0.

- **Audit Fase 6**<br>
  Sebelum pengumpulan, periksa empat hal: slop (kalimat generik atau instruksi tertinggal), analisis terlewat (ID di `MODULE_VARIABLES.md` dan `PROOF_CONTRACT.md` tidak dibahas), bagian hilang (section kosong sesuai kontrak S3), dan halusinasi modul (angka tanpa sumber di modul atau `materials/`).
- **Pencatatan temuan**<br>
  Masukkan setiap temuan ke `GAP_REGISTER.md` dengan `Kategori` (`slop`, `analisis-terlewat`, `bagian-hilang`, `halusinasi-modul`) dan `Bukti` berupa lokasi file:baris. `ANALISIS_VERIFIKASI.md` dan `REVISI_SUMMARY.md` hanya ringkasan opsional, bukan berkas wajib.

## 7. Cuplikan ilustratif

Format langkah dua baris:

```
*Inisialisasi Basis Data* \
Pengujian inisialisasi basis data relasional dan verifikasi skema tabel.
```
```
== Langkah 1 - Judul Langkah
```
Heading level 2 dipakai untuk setiap Langkah, sehingga berukuran 12pt dan rata kiri.

Kode SQL:

```sql
CREATE TABLE mahasiswa (
  nim VARCHAR(10) PRIMARY KEY,
  nama VARCHAR(100) NOT NULL
);
```

Konvensi caption gambar, tabel analitis, artefak pengumpulan, dan infrastruktur agen menunggu kesepakatan terpisah.

#comparison-table((30mm, 80mm), [Perbandingan sesuai modul], [Aspek], [Penjelasan], [Contoh], [Isi dari modul])
