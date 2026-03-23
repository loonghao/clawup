#!/bin/sh
# clawup installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/loonghao/clawup/main/install.sh | sh
#
# Environment variables:
#   CLAWUP_VERSION  - Specific version to install (e.g., "0.1.9"). Default: latest
#   CLAWUP_INSTALL  - Installation directory. Default: $HOME/.clawup/bin
#   CLAWUP_MUSL     - Set to "1" to prefer musl build on Linux. Default: auto-detect

set -eu

REPO="loonghao/clawup"
BINARY_NAME="clawup"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    printf "${BLUE}info${NC}: %s\n" "$1"
}

success() {
    printf "${GREEN}success${NC}: %s\n" "$1"
}

warn() {
    printf "${YELLOW}warn${NC}: %s\n" "$1"
}

error() {
    printf "${RED}error${NC}: %s\n" "$1" >&2
    exit 1
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *) error "Unsupported operating system: $(uname -s)" ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)  echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *) error "Unsupported architecture: $(uname -m)" ;;
    esac
}

# Detect if using musl libc
detect_musl() {
    if [ "${CLAWUP_MUSL:-}" = "1" ]; then
        return 0
    fi
    if command -v ldd >/dev/null 2>&1; then
        if ldd --version 2>&1 | grep -qi musl; then
            return 0
        fi
    fi
    # Check if running on Alpine or similar musl-based distro
    if [ -f /etc/os-release ]; then
        if grep -qi "alpine\|void" /etc/os-release; then
            return 0
        fi
    fi
    return 1
}

# Build the Rust target triple
get_target() {
    os="$1"
    arch="$2"

    case "$os" in
        linux)
            if detect_musl; then
                echo "${arch}-unknown-linux-musl"
            else
                echo "${arch}-unknown-linux-gnu"
            fi
            ;;
        macos)
            echo "${arch}-apple-darwin"
            ;;
        windows)
            echo "${arch}-pc-windows-msvc"
            ;;
    esac
}

# Get the download URL for the release
get_download_url() {
    version="$1"
    target="$2"

    case "$target" in
        *windows*)
            ext="zip"
            ;;
        *)
            ext="tar.gz"
            ;;
    esac

    if [ -z "$version" ] || [ "$version" = "latest" ]; then
        # Get latest release tag
        version=$(get_latest_version)
    fi

    echo "https://github.com/${REPO}/releases/download/v${version}/${BINARY_NAME}-${target}.${ext}"
}

# Get the latest release version
get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | sed -E 's/.*"v([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | sed -E 's/.*"v([^"]+)".*/\1/'
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Download a file
download() {
    url="$1"
    output="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$output" "$url"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
}

# Check if a command exists
check_cmd() {
    command -v "$1" >/dev/null 2>&1
}

