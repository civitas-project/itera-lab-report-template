"""Packet Tracer detection and MCP extension management.

Cross-platform detection of Cisco Packet Tracer installation with support for:
- Multiple OS (Windows, macOS, Linux)
- Custom installation path via PACKET_TRACER_PATH env var or ~/.config/pt-mcp/config.toml
- Minimum version enforcement (>= 8.2)
- MCP Control Center extension management (skip if already installed)
"""

from __future__ import annotations

import os
import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Optional


class PacketTracerError(Exception):
    """Raised when Packet Tracer cannot be found or is incompatible."""


class ExtensionError(Exception):
    """Raised when MCP extension cannot be installed or verified."""


class PacketTracerDetector:
    """Deteksi dan manajemen Cisco Packet Tracer dan MCP extension."""

    MIN_VERSION: tuple[int, ...] = (8, 2, 0)
    EXTENSION_NAME = "MCP-Control-Center.pts"
    EXTENSION_SOURCE_URL = (
        "https://github.com/Mats2208/MCP-Packet-Tracer"
        "/releases/latest/download/MCP-Control-Center.pts"
    )

    # Default search paths per OS
    DEFAULT_PATHS: dict[str, list[Path]] = {
        "win32": [
            Path(os.environ.get("LOCALAPPDATA", "")) / "Cisco Packet Tracer",
            Path("C:/Program Files/Cisco Packet Tracer"),
            Path("C:/Program Files (x86)/Cisco Packet Tracer"),
        ],
        "darwin": [
            Path.home() / "Library/Application Support/Cisco Packet Tracer",
            Path("/Applications/Cisco Packet Tracer.app/Contents"),
        ],
        "linux": [
            Path.home() / ".local/share/Cisco Packet Tracer",
            Path("/opt/Cisco Packet Tracer"),
            Path("/usr/share/Cisco Packet Tracer"),
            Path("/opt/pt"),
        ],
    }

    def __init__(self) -> None:
        self.os_key: str = sys.platform
        self.custom_paths: list[Path] = self._load_custom_paths()

    # ------------------------------------------------------------------
    # Config discovery
    # ------------------------------------------------------------------

    def _load_custom_paths(self) -> list[Path]:
        """Cari path kustom dari env var dan config file."""
        paths: list[Path] = []

        # 1. Environment variable
        if env_path := os.environ.get("PACKET_TRACER_PATH"):
            p = Path(env_path)
            if p.exists():
                paths.append(p)

        # 2. Config file ~/.config/pt-mcp/config.toml
        config_path = Path.home() / ".config" / "pt-mcp" / "config.toml"
        if config_path.exists():
            try:
                with open(config_path, "rb") as f:
                    config = tomllib.load(f)
                pt_path = config.get("packet_tracer", {}).get("path")
                if pt_path:
                    p = Path(pt_path)
                    if p.exists():
                        paths.append(p)
            except Exception:
                pass

        return paths

    # ------------------------------------------------------------------
    # PT detection
    # ------------------------------------------------------------------

    def find_packet_tracer(self) -> Optional[Path]:
        """Cari instalasi Packet Tracer yang valid (versi >= 8.2)."""
        candidates = self.custom_paths + self.DEFAULT_PATHS.get(self.os_key, [])

        seen: set[Path] = set()
        for base in candidates:
            if not base.exists() or base in seen:
                continue
            seen.add(base)

            exe = self._find_executable(base)
            if not exe or not exe.exists():
                continue

            version = self.get_version(exe)
            if version and version >= self.MIN_VERSION:
                return base

        return None

    def _find_executable(self, pt_root: Path) -> Optional[Path]:
        """Cari executable PT di dalam direktori root."""
        candidates: list[Path] = []

        if self.os_key == "win32":
            candidates.extend([
                pt_root / "PacketTracer.exe",
                pt_root / "bin" / "PacketTracer.exe",
            ])
        elif self.os_key == "darwin":
            # macOS .app bundle
            candidates.extend([
                pt_root / "Packet Tracer.app" / "Contents" / "MacOS" / "Packet Tracer",
                pt_root / "Cisco Packet Tracer.app" / "Contents" / "MacOS" / "Packet Tracer",
            ])
        else:
            candidates.extend([
                pt_root / "PacketTracer",
                pt_root / "bin" / "PacketTracer",
                pt_root / "pt",
            ])

        for c in candidates:
            if c.exists() and c.is_file():
                return c
        return None

    def get_version(self, exe: Path) -> Optional[tuple[int, ...]]:
        """Ekstrak versi PT dari executable."""
        try:
            result = subprocess.run(
                [str(exe), "--version"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            output = (result.stdout or "") + (result.stderr or "")
            match = re.search(r"(\d+)\.(\d+)\.(\d+)", output)
            if match:
                return tuple(int(g) for g in match.groups()[:3])
        except Exception:
            pass
        return None

    def get_version_string(self) -> str:
        pt_root = self.find_packet_tracer()
        if not pt_root:
            return "not found"
        exe = self._find_executable(pt_root)
        if not exe:
            return "unknown"
        v = self.get_version(exe)
        if v:
            return f"{v[0]}.{v[1]}.{v[2]}"
        return "unknown"

    def check_version(self) -> None:
        """Verifikasi versi PT >= 8.2. Raise PacketTracerError jika tidak."""
        pt_root = self.find_packet_tracer()
        if not pt_root:
            raise PacketTracerError(
                "Packet Tracer tidak ditemukan. "
                "Set PACKET_TRACER_PATH atau buat ~/.config/pt-mcp/config.toml"
            )
        exe = self._find_executable(pt_root)
        if not exe:
            raise PacketTracerError("Executable Packet Tracer tidak ditemukan")
        version = self.get_version(exe)
        if version is None:
            raise PacketTracerError("Tidak dapat membaca versi Packet Tracer")
        if version < self.MIN_VERSION:
            raise PacketTracerError(
                f"Packet Tracer versi {version[0]}.{version[1]}.{version[2]} "
                f"terlalu rendah. Minimal {self.MIN_VERSION[0]}.{self.MIN_VERSION[1]}.{self.MIN_VERSION[2]}"
            )

    # ------------------------------------------------------------------
    # Extension management
    # ------------------------------------------------------------------

    def get_extensions_dir(self) -> Optional[Path]:
        pt_root = self.find_packet_tracer()
        if not pt_root:
            return None

        if self.os_key == "darwin":
            return pt_root / "extensions"
        return pt_root / "extensions"

    def is_extension_installed(self) -> bool:
        ext_dir = self.get_extensions_dir()
        if not ext_dir:
            return False
        return (ext_dir / self.EXTENSION_NAME).exists()

    def install_extension(self, source_path: Path) -> bool:
        """Install extension ke direktori PT. Return True jika berhasil."""
        ext_dir = self.get_extensions_dir()
        if not ext_dir:
            raise ExtensionError("Tidak dapat menemukan direktori extensions PT")

        ext_dir.mkdir(parents=True, exist_ok=True)
        dest = ext_dir / self.EXTENSION_NAME

        try:
            shutil.copy2(source_path, dest)
            return True
        except Exception as exc:
            raise ExtensionError(f"Gagal install extension: {exc}") from exc

    def ensure_extension(self, source_url: str) -> bool:
        """Pastikan extension terinstal. Download jika perlu, skip jika sudah ada."""
        ext_dir = self.get_extensions_dir()
        if not ext_dir:
            raise ExtensionError("Tidak dapat menemukan direktori extensions PT")

        dest = ext_dir / self.EXTENSION_NAME

        # Skip jika sudah ada
        if dest.exists():
            print(f"✅ Extension sudah terpasang: {dest}")
            return True

        # Download
        import urllib.request

        ext_dir.mkdir(parents=True, exist_ok=True)
        print(f"📥 Mengunduh {self.EXTENSION_NAME} ...")
        try:
            urllib.request.urlretrieve(source_url, dest)
        except Exception as exc:
            raise ExtensionError(f"Gagal mengunduh extension: {exc}") from exc

        print(f"✅ Extension terinstal: {dest}")
        return True
