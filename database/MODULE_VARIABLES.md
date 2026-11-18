# Module Variables - Database Systems - Modul DDL

## Identitas
- **Mata kuliah**<br>
  database
- **Modul contoh**<br>
  DDL Tingkat Lanjut
- **NIM contoh**<br>
  125140075 dari `profile.toml`
- **Sumber**<br>
  PDF modul praktikum Database Systems

## Aturan penulisan
- **Aturan turunan**<br>
  Turunan berasal dari PDF modul praktikum; tidak ada rumus universal yang berlaku untuk semua modul.
- **Nilai dari NIM**<br>
  Nilai dihitung dari NIM mahasiswa yang diproses melalui `profile.toml`.
- **Sumber PDF**<br>
  Sumber setiap variabel adalah seksi PDF modul yang menyatakan aturannya.
- **Status**<br>
  Status `go` bila aturan nyata dalam PDF, `ragu` bila modul tidak menyebutkan; tidak ditebak.

## Daftar variabel
| ID | Variabel | Aturan turunan (dari PDF modul) | Nilai (NIM 125140075) | Sumber PDF | Status |
|---|---|---|---|---|---|
| V-01 | NIM mahasiswa | Nilai kolom NIM diambil dari `profile.toml`, bukan rekaan | `125140075` | PDF modul, bagian struktur tabel mahasiswa | `go` |
| V-02 | Kolom kunci utama | Kolom yang ditetapkan modul sebagai primary key pada tabel mahasiswa | `nim` | PDF modul, bagian DDL `CREATE TABLE` | `go` |
| V-03 | Nama skema | Nama skema/DB harus sesuai yang ditentukan modul; bila modul tidak menyebut, status `ragu` | Sesuai modul (contoh laporan memakai `siakad`) | PDF modul, bagian identifikasi skema | `ragu` |

## Modul lain
| ID | Variabel | Aturan turunan (dari PDF modul) | Nilai (NIM 125140075) | Sumber PDF | Status |
|---|---|---|---|---|---|

Diisi dari PDF modul masing-masing pada fase 2.

Catatan akhir: Nilai di atas contoh untuk NIM 125140075. Untuk mahasiswa lain, baca kembali `profile.toml` dan PDF modul; jangan menyalin contoh.
