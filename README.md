# kdx-rs

An executable to generate `tree.csv` structure for KanadeDX. Ported from Python to Rust for better performance and cross-platform compatibility.

## Usage

```bash
# Run directly with Cargo
cargo run

# Or build and run the binary
cargo build --release
./target/release/kdx-rs
```

## Multi-Platform Builds

This project includes GitHub Actions workflows that automatically build binaries for multiple targets:

- **macOS Apple Silicon** (`aarch64-apple-darwin`)
- **Linux AMD64** (`x86_64-unknown-linux-gnu`) 
- **Linux ARM64** (`aarch64-unknown-linux-gnu`)
- **Windows AMD64** (`x86_64-pc-windows-msvc`)

Binaries are automatically built on:
- Every push to `main`/`master` branch
- Every pull request
- Every release

Download pre-built binaries from the [Releases](../../releases) page or [Actions](../../actions) artifacts.
