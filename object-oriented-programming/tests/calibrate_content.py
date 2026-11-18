#!/usr/bin/env python3
"""
Interactive calibration tool for Page 2 and Page 3 content layout against Template Praktikum PBO RD.pdf.
"""
import subprocess
import xml.etree.ElementTree as ET

def extract_lines(pdf_path, page_idx):
    cmd = ["pdftotext", "-bbox-layout", pdf_path, "-"]
    res = subprocess.run(cmd, stdout=subprocess.PIPE, text=True, check=True)
    raw = res.stdout
    idx = raw.find("<doc>")
    raw = "<root>" + raw[idx:raw.rfind("</doc>") + 6] + "</root>"
    root = ET.fromstring(raw)
    pages = list(root.iter("page"))
    if page_idx >= len(pages):
        return []
    lines = []
    for l in pages[page_idx].iter("line"):
        words = [w for w in l.iter("word") if w.text]
        if not words:
            continue
        text = " ".join(w.text for w in words).strip()
        if words[0].text == "•" and len(words) > 1:
            lines.append({
                "text": "•",
                "xMin": float(words[0].get("xMin")),
                "yMin": float(words[0].get("yMin")),
                "xMax": float(words[0].get("xMax")),
                "yMax": float(words[0].get("yMax")),
            })
            rest_text = " ".join(w.text for w in words[1:]).strip()
            lines.append({
                "text": rest_text,
                "xMin": float(words[1].get("xMin")),
                "yMin": float(words[1].get("yMin")),
                "xMax": float(words[-1].get("xMax")),
                "yMax": float(words[-1].get("yMax")),
            })
        else:
            lines.append({
                "text": text,
                "xMin": float(l.get("xMin")),
                "yMin": float(l.get("yMin")),
                "xMax": float(l.get("xMax")),
                "yMax": float(l.get("yMax")),
            })
    return lines

def compare_page(ref_lines, typst_lines, page_num):
    print(f"\n=================== PAGE {page_num} COMPARISON ===================")
    print(f"{'Ref Text':<40} | {'Ref (x,y)':<16} | {'Typst (x,y)':<16} | {'Delta':<18} | Status")
    print("-" * 105)
    mismatches = 0
    matched_indices = set()
    for r in ref_lines:
        rx, ry = r["xMin"], r["yMin"]
        best_match = None
        best_idx = None
        best_dist = 999999.0
        for idx, t in enumerate(typst_lines):
            if idx in matched_indices:
                continue
            is_match = (
                t["text"] == r["text"]
                or (r["text"] == "" and t["text"] == "•")
                or (r["text"] == "•" and t["text"] == "")
                or r["text"].startswith(t["text"][:12])
                or t["text"].startswith(r["text"][:12])
            )
            if is_match:
                dist = abs(t["xMin"] - rx) + abs(t["yMin"] - ry)
                if dist < best_dist:
                    best_dist = dist
                    best_match = t
                    best_idx = idx
        txt = r["text"][:38]
        if best_match and best_idx is not None:
            matched_indices.add(best_idx)
            tx, ty = best_match["xMin"], best_match["yMin"]
            dx = abs(tx - rx)
            dy = abs(ty - ry)
            ok = round(dx, 1) <= 0.0 and round(dy, 1) <= 0.0
            if not ok:
                mismatches += 1
            st = "MATCH" if ok else f"DIFF(dx={dx:.2f},dy={dy:.2f})"
            print(f"{txt:<40} | {f'{rx:.2f}, {ry:.2f}':<16} | {f'{tx:.2f}, {ty:.2f}':<16} | dx={dx:.2f}pt, dy={dy:.2f}pt | {st}")
        else:
            mismatches += 1
            print(f"{txt:<40} | {f'{rx:.2f}, {ry:.2f}':<16} | {'MISSING':<16} | {'N/A':<18} | MISSING")
    print(f"Total: {len(ref_lines)} lines, Mismatches: {mismatches}")
    return mismatches

if __name__ == "__main__":
    from pathlib import Path
    base_dir = Path(__file__).resolve().parent.parent
    ref_candidates = [
        Path.cwd() / "Template Praktikum PBO RD.pdf",
        base_dir / "Template Praktikum PBO RD.pdf",
        base_dir.parent / "Template Praktikum PBO RD.pdf",
    ]
    ref = str(next((c for c in ref_candidates if c.exists()), "Template Praktikum PBO RD.pdf"))
    typ = str(base_dir / "output.pdf")
    for p in [1, 2]:
        ref_lines = extract_lines(ref, p)
        typ_lines = extract_lines(typ, p)
        compare_page(ref_lines, typ_lines, p + 1)
