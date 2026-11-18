import os
import subprocess
import sys
import pymupdf

def test_full_compile():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    main_typ = os.path.join(base_dir, "main.typ")
    output_pdf = os.path.join(base_dir, "output.pdf")
    fonts_dir = os.path.join(base_dir, "fonts")
    content_dir = os.path.join(base_dir, "content")

    # Check content files existence
    expected_contents = [
        "01-tujuan.typ",
        "02-alat-bahan.typ",
        "03-langkah.typ",
        "04-source-code.typ",
        "05-output.typ",
        "06-analisis.typ",
        "07-kesimpulan.typ",
    ]
    for c in expected_contents:
        c_path = os.path.join(content_dir, c)
        if not os.path.exists(c_path):
            print(f"FAIL: Missing content file {c}", file=sys.stderr)
            sys.exit(1)

    cmd = ["typst", "compile", "--font-path", fonts_dir, main_typ, output_pdf]
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if res.returncode != 0:
        print(f"FAIL: Typst compile failed:\n{res.stderr}", file=sys.stderr)
        sys.exit(1)

    if not os.path.exists(output_pdf):
        print(f"FAIL: Output PDF was not created", file=sys.stderr)
        sys.exit(1)

    doc = pymupdf.open(output_pdf)
    page_count = len(doc)
    print(f"Compilation successful. Generated PDF has {page_count} pages.")
    if page_count != 3:
        print(f"FAIL: Expected 3 pages, got {page_count}", file=sys.stderr)
        sys.exit(1)

    print("SUCCESS: Full compile test passed with 3 pages and all modular content files.")

if __name__ == "__main__":
    test_full_compile()
