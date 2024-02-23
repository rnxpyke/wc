# WC
A WC implementation in Rust.

Currently only counts newlines, but using SIMD, so it's moderately fast.

For best speed, compile with `RUSTFLAGS="-C target-cpu=native"`

## Usage
```bash
wc $FILE
```
