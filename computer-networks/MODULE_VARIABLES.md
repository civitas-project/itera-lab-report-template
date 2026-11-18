# Module Variables - Computer Networks - Modul 5

## Identitas

- Mata kuliah `computer-networks`
- Modul contoh 5 (OSPF/VLSM)
- NIM contoh `125140075` dari `profile.toml`
- Sumber `PDF modul praktikum Computer Networks`

## Aturan penulisan

- **Aturan turunan**<br>
Wajib berasal dari PDF modul; jangan pakai rumus universal; kalau PDF tidak menyebut, tandai `ragu` di `GAP_REGISTER.md` dan minta keputusan manusia.

- **Nilai**<br>
Dihitung dari NIM mahasiswa yang sedang diproses, bukan dari contoh.

- **Sumber**<br>
Seksi PDF modul, bukan nomor halaman rekaan.

- **Status**<br>
`go` bila aturan ada di PDF; `ragu` bila harus diklarifikasi.

## Daftar variabel

| ID | Variabel | Aturan turunan (dari PDF modul) | Nilai (NIM 125140075) | Sumber PDF | Status |
|----|----------|---------------------------------|-----------------------|------------|--------|
| V-01 | Blok jaringan | Dua digit terakhir NIM dipakai sebagai oktet ketiga pada `192.168.X.0/24` | `192.168.75.0/24` | PDF modul, bagian skema pengalamatan IP | `go` |
| V-02 | Subnet LAN kabel | Alokasi `/26` pertama dari blok V-01 | `192.168.75.0/26` | PDF modul, bagian VLSM | `go` |
| V-03 | Subnet LAN nirkabel | Alokasi `/26` berikutnya dari blok V-01 | `192.168.75.64/26` | PDF modul, bagian VLSM | `go` |
| V-04 | Tautan antar-router | Alokasi `/30` berikutnya dari blok V-01 | `192.168.75.128/30` | PDF modul, bagian VLSM | `go` |

## Catatan

Nilai di atas hanya contoh untuk NIM 125140075. Untuk mahasiswa lain, hitung ulang dari NIM `profile.toml`; jangan menyalin nilai contoh.

## Modul lain

Belum ada modul lain di phase 2. Salin tabel di atas untuk setiap modul baru dan sesuaikan ID serta aturan turunan dari PDF modul tersebut.
