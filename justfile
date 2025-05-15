#!/usr/bin/env -S just --justfile

# Set shell configurations
set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

setup:
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    cargo binstall taplo-cli cargo-release watchexec-cli@2.2.1 -y
    snm i -f
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

watch:
    echo "Running the project..."
    # cargo watch -x build
    watchexec -r -e rs cargo build

# release-patch:
#     cargo release patch --no-push --no-publish --execute
#     @echo '✅ Release patch complete!'

# release-minor:
#     cargo release minor --no-push --no-publish --execute
#     @echo '✅ Release minor complete!'

# release-major:
#     cargo release major --no-push --no-publish --execute
#     @echo '✅ Release major complete!'