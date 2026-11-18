# Manual Penggunaan Laporan Praktikum ITERA

Manual tier 2 dengan pendekatan Diátaxis dan Progressive Disclosure (ISO/IEC/IEEE 26514:2022). Setiap bagian tersendiri: baca hanya bagian yang sesuai tugas saat ini. Tidak ada dump kode sumber di sini.

## Peta Pembaca

| Bagian | Audiens | Pertanyaan yang Dijawab | Ke mana harus pergi |
| Bagian I - Memulai | Mahasiswa baru, praktikum pertama kali | Bagaimana memulai dari nol hingga hasil PDF? | Bagian I |
| Bagian II - Panduan Praktikan | Praktikan aktif | Bagaimana mengisi konten dan memperbaiki temuan? | Bagian II |
| Bagian III - Operasional & Pemeliharaan | Panitia asisten, pemelihara | Bagaimana build, paket, dan troubleshooting? | Bagian III |
| Bagian IV - Arsitektur Internal | Pengembang, kontributor | Bagaimana alur kode dan template bekerja? | Bagian IV |
| Bagian V - Referensi Teknis | Semua pembaca | Batas sistem, subcommand, dan jejak requirement? | Bagian V |

## Bagian I - Memulai (Tutorial, 5 menit)

Model alur data:

```
profile.toml -> data/*.typ -> content/*.typ -> dist/
```

`profile.toml` berisi identitas mahasiswa (nama, NIM, kelas, bahasa). `data/*.typ` berisi metadata laporan per modul. `content/*.typ` berisi isi bab. `dist/` berisi output PDF/SVG dan ZIP akhir.

Langkah ringkas:

1. Pasang binary `lab-report` (install.sh di Linux/macOS, install.ps1 di Windows, atau `cargo install --path crates/lab-report`).
2. `lab-report init` - wizard bilingual pertama kali.
3. `lab-report new <course> [dest]` - scaffold proyek starter.
4. `lab-report profile --name "Nama" --nim "123456789" --class "RX"` - isi profil mahasiswa.
5. `lab-report build <course> -m N` - kompilasi PDF modul N.
6. `lab-report audit` - periksa status kesenjangan dan segarnya paket.
7. `lab-report pack <course> -m N` - buat ZIP pengumpulan.

Contoh struktur proyek hasil scaffold:

```
<dest>/
├── REFERENCE.md
├── PROOF_CONTRACT.md
├── MODULE_VARIABLES.md
├── materials/
├── src/
└── dist/
```

## Bagian II - Panduan Praktikan (How-To)

### Mengisi PDF modul

Buka PDF modul dengan alat teks dan gambar:

- `pdftotext -layout`<br>Ekstrak teks modul dengan tata letak agar angka halaman dan judul bab terbaca.
- `pdfimages -list`<br>Daftarkan gambar embedded di PDF modul.
- `pdftoppm` + vision<br>Render halaman sebagai citra untuk inspeksi visual.

Gambar modul adalah konteks, bukan bukti. Bukti konektivitas adalah output CLI asli (mis. `Reply from ...` dari ping, `show ip route`, hasil eksekusi SQL, atau log konsol Java). Jangan terserahkan kanvas topologi kosong sebagai bukti.

### Menulis konten

Tulis bab di `content/*.typ` sesuai template course. Hindari teks aneh placeholder seperti instruksi template. Jika ada placeholder, hapus segera.

### Menjalankan pemeriksaan kualitas

```
lab-report lint <course>
lab-report test <course>
lab-report audit <course>
```

### Memperbaiki temuan GAP_REGISTER.md

Setiap linter atau audit menghasilkan entri di `GAP_REGISTER.md`. Perbaiki setiap entri satu per satu, lalu jalankan ulang `lab-report lint --fix` dan verifikasi ulang hingga nol temuan.

## Bagian III - Operasional & Pemeliharaan (Runbook)

### Build mandiri tanpa CLI binary

