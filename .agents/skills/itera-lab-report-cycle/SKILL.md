---
name: itera-lab-report-cycle
description: Use when executing the 8-phase laprak creation cycle (metadata → ingesti materi → brainstorming → content creation → build/verify → review → revise → pack) for ITERA lab reports.
---

# Siklus Pembuatan Laprak: ITERA

Skill ini mengatur alur kerja (workflow) pembuatan laporan praktikum (laprak) dari inisialisasi metadata hingga pengumpulan. Skill ini berlaku secara universal untuk semua course (`computer-networks`, `database`, `object-oriented-programming`, dan lainnya). Parameter per-course (font, ukuran kertas, aturan NIM/variabel) diatur di `.agent-context.md` masing-masing course.

Pemuatan skill: load *terlebih dahulu* sebelum memulai siklus pembuatan laprak. Skill `itera-lab-report-standard` dimuat untuk aturan tipografi dan formatting (AGENTS.md).

---

## Ringkasan 8 Fase

| Fase | Nama | Input Utama | Output Utama | Gate Keluar |
|------|------|-------------|--------------|-------------|
| 1 | Inisialisasi & Metadata | `profile.toml`, data siswa | Metadata lengkap, `.agent-context.md` | Metadata valid dan terpaju |
| 2 | Ingesti Materi PDF | PDF materi praktikum | Ringkasan materi, variabel NIM | Setiap informasi kritis teridentifikasi |
| 3 | Brainstorming & Konfirmasi | Metadata + materi | Daftar gap (kurang/bolong/ragu) | Semua gap dijawab (go/no-go) |
| 4 | Pembuatan Konten Laprak | Hasil fase 1–3 | Draft Typst source | `lab-report lint` bersih |
| 5 | Build & Verifikasi | Draft Typst | PDF / SVG + laporan verifikasi | Kompilasi sukses, verifikasi lolos |
| 6 | Review | Draft + hasil build | Catatan revisi | Review disetujui atau revisi tercatat |
| 7 | Revisi | Catatan revisi | Revisi terintegrasi | Semua komentar ditindaklanjuti |
| 8 | Paket & Pengumpulan | Versi final | ZIP siap dikumpulkan | Paket lengkap dan terverifikasi |

---

## Fase 1: Inisialisasi & Metadata

*Tujuan* \ Kumpulkan dan validasi semua metadata siswa dan course.

*Aksi Agentic AI* \
- Baca `profile.toml` di root repo.
- Sinkronkan metadata ke `data/student.typ`, `data/class.typ`, `data/assistants.typ`, `data/report.typ` setiap course yang terkait.
- Generate `.agent-context.md` via `lab-report new <course>` (jika belum ada) atau perbarui jika metadata berubah.
- Verifikasi nama, NIM, kelas (RA/RB/RC/...), dosen pengampu, asisten, mata kuliah, modul, nomor tugas.

*Gate keluar* \ Metadata valid dan terpaju di semua course data directory.

*Dokumen terkait* \ `.agent-context.md` (per course), `profile.toml` (root).

---

## Fase 2: Ingesti Materi PDF

*Tujuan* \ Ekstrak semua informasi dari PDF materi praktikum resmi.

*Aksi Agentic AI* \
- Simpan PDF asli di `materials/` proyek. Jangan menyalin isinya langsung ke `content/`.
- Jalankan `pdfinfo materials/<modul>.pdf` untuk mengetahui jumlah halaman, lalu `pdftotext -layout materials/<modul>.pdf materials/modul.txt`. Flag `-layout` menjaga kolom tabel dan blok kode.
- Cek hasil dengan `wc -c materials/modul.txt`. Bila hampir kosong, teks tidak tersedia dan halaman harus dibaca secara visual.
- Jalankan `pdfimages -list materials/<modul>.pdf` untuk menemukan halaman berisi gambar. Render halaman itu dengan `pdftoppm -png -r 110 -f <n> -l <n> materials/<modul>.pdf materials/halaman`, lalu baca PNG dengan kemampuan vision agen.
- Catat isi gambar sebagai konteks atau instruksi modul, bukan sebagai bukti verifikasi. Bukti laporan wajib berasal dari eksekusi nyata di host. Nilai yang tidak jelas ditandai `ragu` di `GAP_REGISTER.md`.
- Ekstrak topik, alat & bahan, langkah praktikum, variabel NIM/IP, tugas, dan referensi dari PDF ke ringkasan terstruktur. Jangan menebak.
- Identifikasi variabel yang harus diturunkan dari NIM. Contoh ilustratif (bukan aturan): subnetting Jarkom pada modul tertentu memakai `10.NIM[8:10].NIM[10:12].0/24`. Aturan turunan wajib diambil dari PDF modul; catat ke `MODULE_VARIABLES.md`, dan bila tidak disebut modul tandai `ragu` di `GAP_REGISTER.md`.
- Simpan ringkasan materi terstruktur.

