#!/usr/bin/env python3
"""Interactive Setup Wizard untuk Packet Tracer + MCP Bridge.

Usage:
    python scripts/setup_wizard.py

This wizard guides a user through:
1. Checking Packet Tracer installation & version (>= 8.2 required)
2. Installing uv (Python package manager)
3. Downloading & installing MCP-Control-Center.pts extension
4. Starting PT bridge + MCP server
5. Verifying connection
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import urllib.request
from pathlib import Path

GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
RESET = "\033[0m"
BOLD = "\033[1m"

EXTENSION_NAME = "MCP-Control-Center.pts"
EXTENSION_URL = (
    "https://github.com/Mats2208/MCP-Packet-Tracer"
    "/releases/latest/download/MCP-Control-Center.pts"
)


def step(num: int, total: int, msg: str) -> None:
    print(f"\n{BLUE}{BOLD}[{num}/{total}]{RESET} {msg}")


def ok(msg: str) -> None:
    print(f"  {GREEN}✅{RESET} {msg}")


def fail(msg: str) -> None:
    print(f"  {RED}❌{RESET} {msg}")


def warn(msg: str) -> None:
    print(f"  {YELLOW}⚠️{RESET} {msg}")


def info(msg: str) -> None:
    print(f"  {BLUE}ℹ️{RESET} {msg}")


def run_cmd(cmd: str | list[str]) -> bool:
    """Run shell command, return True if success."""
    try:
        subprocess.run(cmd, shell=True, check=True, capture_output=True)
        return True
    except subprocess.CalledProcessError:
        return False


def check_pt_version() -> tuple[int, ...] | None:
    """Try to detect Packet Tracer version."""
    import platform

    exe_candidates: list[str] = []
    system = platform.system()

    if system == "Windows":
        exe_candidates = [
            "PacketTracer.exe",
            r"C:\Program Files\Cisco Packet Tracer\PacketTracer.exe",
            r"C:\Program Files (x86)\Cisco Packet Tracer\PacketTracer.exe",
        ]
    elif system == "Darwin":
        exe_candidates = [
            "/Applications/Cisco Packet Tracer.app/Contents/MacOS/Packet Tracer",
        ]
        home_app = Path.home() / "Library/Application Support/Cisco Packet Tracer"
        if home_app.exists():
            exe_candidates.append(str(home_app / "PacketTracer"))
    else:  # Linux
        exe_candidates = [
            "packettracer",
            "/opt/Cisco Packet Tracer/PacketTracer",
            "/usr/share/Cisco Packet Tracer/PacketTracer",
            "/opt/pt/PacketTracer",
        ]

    for exe in exe_candidates:
        try:
            result = subprocess.run(
                [exe, "--version"], capture_output=True, text=True, timeout=10
            )
            output = result.stdout + result.stderr
            import re

            m = re.search(r"(\d+)\.(\d+)\.(\d+)", output)
            if m:
                return tuple(int(x) for x in m.groups()[:3])
        except Exception:
            continue
    return None


def check_uv() -> bool:
    return shutil.which("uv") is not None


def install_uv() -> bool:
    system = sys.platform
    try:
        if system == "win32":
            cmd = 'powershell -c "irm https://astral.sh/uv/install.ps1 | iex"'
        else:
            cmd = "curl -LsSf https://astral.sh/uv/install.sh | sh"
        return run_cmd(cmd)
    except Exception:
        return False


def install_extension() -> bool:
    """Download MCP extension to PT extensions folder."""
    system = sys.platform
    home = Path.home()

    if system == "win32":
        ext_dir = Path(os.environ.get("LOCALAPPDATA", "")) / "Cisco Packet Tracer" / "extensions"
    elif system == "darwin":
        ext_dir = home / "Library/Application Support/Cisco Packet Tracer/extensions"
    else:
        ext_dir = home / ".local/share/Cisco Packet Tracer/extensions"

    dest = ext_dir / EXTENSION_NAME
    if dest.exists():
        ok(f"Extension sudah ada: {dest}")
        return True

    ext_dir.mkdir(parents=True, exist_ok=True)
    info(f"Mengunduh {EXTENSION_NAME}...")
    try:
        urllib.request.urlretrieve(EXTENSION_URL, dest)
        ok(f"Extension diunduh ke: {dest}")
        return True
    except Exception as exc:
        fail(f"Gagal mengunduh: {exc}")
        return False


def check_port(host: str, port: int, timeout: float = 10.0) -> bool:
    import socket

    import time as t

    start = t.time()
    while t.time() - start < timeout:
        try:
            with socket.create_connection((host, port), timeout=1.0):
                return True
        except OSError:
            t.sleep(0.5)
    return False


def start_pt() -> bool:
    """Start Packet Tracer if not running."""
    system = sys.platform
    try:
        subprocess.run(["pgrep", "-f", "PacketTracer"], check=True, capture_output=True)
        ok("Packet Tracer sudah berjalan")
        return True
    except subprocess.CalledProcessError:
        pass

    info("Meluncurkan Packet Tracer...")
    if system == "win32":
        exe = r"C:\Program Files\Cisco Packet Tracer\PacketTracer.exe"
    elif system == "darwin":
        exe = "/Applications/Cisco Packet Tracer.app/Contents/MacOS/Packet Tracer"
    else:
        exe = "packettracer"

    try:
        subprocess.Popen([exe], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    except FileNotFoundError:
        fail(f"Executable PT tidak ditemukan: {exe}")
        return False

    if not check_port(54321, timeout=60):
        fail("PT bridge (54321) tidak siap dalam 60 detik")
        return False
    ok("PT bridge siap (54321)")
    return True


def start_mcp() -> bool:
    """Start MCP server."""
    try:
        subprocess.run(
            ["pgrep", "-f", "packet_tracer_mcp"], check=True, capture_output=True
        )
        ok("MCP server sudah berjalan")
        return True
    except subprocess.CalledProcessError:
        pass

    info("Meluncurkan MCP server (uvx cisco-pt-mcp)...")
    subprocess.Popen(
        ["uvx", "cisco-pt-mcp"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    if not wait_port(39000, timeout=30):
        fail("MCP server (39000) tidak siap dalam 30 detik")
        return False
    ok("MCP server siap (39000)")
    return True


def wait_port(host: str, port: int, timeout: float = 30.0) -> bool:
    import socket

    import time as t

    start = t.time()
    while t.time() - start < timeout:
        try:
            with socket.create_connection((host, port), timeout=1.0):
                return True
        except OSError:
            t.sleep(0.5)
    return False


def verify() -> bool:
    """Verify both ports are reachable."""
    import socket

    ok_all = True
    for port, name in [(54321, "PT Bridge"), (39000, "MCP Server")]:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=2.0):
                ok(f"{name} ({port}) merespons")
        except OSError:
            fail(f"{name} ({port}) tidak merespons")
            ok_all = False
    return ok_all


def main() -> None:
    print(f"\n{BOLD}{BLUE}{'='*60}{RESET}")
    print(f"{BOLD}{BLUE}  📦 PACKET TRACER + MCP SETUP WIZARD{RESET}")
    print(f"{BLUE}{'='*60}{RESET}\n")

    # Step 1: PT version
    step(1, 5, "Cek Packet Tracer...")
    v = check_pt_version()
    if not v:
        fail("Packet Tracer tidak ditemukan!")
        info("Install dari https://www.netacad.com/cisco-packet-tracer")
        sys.exit(1)
    if v < (8, 2, 0):
        fail(f"Versi {v[0]}.{v[1]}.{v[2]} terlalu rendah (min 8.2)")
        sys.exit(1)
    ok(f"Packet Tracer {v[0]}.{v[1]}.{v[2]}")

    # Step 2: uv
    step(2, 5, "Cek uv...")
    if not check_uv():
        info("Menginstall uv...")
        if not install_uv():
            fail("Gagal install uv. Install manual: https://astral.sh/uv")
            sys.exit(1)
    ok("uv siap")

    # Step 3: Extension
    step(3, 5, "Pasang MCP Extension...")
    if not install_extension():
        fail("Gagal pasang extension")
        sys.exit(1)

    # Step 4: Start PT + MCP
    step(4, 5, "Mulai Packet Tracer + MCP...")
    if not start_pt():
        sys.exit(1)
    if not start_mcp():
        sys.exit(1)

    # Step 5: Verify
    step(5, 5, "Verifikasi koneksi...")
    if not verify():
        fail("Beberapa layanan tidak siap")
        sys.exit(1)

    # Done
    print(f"\n{GREEN}{BOLD}{'='*60}{RESET}")
    print(f"{BOLD}{GREEN}  🎉 SETUP SELESAI - AI AGENT SIAP PAKAI PACKET TRACER{RESET}")
    print(f"{BLUE}{'='*60}{RESET}\n")
    print(f"  {BLUE}PT Bridge   :{RESET} http://127.0.0.1:54321")
    print(f"  {BLUE}MCP Server  :{RESET} http://127.0.0.1:39000/mcp\n")
    print(f"  {BOLD}Langkah selanjutnya:{RESET}")
    print(f"    1. Konfigurasikan AI client (Claude/Cursor/VS Code)")
    print(f"       - Tambahkan server 'packet-tracer' dengan uvx cisco-pt-mcp")
    print(f"    2. Restart AI client")
    print(f"    3. Coba: 'Buat VLAN 10 di Packet Tracer'")
    print(f"\n{BOLD}{BLUE}{'='*60}{RESET}\n")


if __name__ == "__main__":
    main()
