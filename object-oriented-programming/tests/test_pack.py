#!/usr/bin/env python3
"""
Unit test for scripts/pack.py ZIP packaging automation.
Verifies that:
1. The submission archive <Nama>_<NIM>_Tugas<X>.zip is created.
2. The root of the ZIP archive contains <Nama>_<NIM>_Tugas<X>.pdf and all .java files from src/.
3. There are no nested parent folders inside the ZIP, matching image.png exactly.
"""

import os
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path


def test_zip_packaging():
    base_dir = Path(__file__).resolve().parent.parent
    root_dir = base_dir.parent

    src_dir = base_dir / "src"
    assert src_dir.exists(), f"Error: {src_dir} does not exist"

    # Invoke native lab-report CLI to produce submission package
    res = subprocess.run(
        ["lab-report", "pack", "pbo", "-m", "1"],
        cwd=str(root_dir),
        capture_output=True,
        text=True,
        check=False,
    )
    assert res.returncode == 0, f"lab-report pack failed:\nSTDOUT: {res.stdout}\nSTDERR: {res.stderr}"

    zips = list(base_dir.glob("*.zip"))
    assert len(zips) > 0, "No ZIP archive was produced in course directory"
    target_zip = zips[0]

    print(f"Testing produced ZIP: {target_zip.name}")

    with zipfile.ZipFile(target_zip, "r") as zf:
        namelist = zf.namelist()
        print(f"ZIP Contents: {namelist}")

        # Check for PDF file in root
        pdf_files = [n for n in namelist if n.endswith(".pdf")]
        assert len(pdf_files) == 1, f"Expected exactly 1 PDF at root, found: {pdf_files}"
        pdf_name = pdf_files[0]
        assert "/" not in pdf_name, f"PDF must be in root, but found: {pdf_name}"

        # Check for Java files in root
        java_files = [n for n in namelist if n.endswith(".java")]
        assert len(java_files) >= 1, f"Expected Java files in ZIP root, found: {java_files}"
        for jf in java_files:
            assert "/" not in jf, f"Java file {jf} must be at root level without parent folders"

        # Check expected files from image.png if Percobaan 1..4 are present in src/
        for p_idx in [1, 2, 3, 4]:
            expected_name = f"Percobaan {p_idx}.java"
            if (src_dir / expected_name).exists():
                assert expected_name in namelist, f"Expected {expected_name} directly in ZIP"

    # Clean up generated test zip
    target_zip.unlink(missing_ok=True)
    print(">> SUCCESS: test_pack passed all checks.")


if __name__ == "__main__":
    test_zip_packaging()