*Gate keluar* \ Setiap informasi kritis dari PDF sudah teridentifikasi; tidak ada dugaan kosong yang ditanggalkan.

*Dokumen terkait* \ PDF materi praktikum, `content/tools-and-materials.typ`, `content/documentation/step-*.typ`, `materials/`, `GAP_REGISTER.md`.

---

## Fase 3: Brainstorming & Konfirmasi Kelengkapan

*Tujuan* \ Memeriksa keterikatan antara metadata, materi, dan rencana isi laprak. Tandai apa yang kurang, bolong, atau ragu.

*Aksi Agentic AI* \
- Bandingkan hasil Fase 1 dan Fase 2 dengan template course.
- Tandai item yang kurang, bolong, atau ragu ke `GAP_REGISTER.md`.
- Untuk setiap item, tentukan status (`kurang` / `bolong` / `ragu`), pemilik (`Manusia` / `AI` / `Bersama`), dan keputusan (`go` / `no-go`).
- Item `kurang` atau `bolong` **wajib** punya keputusan sebelum lanjut ke Fase 4.
- Item `ragu` **wajib** diklarifikasi; tidak boleh diabaikan diam-diam.

*Gate keluar* \ Semua gap di Fase 3 sudah dijawab (diselesaikan atau secara sengaja diabaikan dengan alasan tertulis).

*Dokumen terkait:*
`GAP_REGISTER.md` (root).

---

## Fase 4: Pembuatan Konten Laprak

*Tujuan* \ Menulis draft laporan praktikum dalam format Typst sesuai invarian `AGENTS.md`.

*Aksi Agentic AI* \
- Tulis `content/*.typ`, `content/documentation/step-*.typ`, `tools-and-materials.typ`, dll.
- Terapkan invarian tipografi dari `AGENTS.md` dan skill `itera-lab-report-template`:
  - Spasi 1.5: `leading: 0.75em`, `spacing: 0.75em`, tanpa `#v(...)` / double blank line.
  - Tanpa kolon pada bold title: `*Judul* \ \n Penjelasan`, bukan `*Judul:*`.
  - Hanging indent, cover & inner title stack, table ≤ 125 mm, figure tight-crop.
- Hardware asli (bukan fabricated), output CLI autentik, zero placeholder text.
- Jalankan `lab-report lint` untuk mendeteksi pelanggaran.

*Gate keluar* \ `lab-report lint` bersih (tanpa pelanggaran tipografi dan placeholder).

*Dokumen terkait* \ `AGENTS.md`, `content/` (per course), `template/` (per course).

---

## Fase 5: Build & Verifikasi

*Tujuan* \ Kompilasi laporan dan verifikasi geometri serta font.

*Aksi Agentic AI* \
- Jalankan `lab-report build <course> -m <modul>`.
- Jalankan `lab-report test <course>` (verifikasi deklaratif geometri dan font).
- Jalankan `lab-report lint --fix <course>` untuk perbaikan otomatis yang aman.
- Jika build gagal, perbaiki syntax Typst dan ulangi.

*Gate keluar* \ Kompilasi berhasil, verifikasi lolos, linter bersih (exit code 0).

*Dokumen terkait* \ PDF/SVG output, `crates/lab-report/` (CLI).

---

## Fase 6: Review

