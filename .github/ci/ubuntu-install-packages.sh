#!/bin/sh
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    asciidoctor \
    pkg-config \
    openssl1.0 \
    libssl1.0.0 \
    libssl1.0-dev \
    zsh xz-utils liblz4-tool musl-tools
