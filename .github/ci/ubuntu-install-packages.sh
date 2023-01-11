#!/bin/sh
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    asciidoctor \
    libssl-dev \
    zsh xz-utils liblz4-tool musl-tools