main() {
    os=$(detect_os)
    arch=$(detect_arch)
    target=$(get_target "$os" "$arch")

    info "Detected platform: ${os} ${arch} (${target})"

    # Determine version
    version="${CLAWUP_VERSION:-latest}"
    if [ "$version" = "latest" ]; then
        info "Fetching latest version..."
        version=$(get_latest_version)
    fi

    if [ -z "$version" ]; then
        error "Failed to determine version. Set CLAWUP_VERSION or check your network connection."
    fi

    info "Installing clawup v${version}..."

    # Determine install directory
    install_dir="${CLAWUP_INSTALL:-$HOME/.clawup/bin}"
    mkdir -p "$install_dir"

    # Check for existing installation
    old_version=""
    if [ -f "${install_dir}/${BINARY_NAME}" ]; then
        old_version=$("${install_dir}/${BINARY_NAME}" --version 2>/dev/null | head -1 | awk '{print $2}' || true)
        if [ -n "$old_version" ]; then
            info "Found existing installation: clawup v${old_version}"
        fi
    fi

    # Create temp directory
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    # Determine file extension
    case "$target" in
        *windows*)
            archive_name="${BINARY_NAME}-${target}.zip"
            ;;
        *)
            archive_name="${BINARY_NAME}-${target}.tar.gz"
            ;;
    esac

    download_url="https://github.com/${REPO}/releases/download/v${version}/${archive_name}"
    archive_path="${tmp_dir}/${archive_name}"

    info "Downloading ${download_url}..."
    download "$download_url" "$archive_path" || error "Download failed. Check if v${version} has pre-built binaries for ${target}."

    # Verify SHA256 checksum if available
    checksums_url="https://github.com/${REPO}/releases/download/v${version}/checksums-sha256.txt"
    checksums_path="${tmp_dir}/checksums-sha256.txt"
    if download "$checksums_url" "$checksums_path" 2>/dev/null; then
        info "Verifying SHA256 checksum..."
        # Extract expected hash — supports both standard "hash  filename" and legacy "hashfilename"
        checksum_line=$(grep "${archive_name}" "$checksums_path" | head -1)
        if echo "$checksum_line" | grep -q '  '; then
            # Standard sha256sum format: "<hash>  <filename>"
            expected_hash=$(echo "$checksum_line" | awk '{print $1}')
        else
            # Legacy format: hash directly concatenated with filename (no separator)
            expected_hash=$(echo "$checksum_line" | sed "s/${archive_name}//")
        fi
        if [ -n "$expected_hash" ]; then
            if check_cmd sha256sum; then
                actual_hash=$(sha256sum "$archive_path" | awk '{print $1}')
            elif check_cmd shasum; then
                actual_hash=$(shasum -a 256 "$archive_path" | awk '{print $1}')
            else
                warn "Neither sha256sum nor shasum found, skipping checksum verification"
                actual_hash=""
            fi

            if [ -n "$actual_hash" ]; then
                if [ "$actual_hash" = "$expected_hash" ]; then
                    success "Checksum verified ✓"
                else
                    error "Checksum mismatch! Expected: ${expected_hash}, Got: ${actual_hash}"
                fi
            fi
        else
            warn "Checksum not found for ${archive_name}, skipping verification"
        fi
    else
        warn "Checksums file not available, skipping verification"
    fi

    # Extract
    info "Extracting..."
    case "$archive_name" in
        *.tar.gz)
            tar xzf "$archive_path" -C "$tmp_dir"
            ;;
        *.zip)
            if check_cmd unzip; then
                unzip -qo "$archive_path" -d "$tmp_dir"
            else
                error "unzip is required to extract .zip archives"
            fi
            ;;
    esac

    # Find the binary
    binary_path=""
    if [ -f "${tmp_dir}/${BINARY_NAME}" ]; then
        binary_path="${tmp_dir}/${BINARY_NAME}"
    elif [ -f "${tmp_dir}/${BINARY_NAME}.exe" ]; then
        binary_path="${tmp_dir}/${BINARY_NAME}.exe"
    else
        # Search recursively
        binary_path=$(find "$tmp_dir" -name "${BINARY_NAME}" -o -name "${BINARY_NAME}.exe" | head -1)
    fi

    if [ -z "$binary_path" ]; then
        error "Could not find ${BINARY_NAME} binary in the downloaded archive"
    fi

    # Install
    chmod +x "$binary_path"
    mv "$binary_path" "${install_dir}/${BINARY_NAME}"

    success "clawup v${version} installed to ${install_dir}/${BINARY_NAME}"

    # Show upgrade info if applicable
    if [ -n "$old_version" ] && [ "$old_version" != "$version" ]; then
        success "Upgraded: v${old_version} → v${version}"
    fi

    # Check if install_dir is in PATH
    case ":${PATH}:" in
        *":${install_dir}:"*)
            ;;
        *)
            echo ""
            warn "\"${install_dir}\" is not in your PATH."
            echo ""
            echo "  Add it to your shell profile:"
            echo ""
            if [ -n "${ZSH_VERSION:-}" ] || [ "$(basename "${SHELL:-}")" = "zsh" ]; then
                echo "    echo 'export PATH=\"${install_dir}:\$PATH\"' >> ~/.zshrc"
                echo "    source ~/.zshrc"
            elif [ -n "${BASH_VERSION:-}" ] || [ "$(basename "${SHELL:-}")" = "bash" ]; then
                echo "    echo 'export PATH=\"${install_dir}:\$PATH\"' >> ~/.bashrc"
                echo "    source ~/.bashrc"
            else
                echo "    export PATH=\"${install_dir}:\$PATH\""
            fi
            echo ""
            ;;
    esac

    # Verify installation
    if "${install_dir}/${BINARY_NAME}" --version >/dev/null 2>&1; then
        installed_version=$("${install_dir}/${BINARY_NAME}" --version 2>&1 | head -1)
        success "Verified: ${installed_version}"
    fi

    echo ""
    info "Run 'clawup --help' to get started"
}

main "$@"
