#!/usr/bin/env -S just --justfile

# Set shell configurations
set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

setup:
    cargo install cargo-binstall
    cargo binstall taplo-cli -y
    
    @echo '✅ Setup complete!'

ready:
  just fmt    
  # just lint 
  @echo '✅ All passed!'

fmt:
    cargo fmt --all -- --emit=files
    taplo fmt **/Cargo.toml
    @echo '✅ Format complete!'

lint: 
    cargo clippy --workspace --all-targets -- --deny warnings
    @echo '✅ Lint complete!'