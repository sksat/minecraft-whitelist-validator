#!/usr/bin/env bash
set -e
set -o pipefail

cd /app
./minecraft-whitelist-validator "$1"
