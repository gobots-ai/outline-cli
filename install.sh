#!/usr/bin/env bash
set -euo pipefail

cargo build --release
sudo cp target/release/outline /usr/local/bin/outline
echo "installed outline to /usr/local/bin/outline"
