#!/bin/sh
# @impl:FR-CLI-18
# One-liner installer for lab-report on Linux and macOS
# Usage: curl -fsSL https://codeberg.org/civitas-project/lab-report-template/raw/branch/main/install.sh | sh

set -e

REPO="civitas-project/lab-report-template"
BASE_URL="https://codeberg.org/${REPO}"

echo "==> Installing lab-report CLI from Codeberg..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)
    TARGET_OS="linux"
    ;;
  darwin)
    TARGET_OS="darwin"
    ;;
  *)
    echo "Error: Unsupported operating system: $OS" >&2
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    TARGET_ARCH="x86_64"
    ;;
  aarch64|arm64)
    TARGET_ARCH="aarch64"
    ;;
  *)
    echo "Error: Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

# Default fallback binary download
ARCHIVE_NAME="lab-report-linux-x86_64.tar.gz"
DOWNLOAD_URL="${BASE_URL}/releases/download/latest/${ARCHIVE_NAME}"

# Target install directory
if [ -d "$HOME/.local/bin" ]; then
  INSTALL_DIR="$HOME/.local/bin"
elif [ -w "/usr/local/bin" ]; then
  INSTALL_DIR="/usr/local/bin"
else
  INSTALL_DIR="$HOME/.cargo/bin"
  mkdir -p "$INSTALL_DIR"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "--> Downloading release archive from ${DOWNLOAD_URL}..."
if curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE_NAME" 2>/dev/null; then
  echo "--> Verifying SHA-256 checksum..."
  if curl -fsSL "${DOWNLOAD_URL}.sha256" -o "$TMP_DIR/$ARCHIVE_NAME.sha256" 2>/dev/null; then
    (cd "$TMP_DIR" && {
      if command -v sha256sum >/dev/null 2>&1; then
        sha256sum -c "$ARCHIVE_NAME.sha256"
      elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 -c "$ARCHIVE_NAME.sha256"
      else
        echo "Warning: sha256sum/shasum not found, skipping checksum verification."
      fi
    })
  else
    echo "Warning: Checksum file not found, proceeding with caution."
  fi
  tar -xzf "$TMP_DIR/$ARCHIVE_NAME" -C "$TMP_DIR"
  mv "$TMP_DIR/lab-report" "$INSTALL_DIR/lab-report"
else
  echo "Note: Prebuilt release asset not yet found at ${DOWNLOAD_URL}."
  echo "--> Falling back to cargo install if Rust is available..."
  if command -v cargo >/dev/null 2>&1; then
    cargo install --git "https://codeberg.org/${REPO}.git" --bin lab-report
    echo ">> Successfully installed via cargo!"
    exit 0
  else
    echo "Error: Prebuilt binary not available and cargo is not installed." >&2
    echo "Please visit ${BASE_URL} to download releases directly." >&2
    exit 1
  fi
fi

chmod +x "$INSTALL_DIR/lab-report"
echo ">> Successfully installed lab-report to $INSTALL_DIR/lab-report!"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo ""
  echo "⚠️  $INSTALL_DIR is not in your PATH."
  echo "   Add it by running:"
  echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "🎉 Run 'lab-report init' to start setup wizard!"
