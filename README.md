# qrgen

Small Rust CLI that takes a URL and writes a QR code PNG.

## Usage

```bash
cargo run -- "https://example.com"
```

This writes `qr.png` in the current directory.

Write to a specific file:

```bash
cargo run -- "https://example.com" --output assets/example.png
```

Control the output size:

```bash
cargo run -- "https://example.com" --size 768
```

Disable the standard QR quiet zone:

```bash
cargo run -- "https://example.com" --no-quiet-zone
```

## Build

```bash
cargo build --release
```

Then run:

```bash
./target/release/qrgen "https://example.com" --output code.png
```

Then copy the binary to a directory in your PATH:

```bash
cp target/release/qrgen /usr/local/bin/
```

or add the 'target/release' directory to your PATH: 
```bash
export PATH="$PATH:$(pwd)/target/release"
```