```
python3 build.py -m N
```

Jalankan di dalam direktori course. Menggunakan hanya Python standard library, tidak butuh pemasangan paket eksternal.

### Membuat ZIP pengumpulan

```
lab-report pack <course> -m N
```

ZIP berisi PDF dan kode sumber. Jika ZIP basi (PDF lama dibandingkan sumber), jalankan ulang `lab-report pack` untuk menyegarkan arsip.

### MCP server

```
lab-report mcp
```

Server JSON-RPC 2.0 over stdio untuk agen AI: inspect host, lint, render diagram, build, verify, pack, profile.

### Troubleshooting

- Typst tidak ditemukan:<br>Pasang typst atau gunakan `python3 build.py` yang hanya menyusun berkas Typst (kompilasi perlu binary typst terpisah).
- Course tidak terdeteksi:<br>Jalankan perintah di dalam direktori course, atau buat `.agent-context.md` di root workspace agar deteksi otomatis mengenali course.
- ZIP basi:<br>`lab-report pack` ulang.

Catatan rilis: pipeline CI rilis sengaja dihapus. Rilis dilakukan secara manual dari binary `target/release/lab-report`.

## Bagian IV - Arsitektur Internal (Explanation)

Alur modul Rust:

```
main.rs -> courses.rs -> runner.rs -> linter.rs -> verifier.rs -> packer.rs -> auditor.rs
```

- `main.rs`: titik entri CLI dan resolusi course.
- `courses.rs`: katalog course, alias, dan deteksi otomatis.
- `runner.rs`: kompilasi Typst, watch mode, sinkronisasi data mahasiswa, diagnosis perbaikan.
- `linter.rs`: pemeriksa tipografi dan invarian template.
- `verifier.rs`: verifikasi geometri koordinat dan regresi SVG.
- `packer.rs`: pembuatan ZIP pengumpulan.
- `auditor.rs`: audit status kesenjangan dan segarnya paket.

Template bersama tinggal di `templates/b5/`; setiap course punya shim (berkas pengikat) yang menyesuaikan elemen per course. Template di-staging di sandbox Typst terlebih dahulu untuk mengisolasi kompilasi dari berkas sumber asli dan mencegah pencemaran berkas sumber.

## Bagian V - Referensi Teknis

### Batas sistem

| ID | Batas | Sumber |
| LIM-01 | Lebar tabel total <= 125 mm (textwidth B5) | README |
| LIM-02 | Jarak baris 0.75em leading + spacing, tanpa spasi vertikal manual | AGENTS.md |
| LIM-03 | Judul bold tanpa kolon di akhir | AGENTS.md |
| LIM-04 | Startup CLI < 10 ms | README |
| LIM-05 | Zero peringatan clippy pada workspace | README |

### Daftar subcommand

| Subcommand | Fungsi |
| build | Kompilasi laporan ke PDF atau SVG |
| watch | Kompilasi ulang otomatis saat berkas berubah |
| pack | Buat ZIP pengumpulan (PDF + kode) |
| test | Verifikasi geometri koordinat dan font |
| audit | Audit status kesenjangan dan segarnya paket |
| lint | Audit invarian tipografi dan placeholder template |
| inspect | Periksa spesifikasi perangkat keras host |
| render | Kompilasi PlantUML ke vektor SVG |
| new | Scaffold proyek starter course |
| profile | Lihat atau ubah konfigurasi mahasiswa |
| init | Wizard setup bilingual pertama kali |
| mcp | Server JSON-RPC 2.0 untuk agen AI |
| tui | Antarmuka terminal interaktif |

Total 13 subcommand.

### Jejak requirement

19 requirement dari `docs/PRD.md` (`@req:FR-CLI-01` hingga `@req:FR-CLI-19`) yang masing-masing berpasangan dengan `@design`, `@impl`, dan `@test` dalam berkas sumber dan uji. Lihat rincian lengkap di `docs/PRD.md` dan `docs/TDD.md`.