*Tujuan* \ Tinjau draft sebelum pengumpulan dan temukan semua masalah yang belum terlihat saat menulis.

*Aksi Agentic AI* \
- Jalankan `lab-report lint <course>` dan baca ulang setiap paragraf.
- Lakukan empat pemeriksaan berikut, lalu catat setiap temuan ke `GAP_REGISTER.md`:
  - *Slop / tulisan murah* \
    Kalimat generik tanpa angka atau token bukti, klaim berhasil tanpa bukti, em-dash, atau instruksi template yang tertinggal. Gunakan `lab-report lint` sebagai pemeriksaan pertama.
  - *Analisis terlewat* \
    Setiap ID di `MODULE_VARIABLES.md` dan `PROOF_CONTRACT.md` harus dibahas di `content/*.typ`. ID yang tidak muncul ditandai `bolong`.
  - *Bagian hilang* \
    Cocokkan struktur file dengan "Kontrak per bagian" di `REFERENCE.md` bagian 3. Bagian yang kosong atau masih berisi placeholder ditandai `kurang`.
  - *Halusinasi modul* \
    Setiap angka, alamat, rumus, dan spesifikasi host harus bisa ditelusur ke `MODULE_VARIABLES.md`, `PROOF_CONTRACT.md`, hasil `lab-report inspect`, atau berkas di `materials/`. Yang tidak bisa ditelusur ditandai `ragu`, bukan ditulis sebagai fakta.
- Isi kolom `Kategori` dengan salah satu: `slop`, `analisis-terlewat`, `bagian-hilang`, `halusinasi-modul`.
- Isi kolom `Bukti` dengan lokasi temuan, misalnya `step-2.typ:14` atau `PROOF_CONTRACT.md` P-02.
- Tetapkan `Pemilik` (`Manusia`, `AI`, atau `Bersama`) dan `Keputusan` (`go` / `no-go`) untuk setiap temuan.
- Ringkasan `ANALISIS_VERIFIKASI.md` dan `REVISI_SUMMARY.md` bersifat opsional, dibuat dari isi `GAP_REGISTER.md` bila diminta, bukan berkas wajib.

*Gate keluar* \ Semua temuan Fase 6 memiliki kategori, bukti, pemilik, dan keputusan `go` / `no-go`.

*Dokumen terkait* \ Draft laporan, `GAP_REGISTER.md`, `REFERENCE.md`, `MODULE_VARIABLES.md`, `PROOF_CONTRACT.md`.

---

## Fase 7: Revisi

*Tujuan* \ Menerapkan semua catatan revisi dari Fase 6.

*Aksi Agentic AI* \
- Terapkan catatan revisi ke file Typst yang relevan.
- Update `GAP_REGISTER.md`: ubah status item menjadi `ditangani`, isi `Catatan penanganan`.
- Ulangi Fase 5 (build & verifikasi) sampai semua komentar ditangani dan lolos gate.

*Gate keluar* \ Semua komentar review ditindaklanjuti; build & lint lolos.

*Dokumen terkait* \ `GAP_REGISTER.md`, file Typst per course.

---

## Fase 8: Paket & Pengumpulan

*Tujuan* \ Membungkus PDF dan source code ke dalam ZIP siap dikumpulkan.

*Aksi Agentic AI* \
- Jalankan `lab-report pack <course> -m <modul>`.
- Verifikasi ZIP berisi PDF dan source code lengkap.
- Simpan paket di lokasi yang ditentukan untuk pengumpulan.

*Gate keluar* \ Paket lengkap dan terverifikasi.

*Dokumen terkait* \ ZIP output, `crates/lab-report/src/packer.rs`.

---

## Peran Manusia vs Agentic AI

| Fase | Manusia | Agentic AI |
|------|---------|------------|
| 1 | Isi/verifikasi metadata | Generate `.agent-context.md`, sinkronkan data siswa |
| 2 | Beri PDF materi | Ekstrak info, susun ringkasan, identifikasi variabel NIM |
| 3 | Konfirmasi gap & keputusan go/no-go | Tandai kurang/bolong/ragu, sarankan koreksi |
| 4 | Review isi teknis | Tulis konten Typst sesuai invarian `AGENTS.md` |
| 5 | Setujui hasil build | Jalankan `build` / `test` / `lint --fix`, perbaiki otomatis yang aman |
| 6 | Review akhir | Audit ulang invarian `AGENTS.md` |
| 7 | Setujui revisi | Terapkan catatan revisi |
| 8 | Kumpulkan | Buat ZIP paket via `lab-report pack` |

