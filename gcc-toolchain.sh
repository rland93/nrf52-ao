#!/usr/bin/env bash
# toolchain download
#
# download a pinned arm-none-eabi-gcc toolchain into ./toolchain/ and
# verify its integrity before installation
#
# the first time it runs it will print the hash for manual verification

set -euo pipefail

# version of the gnu toolchain to install
VERSION="13.3.rel1"

# pin the SHA256 after verifying against https://developer.arm.com/downloads
#
# leaving empty will cause the script to print the hash and exit.
PINNED_SHA256_DARWIN_ARM64="fb6921db95d345dc7e5e487dd43b745e3a5b4d5c0c7ca4f707347148760317b4"
PINNED_SHA256_DARWIN_X86_64=""
PINNED_SHA256_LINUX_X86_64=""
PINNED_SHA256_LINUX_ARM64=""

# determine os and architecture
#
# tarball_arch - constructs the url
# pinned - the hash for this platform
os="$(uname -s)"
arch="$(uname -m)"
case "${os}-${arch}" in
    Darwin-arm64)  tarball_arch="darwin-arm64";   pinned="${PINNED_SHA256_DARWIN_ARM64}"  ;;
    Darwin-x86_64) tarball_arch="darwin-x86_64";  pinned="${PINNED_SHA256_DARWIN_X86_64}" ;;
    Linux-x86_64)  tarball_arch="x86_64";         pinned="${PINNED_SHA256_LINUX_X86_64}"  ;;
    Linux-aarch64) tarball_arch="aarch64";        pinned="${PINNED_SHA256_LINUX_ARM64}"   ;;
    *)
        echo "Unsupported platform: ${os}-${arch}" >&2
        exit 1
        ;;
esac

# name of the tarball
tarball="arm-gnu-toolchain-${VERSION}-${tarball_arch}-arm-none-eabi.tar.xz"

# url to the tarball
url="https://developer.arm.com/-/media/Files/downloads/gnu/${VERSION}/binrel/${tarball}"

# absolute path to the project root
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# we download into toolchain/
dest="${here}/toolchain"

# don't download if we already have the toolchain
if [[ -x "${dest}/bin/arm-none-eabi-gcc" ]]; then
    echo "Toolchain already installed:"
    "${dest}/bin/arm-none-eabi-gcc" --version | head -1
    exit 0
fi

# temp dir for downloads to go into
tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT

# do the download
echo "Downloading ${tarball}..."
curl -fL "${url}" -o "${tmp}/${tarball}"

# verify sha
echo "Fetching ARM's published SHA256..."
if curl -fL "${url}.sha256asc" -o "${tmp}/${tarball}.sha256asc" 2>/dev/null; then
    (cd "${tmp}" && shasum -a 256 -c "${tarball}.sha256asc")
else
    echo "  (no .sha256asc available — relying on pinned hash only)"
fi

computed="$(shasum -a 256 "${tmp}/${tarball}" | awk '{print $1}')"

if [[ -z "${pinned}" ]]; then
    echo
    echo "No pinned SHA256 for ${os}-${arch}."
    echo "Computed: ${computed}"
    echo "Verify this against https://developer.arm.com/downloads and then set"
    echo "PINNED_SHA256_${os^^}_${arch^^} in this script before re-running."
    exit 1
fi

if [[ "${computed}" != "${pinned}" ]]; then
    echo "SHA256 mismatch!" >&2
    echo "  expected: ${pinned}" >&2
    echo "  got:      ${computed}" >&2
    exit 1
fi

# extract downloaded
echo "Extracting into ${dest}..."
mkdir -p "${dest}"
tar -xJf "${tmp}/${tarball}" -C "${dest}" --strip-components=1

echo "Installed:"
"${dest}/bin/arm-none-eabi-gcc" --version | head -1
