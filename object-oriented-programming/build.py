#!/usr/bin/env python3
"""Cross-platform Python build script for Typst lab reports."""
from __future__ import annotations

import re
import shutil
import subprocess
import sys
from pathlib import Path


def main() -> int:
    course_dir = Path(__file__).resolve().parent
    main_typ = course_dir / "main.typ"
    fonts_dir = course_dir / "fonts"

    profile_path = course_dir / "profile.toml"
    if not profile_path.is_file():
        profile_path = course_dir.parent / "profile.toml"
    if profile_path.is_file():
        profile_text = profile_path.read_text(encoding="utf-8")
    else:
        profile_text = ""
    name_match = re.search(r'^name\s*=\s*"([^"]+)"', profile_text, re.M)
    nim_match = re.search(r'^nim\s*=\s*"([^"]+)"', profile_text, re.M)
    pattern_match = re.search(r'^pattern\s*=\s*"([^"]+)"', profile_text, re.M)
    name = name_match.group(1) if name_match else "Mahasiswa"
    nim = nim_match.group(1) if nim_match else "000000000"
    pattern = pattern_match.group(1) if pattern_match else "{name}_{nim}_Tugas{module}"

    argv = sys.argv[1:]
    module = "1"
    parsed = []
    index = 0
    while index < len(argv):
        arg = argv[index]
        if arg == "-m" and index + 1 < len(argv):
            module = argv[index + 1]
            index += 2
            continue
        if arg.startswith("--modul="):
            module = arg.split("=", 1)[1]
            index += 1
            continue
        if arg == "--modul" and index + 1 < len(argv):
            module = argv[index + 1]
            index += 2
            continue
        parsed.append(arg)
        index += 1

    action = parsed[0] if parsed and parsed[0] in ("build", "watch", "compile") else "build"
    extra_args = parsed[1:] if parsed and parsed[0] in ("build", "watch", "compile") else parsed

    filename = (
        pattern.replace("{name}", name.replace(" ", "_"))
        .replace("{nim}", nim)
        .replace("{module}", module)
    )
    if not filename.endswith(".pdf"):
        filename += ".pdf"
    dist_dir = course_dir / "dist"
    dist_dir.mkdir(parents=True, exist_ok=True)
    output_path = dist_dir / filename

    if shutil.which("typst") is None:
        print("Error: 'typst' executable not found in PATH.", file=sys.stderr)
        print("Please install Typst: https://github.com/typst/typst#installation", file=sys.stderr)
        return 1

    typst_cmd = "watch" if action == "watch" else "compile"
    cmd = ["typst", typst_cmd]
    if fonts_dir.exists():
        cmd.extend(["--font-path", str(fonts_dir)])
    cmd.extend([str(main_typ), str(output_path), *extra_args])

    return subprocess.run(cmd, check=False).returncode


if __name__ == "__main__":
    sys.exit(main())