---

## Kaitan dengan GAP_REGISTER.md

`GAP_REGISTER.md` di root repo mencatat item **kurang**, **bolong**, atau **ragu** yang ditemukan di setiap fase.

- Fase 1–3: catat gap metadata dan materi.
- Fase 4–5: catat asumsi teknis dan kesalahan build/lint.
- Fase 6: catat komentar review dari dosen/asisten.
- Fase 7: catat revisi yang belum diterapkan.

*Aturan gate keluar fase* \ Sebelum melanjutkan ke fase berikutnya, semua item di fase saat ini dengan status `kurang` atau `bolong` harus sudah memiliki keputusan `go` atau `no-go`. Item `no-go` mencegah lanjut ke fase berikutnya.

Item `ragu` harus diklarifikasi dan diubah ke `ditangani` atau diputuskan sengaja diabaikan dengan catatan alasan.

---

## Checklist Eksekusi Agentic AI (Sebelum Lanjut Fase Berikutnya)

- [ ] Metadata terverifikasi dan terpaju di semua course (Fase 1).
- [ ] Informasi kritis dari PDF teridentifikasi, tidak ada dugaan kosong (Fase 2).
- [ ] Semua gap di `GAP_REGISTER.md` sudah punya keputusan `go` / `no-go` (Fase 3).
- [ ] `lab-report lint` bersih, tanpa colon pada bold title dan tanpa placeholder (Fase 4).
- [ ] `lab-report build` dan `lab-report test` exit code 0 (Fase 5).
- [ ] Hasil audit invarian `AGENTS.md` tercatat (Fase 6).
- [ ] Semua komentar revisi diterapkan, `GAP_REGISTER.md` diupdate ke status `ditangani` (Fase 7).
- [ ] ZIP paket terverifikasi lengkap (Fase 8).

---

## Red Flags: Segera Hentikan dan Perbaiki

| Rationalisasi / Excuse | Realitas & Aturan Mutlak |
|------------------------|--------------------------|
| *"Boleh pakai hardware spec generik (Intel i7, 16GB) untuk template."* | **DILARANG.** Evaluator memeriksa autentikasi sistem. Jalankan `lscpu` dan `free` pada host machine. |
| *"Tambah kolon setelah bold text seperti `*Langkah 1:*` agar rapi."* | **DILARANG.** Kolon setelah bold title melanggar standar tipografi repo. Pakai `*Langkah 1* \ \n isi`. |
| *"Spasi paragraf ekstra (12pt gap) membuat lebih mudah dibaca."* | **DILARANG.** Standar adalah Word 1.5 line spacing (`0.75em`) tanpa extra paragraph gap. |
| *"Screenshot topologi cukup sebagai bukti routing berfungsi."* | **DILARANG.** Bukti membutuhkan output CLI (`Reply from...`, `show ip route`). |
| *"Biarkan teks instruksi 'Jelaskan langkah-langkah...' untuk pembaca."* | **DILARANG.** Teks instruksi adalah placeholder slop template dan harus dihapus. |
| *"Tidak perlu recompile PDF setelah mengedit file .typ."* | **DILARANG.** Selalu compile dan test (`lab-report build <course>`) setelah perubahan. |

*Red Flags: Hentikan Segera Jika Terjadi* \
- Kemunculan `*Judul:*` di file step atau task.
- Extra blank lines atau `#v(...)` spacer antara paragraf body.
- Alamat subnet yang tidak cocok dengan NIM siswa.
- Screenshot Cisco Packet Tracer dengan interface LED merah/amber.
- Hardware spesifikasi yang tidak cocok dengan bacaan terminal `lscpu`/`free`.
- Return code $\ne 0$ dari `lab-report build` atau kegagalan tes dari `lab-report test`.
