import shutil
import sys

def test_environment():
    try:
        import pymupdf
        print("PyMuPDF is available:", pymupdf.__version__)
    except ImportError as e:
        print("Error: PyMuPDF not found:", e, file=sys.stderr)
        sys.exit(1)

    try:
        import PIL
        print("Pillow is available:", PIL.__version__)
    except ImportError as e:
        print("Error: Pillow not found:", e, file=sys.stderr)
        sys.exit(1)

    try:
        import fontTools
        print("fontTools is available:", fontTools.__version__)
    except ImportError as e:
        print("Error: fontTools not found:", e, file=sys.stderr)
        sys.exit(1)

    typst_path = shutil.which("typst")
    if not typst_path:
        print("Error: typst binary not found in PATH", file=sys.stderr)
        sys.exit(1)
    print("typst binary found:", typst_path)

    pdftotext_path = shutil.which("pdftotext")
    if not pdftotext_path:
        print("Error: pdftotext binary not found in PATH", file=sys.stderr)
        sys.exit(1)
    print("pdftotext binary found:", pdftotext_path)

    print("ALL ENVIRONMENT CHECKS PASSED.")

if __name__ == "__main__":
    test_environment()
