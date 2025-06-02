#!/bin/bash
# scripts/run_dev.sh
set -e

echo "Building workspace..."
cargo build --workspace

echo "Running xelarius-node..."
cargo run -p xelarius-node
