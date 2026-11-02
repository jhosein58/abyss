#!/usr/bin/env bash
set -e

REPO="jhosein58/abyss"
VERSION="${ABYSS_VERSION:-latest}"
INSTALL_DIR="${ABYSS_INSTALL_DIR:-$HOME/.abyss}"
BIN_DIR="${INSTALL_DIR}/bin"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "${OS}" != "linux" ] || [ "${ARCH}" != "x86_64" ]; then
    echo "[error] currently only linux-x86_64 is supported by this installer." >&2
    exit 1
fi

if [ "${VERSION}" = "latest" ]; then
    TAR_URL="https://github.com/${REPO}/releases/latest/download/abyss-linux-x86_64.tar.gz"
else
    TAR_URL="https://github.com/${REPO}/releases/download/${VERSION}/abyss-linux-x86_64.tar.gz"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

echo "Downloading Abyss toolchain from ${TAR_URL}..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${TAR_URL}" -o "${TMP_DIR}/abyss.tar.gz"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "${TMP_DIR}/abyss.tar.gz" "${TAR_URL}"
else
    echo "[error] curl or wget is required." >&2
    exit 1
fi

mkdir -p "${INSTALL_DIR}"
rm -rf "${INSTALL_DIR}/bin" "${INSTALL_DIR}/pods"
tar -xzf "${TMP_DIR}/abyss.tar.gz" -C "${INSTALL_DIR}" --strip-components=1

chmod +x "${BIN_DIR}/abyssc" 2>/dev/null || true
chmod +x "${BIN_DIR}/sonar" 2>/dev/null || true

add_to_profile() {
    PROFILE_FILE="$1"
    LINE='export PATH="'"${BIN_DIR}"':$PATH"'
    if [ -f "${PROFILE_FILE}" ] && ! grep -q "${BIN_DIR}" "${PROFILE_FILE}"; then
        echo "" >> "${PROFILE_FILE}"
        echo "${LINE}" >> "${PROFILE_FILE}"
        echo "Updated ${PROFILE_FILE}"
    fi
}

add_to_profile "${HOME}/.bashrc"
add_to_profile "${HOME}/.zshrc"
add_to_profile "${HOME}/.profile"

echo ""
echo "✔ Abyss & Sonar installed successfully to ${INSTALL_DIR}!"
echo "Binaries available in: ${BIN_DIR}"
echo "Restart your terminal or run: export PATH=\"${BIN_DIR}:\$PATH\""
