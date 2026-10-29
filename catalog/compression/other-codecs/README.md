# Zstandard, LZ4, Snappy and Brotli

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md)</sub>

**The default:** flate2, or the C library bindings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**bzip2 0.6 (libbz2-rs-sys Rust backend)**](trifectatechfoundation-bzip2-rs.md) | bzip2 crate 0.6 swapped C libbzip2 for a Rust port: 10-15% fewer cycles to compress, 4-10% fewer to decompress. | C | ✅ Holds up |
| [**snap (pure-Rust Snappy)**](burntsushi-rust-snappy.md) | Pure-Rust Snappy with raw and frame formats; measured within about 10% of C++ Snappy either way on x86_64. | C | ✅ Holds up |
| [**lz4_flex (pure-Rust LZ4)**](pseitz-lz4-flex.md) | Pure-Rust LZ4 that matches the C library when unsafe features are on; the default safe build is up to 20% slower than C. | A | 🟡 Holds up in part |
| [**ruzstd (pure-Rust zstd)**](killingspark-zstd-rs.md) | Pure-Rust zstd decoder and partial encoder; 1.4x to 3.5x slower than the C library behind the zstd crate. | C | ❌ Doesn't hold up |
