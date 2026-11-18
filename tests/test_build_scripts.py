import subprocess
import sys
from pathlib import Path

import pytest


def test_no_sh_scripts_remain():
    root = Path(__file__).resolve().parent.parent
    sh_files = list(root.glob("**/*.sh"))
    # Exclude any third-party, hidden virtualenv, or release installer scripts
    sh_files = [f for f in sh_files if ".venv" not in f.parts and ".git" not in f.parts and f.name != "install.sh"]
    assert len(sh_files) == 0, f"Found remaining course .sh scripts: {sh_files}"


@pytest.mark.parametrize("course_dir", ["database", "computer-networks", "object-oriented-programming"])
def test_course_build_py_compiles(course_dir):
    root = Path(__file__).resolve().parent.parent
    script = root / course_dir / "build.py"
    assert script.exists(), f"Missing build.py in {course_dir}"
    res = subprocess.run(
        [sys.executable, str(script)], cwd=str(root / course_dir), capture_output=True, text=True, check=False
    )
    assert res.returncode == 0, f"build.py failed in {course_dir}:\n{res.stderr}\n{res.stdout}"
    dist_dir = root / course_dir / "dist"
    pdfs = list(dist_dir.glob("*.pdf")) if dist_dir.is_dir() else []
    assert pdfs, f"No final PDF found in {dist_dir}"
