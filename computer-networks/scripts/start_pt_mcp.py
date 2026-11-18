#!/usr/bin/env python3
"""Entry point: jalankan Packet Tracer + MCP bridge untuk integrasi AI agent.

Prinsip:
- PT versi >= 8.2 harus terinstal (hard fail jika di bawah).
- MCP extension diinstall sekali, skip jika sudah ada.
- Background: semua operasi headless (tidak membuka jendela PT GUI).
"""

from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path

from pt_utils import ExtensionError, PacketTracerDetector, PacketTracerError


def wait_port(port: int, host: str = "127.0.0.1", timeout: float = 30.0) -> bool:
    """Tunggu port TCP terbuka."""
    import socket

    start = time.time()
    while time.time() - start < timeout:
        try:
            with socket.create_connection((host, port), timeout=1.0):
                return True
        except OSError:
            time.sleep(0.5)
    return False


def ensure_pt_running(detector: PacketTracerDetector) -> None:
    """Pastikan Packet Tracer sedang berjalan (headless)."""
    import subprocess

    try:
        subprocess.run(
            ["pgrep", "-f", "PacketTracer"],
            check=True,
            capture_output=True,
        )
        print("✅ Packet Tracer sudah berjalan")
        return
    except subprocess.CalledProcessError:
        pass

    pt_root = detector.find_packet_tracer()
    if not pt_root:
        raise PacketTracerError("PT tidak ditemukan, tidak dapat meluncurkan")

    exe = detector._find_executable(pt_root)
    if not exe:
        raise PacketTracerError("Executable PT tidak ditemukan")

    print("🚀 Meluncurkan Packet Tracer (headless)...")
    subprocess.Popen(
        [str(exe)],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


def ensure_mcp_server() -> subprocess.Popen:
    """Jalankan MCP server (cisco-pt-mcp) jika belum running."""
    import subprocess

    try:
        subprocess.run(
            ["pgrep", "-f", "packet_tracer_mcp"],
            check=True,
            capture_output=True,
        )
        print("✅ MCP server sudah berjalan")
        # Return a dummy process that won't be terminated
        return subprocess.Popen(["true"])
    except subprocess.CalledProcessError:
        pass

    print("🚀 Meluncurkan MCP server...")
    proc = subprocess.Popen(
        ["uvx", "cisco-pt-mcp"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return proc


def main() -> int:
    print("=" * 60)
    print("  MCP Bridge Setup - Cisco Packet Tracer")
    print("=" * 60)

    detector = PacketTracerDetector()

    # 1. Deteksi PT dan cek versi
    print("\n[1/4] Deteksi Packet Tracer...")
    try:
        detector.check_version()
        print(f"✅ PT versi {detector.get_version_string()} (>= 8.2)")
    except PacketTracerError as exc:
        print(f"❌ {exc}")
        return 1

    # 2. Install extension jika belum ada
    print("\n[2/4] Cek MCP extension...")
    try:
        detector.ensure_extension(detector.EXTENSION_SOURCE_URL)
    except ExtensionError as exc:
        print(f"❌ {exc}")
        return 1

    # 3. Start PT
    print("\n[3/4] Pastikan PT berjalan...")
    try:
        ensure_pt_running(detector)
    except PacketTracerError as exc:
        print(f"❌ {exc}")
        return 1

    if not wait_port(54321, timeout=60):
        print("❌ PT HTTP bridge (54321) tidak siap dalam 60 detik")
        return 1
    print("✅ PT bridge siap (54321)")

    # 4. Start MCP server
    print("\n[4/4] Start MCP server...")
    mcp_proc = ensure_mcp_server()

    if not wait_port(39000, timeout=30):
        print("❌ MCP server (39000) tidak siap dalam 30 detik")
        mcp_proc.terminate()
        return 1
    print("✅ MCP server siap (39000)")

    print("\n" + "=" * 60)
    print("  MCP Bridge READY")
    print("=" * 60)
    print(f"  PT Bridge : http://127.0.0.1:54321")
    print(f"  MCP Server: http://127.0.0.1:39000/mcp")
    print("=" * 60)

    try:
        mcp_proc.wait()
    except KeyboardInterrupt:
        print("\n🛑 Menghentikan MCP server...")
        mcp_proc.terminate()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
