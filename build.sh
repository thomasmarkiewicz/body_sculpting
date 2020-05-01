#!/bin/sh
export PKG_CONFIG_ALLOW_CROSS=1
cargo build --release --target=aarch64-unknown-linux-gnu
flatpak-builder --force-clean --arch=aarch64 --repo=myrepo _flatpak app.sculpting.body.json
flatpak build-bundle --arch=aarch64 myrepo bodysculpting.flatpak app.sculpting.body
