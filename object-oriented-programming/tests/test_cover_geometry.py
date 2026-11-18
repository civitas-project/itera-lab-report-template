import os
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

def extract_bbox_xml(pdf_path):
    cmd = ["pdftotext", "-bbox-layout", str(pdf_path), "-"]
    res = subprocess.run(cmd, check=True, stdout=subprocess.PIPE, text=True)
    raw = res.stdout
    if "<!DOCTYPE" in raw:
        idx = raw.find("<doc>")
        raw = "<root>" + raw[idx:raw.rfind("</doc>") + 6] + "</root>"
    else:
        raw = "<root>" + raw + "</root>"
    return ET.fromstring(raw)

def extract_page_lines(root, page_idx):
    pages = list(root.iter("page"))
    if page_idx >= len(pages):
        return []
    lines = []
    for line in pages[page_idx].iter("line"):
        words = [w.text for w in line.iter("word") if w.text]
        text = " ".join(words).strip()
        if text:
            lines.append({
                "text": text,
                "xMin": float(line.get("xMin", "0")),
                "yMin": float(line.get("yMin", "0")),
                "xMax": float(line.get("xMax", "0")),
                "yMax": float(line.get("yMax", "0")),
            })
    return lines

def test_cover_geometry():
    base_dir = Path(__file__).resolve().parent.parent
    ref_candidates = [
        Path.cwd() / "Template Praktikum PBO RD.pdf",
        base_dir / "Template Praktikum PBO RD.pdf",
        base_dir.parent / "Template Praktikum PBO RD.pdf",
    ]
    ref_pdf = next((c for c in ref_candidates if c.exists()), None)
    if not ref_pdf:
        print("Error: Reference PDF Template Praktikum PBO RD.pdf not found", file=sys.stderr)
        sys.exit(1)

    output_pdf = base_dir / "test_output.pdf"
    typ_file = base_dir / "main.typ"
    fonts_dir = base_dir / "fonts"

    if not typ_file.exists():
        print(f"Error: {typ_file} does not exist", file=sys.stderr)
        sys.exit(1)

    compile_cmd = ["typst", "compile", "--font-path", str(fonts_dir), str(typ_file), str(output_pdf)]
    subprocess.run(compile_cmd, check=True)

    ref_root = extract_bbox_xml(ref_pdf)
    out_root = extract_bbox_xml(output_pdf)

    ref_lines = extract_page_lines(ref_root, 0)
    out_lines = extract_page_lines(out_root, 0)

    # Key geometric anchors that must match within sub-point tolerance
    key_anchors = [
        "LAPORAN PRAKTIKUM",
        "PEMROGRAMAN BERORIENTASI OBJEK",
        "MODUL X",
        "JUDUL PRAKTIKUM",
        "Disusun Oleh:",
        "Nama",
        "NIM",
        "Program Studi",
        "Mata Kuliah",
        "Kelas",
        "Dosen Pengampu",
        "Tim Asisten",
        "[Nama Mahasiswa]",
        "[NIM]",
        "Teknik Informatika",
        "Pemrograman Berorientasi Objek",
        "[Kelas PBO (Bukan Kelas Praktikum)]",
        "TEKNIK INFORMATIKA",
        "FAKULTAS TEKNOLOGI INDUSTRI",
        "INSTITUT TEKNOLOGI SUMATERA",
        "[BULAN PENGERJAAN] 2026",
    ]

    print(f"Verifying {len(key_anchors)} Key Cover Geometric Anchors...")
    tolerance = 0.0  # Absolute zero tolerance
    mismatches = 0

    student_file = base_dir / "data" / "student.typ"
    student_nama = "[Nama Mahasiswa]"
    student_nim = "[NIM]"
    if student_file.exists():
        content = student_file.read_text(encoding="utf-8")
        m_nama = re.search(r'nama\s*:\s*"([^"]+)"', content)
        m_nim = re.search(r'nim\s*:\s*"([^"]+)"', content)
        if m_nama:
            student_nama = m_nama.group(1)
        if m_nim:
            student_nim = m_nim.group(1)

    for anchor in key_anchors:
        ref_match = next((l for l in ref_lines if l["text"] == anchor), None)
        if anchor == "[Nama Mahasiswa]":
            out_match = next((l for l in out_lines if l["text"] in [anchor, student_nama]), None)
        elif anchor == "[NIM]":
            out_match = next((l for l in out_lines if l["text"] in [anchor, student_nim]), None)
        else:
            out_match = next((l for l in out_lines if l["text"] == anchor), None)

        if not ref_match:
            print(f"Warning: Anchor not in ref: {anchor}")
            continue
        if not out_match:
            mismatches += 1
            print(f"FAIL: Anchor missing in output: {anchor}")
            continue

        dx = abs(out_match["xMin"] - ref_match["xMin"])
        dy = abs(out_match["yMin"] - ref_match["yMin"])

        if round(dx, 1) > tolerance or round(dy, 1) > tolerance:
            mismatches += 1
            print(f"FAIL: {anchor} ref({ref_match['xMin']:.2f}, {ref_match['yMin']:.2f}) out({out_match['xMin']:.2f}, {out_match['yMin']:.2f}) dx={dx:.2f} dy={dy:.2f}")
        else:
            print(f"  OK: {anchor:<35} | dx={dx:.2f}pt, dy={dy:.2f}pt")

    output_pdf.unlink(missing_ok=True)

    if mismatches > 0:
        print(f"\n>> {mismatches} anchors failed verification.", file=sys.stderr)
        sys.exit(1)

    print("\n>> SUCCESS: All key cover geometric anchors conform to absolute zero tolerance (0.0pt).")

if __name__ == "__main__":
    test_cover_geometry()
