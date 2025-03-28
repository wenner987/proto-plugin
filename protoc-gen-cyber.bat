@echo off
setlocal
set CARGO_TARGET_DIR=target
cargo run --quiet -- %*