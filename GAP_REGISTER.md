# GAP_REGISTER: Pencatatan Item Kurang / Bolong / Ragu

Berkas ini mencatat item yang kurang, bolong, atau ragu yang ditemukan selama siklus pembuatan laprak (lihat `SIKLUS_PEMBUATAN_LAPRAK.md`). Satu file untuk semua course.

## Ringkasan

| Metrik | Jumlah |
|--------|--------|
| Total item | 0 |
| Status: kurang | 0 |
| Status: bolong | 0 |
| Status: ragu | 0 |
| Status: ditangani | 0 |
| Belum ditangani | 0 |

## Aturan Pengisian

- *ID* `GR-F{fase}-{urutan 2 digit}`, contoh `GR-F2-01` (Fase 2, item ke-1).
- *Status* `kurang` / `bolong` / `ragu` / `ditangani`.
- *Pemilik* `Manusia` / `AI` / `Bersama`.
- *Keputusan* `go` (lanjut) / `no-go` (tunda fase berikutnya).
- Item `kurang` atau `bolong` **wajib** punya keputusan `go` / `no-go` sebelum lanjut ke fase berikutnya.
- Item `ragu` **wajib** diklarifikasi; tidak boleh diabaikan diam-diam.
- *Kategori* `slop`, `analisis-terlewat`, `bagian-hilang`, atau `halusinasi-modul`. Isi `-` bila tidak relevan.
- *Bukti* Lokasi temuan, misalnya `step-2.typ:14` atau `PROOF_CONTRACT.md` P-02. Wajib untuk item hasil audit Fase 6.
- Setelah ditangani, ganti status menjadi `ditangani` dan isi `Catatan penanganan`; jangan hapus baris.

## Fase 1: Inisialisasi & Metadata

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 2: Ingesti Materi PDF

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 3: Brainstorming & Konfirmasi

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 4: Pembuatan Konten Laprak

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 5: Build & Verifikasi

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 6: Review

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 7: Revisi

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Fase 8: Paket & Pengumpulan

| ID | Fase | Item | Kategori | Bukti | Status | Pemilik | Keputusan | Catatan | Referensi |
|----|------|------|--------|-------|--------|---------|-----------|---------|-----------|

## Contoh Baris Setelah Ditangani

| GR-F6-01 | 6 | Angka subnet tidak sesuai variabel modul | halusinasi-modul | step-2.typ:14 vs MODULE_VARIABLES.md V-01 | ditangani | AI | go | Angka diperbaiki menjadi 192.168.75.0/24 sesuai modul | MODUL 5 PDF hal. 4 |
