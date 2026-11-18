# Security Policy

## Supported Versi

| Versi | Status |
|-------|--------|
| 0.1.x | didukung |

Rilis alpha menerima perbaikan hanya dengan upaya terbaik.

## Melaporkan Kerentanian

Kerentangan HARUS TIDAK dilaporkan melalui isu publik GitHub atau Codeberg. Kirim laporan secara pribadi ke email `timothy@itera.ac.id`.

Sertakan dalam laporan:
- versi yang terkena dampak
- langkah reproduksi
- dampak
- bukti konsep (proof-of-concept), jika tersedia

## SLA Pengungkapan

| Tahapan | Target |
|---------|--------|
| triase | 24 jam |
| reproduksi | 72 jam |
| patch | 14 hari |
| pengungkapan publik | 30 hari setelah patch |

Masalah yang belum di-patch tetap bersifat pribadi sampai jendela waktu ditutup.

## Cakupan

Dalam cakupan:
- CLI native `lab-report`
- skrip pembangun
- pipeline rendering templat

Di luar cakupan:
- runtime pihak ketiga Typst, MariaDB, dan Java
- PDF modul yang disediakan dosen pengampu

## Apresiasi

Laporan ditangani secara rahasia. Kredit pelapor hanya diberikan dengan persetujuan pelapor.
