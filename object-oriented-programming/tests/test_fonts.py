import os
import subprocess
import sys

def test_fonts():
    fonts_dir = os.path.join(os.path.dirname(__file__), "..", "fonts")
    if not os.path.exists(fonts_dir):
        print(f"Error: fonts directory does not exist: {fonts_dir}", file=sys.stderr)
        sys.exit(1)

    cmd = ["typst", "fonts", "--variants", "--font-path", fonts_dir]
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if res.returncode != 0:
        print(f"Error running typst fonts: {res.stderr}", file=sys.stderr)
        sys.exit(1)

    output = res.stdout
    has_arial_black = "ariblk.ttf" in output and "Weight: 900" in output
    has_times = "Times New Roman" in output and "times.ttf" in output and "timesbd.ttf" in output

    if not has_arial_black:
        print("Error: Arial Black (ariblk.ttf Weight: 900) not found in typst fonts output", file=sys.stderr)
        sys.exit(1)
    if not has_times:
        print("Error: Times New Roman not found in typst fonts output", file=sys.stderr)
        sys.exit(1)

    print("SUCCESS: Arial Black (Weight 900) and Times New Roman (Regular & Bold) loaded in typst.")

if __name__ == "__main__":
    test_fonts()
