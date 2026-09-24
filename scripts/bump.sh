#!/usr/bin/env bash
# Set the extension's version to a Navigator release tag.
#
# The extension carries Navigator's version rather than its own so a reader can
# tell at a glance which release it was cut beside, the same way navigator-ux
# does. `navigator-lsp` itself is still resolved at runtime, so this number
# names the shim, not the language server it downloads.
set -euo pipefail

tag="${1:?usage: scripts/bump.sh <YY.M.D | YY.M.D-hotfix.N | YY.M.D-rc.N>}"
if ! printf '%s' "${tag}" |
    grep -Eq '^[0-9]{2}\.(0|[1-9][0-9]?)\.(0|[1-9][0-9]?)(-(hotfix|rc)\.(0|[1-9][0-9]*))?$'; then
    echo "bump.sh: '${tag}' is not a YY.M.D, YY.M.D-hotfix.N, or YY.M.D-rc.N release version" >&2
    exit 1
fi

cd "$(dirname "$0")/.."
# The first `version = ` line in each file is the package's own; `-i.bak` is the
# spelling GNU and BSD sed both accept.
for file in extension.toml Cargo.toml; do
    sed -i.bak "1,/^version = /s/^version = \".*\"/version = \"${tag}\"/" "${file}"
    rm "${file}.bak"
    grep -qx "version = \"${tag}\"" "${file}" || { echo "bump.sh: ${file} did not take ${tag}" >&2; exit 1; }
done
# Carry the new version into Cargo.lock without touching any dependency.
cargo update --workspace --offline >/dev/null 2>&1 || cargo update --workspace
echo "extension version is now ${tag}"
