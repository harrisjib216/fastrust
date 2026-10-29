# Compression

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Deflate, zlib and gzip](deflate-and-gzip/README.md)

**The default:** flate2 with its default miniz_oxide backend.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**zlib-rs (flate2 zlib-rs backend)**](deflate-and-gzip/trifectatechfoundation-zlib-rs.md) | Rust port of zlib-ng as a flate2 backend; inflates about 1.3x and deflates 1.9x to 2.6x faster than flate2's default miniz_oxide. | A | ✅ Holds up |
| [**fdeflate (safe-Rust inflate, PNG-tuned)**](deflate-and-gzip/image-rs-fdeflate.md) | Safe-Rust deflate decoder from image-rs; inflated 1.5x to 1.6x faster than miniz_oxide in a flate2 prototype. | A | 🟡 Holds up in part |
| [**zlib-ng via flate2 (libz-ng-sys backend)**](deflate-and-gzip/rust-lang-libz-sys-zlib-ng.md) | flate2's zlib-ng C backend: level-6 deflate 2.1x to 2.7x faster than miniz_oxide; inflate gains are small to none. | A | 🟡 Holds up in part |
| [**libdeflater (libdeflate bindings)**](deflate-and-gzip/libdeflater-libdeflater.md) | Bindings to libdeflate for whole-buffer gzip/zlib; 2019 README shows 2-3x over flate2's old C miniz default. | C | 🟡 Holds up in part |

## [Zstandard, LZ4, Snappy and Brotli](other-codecs/README.md)

**The default:** flate2, or the C library bindings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**bzip2 0.6 (libbz2-rs-sys Rust backend)**](other-codecs/trifectatechfoundation-bzip2-rs.md) | bzip2 crate 0.6 swapped C libbzip2 for a Rust port: 10-15% fewer cycles to compress, 4-10% fewer to decompress. | C | ✅ Holds up |
| [**snap (pure-Rust Snappy)**](other-codecs/burntsushi-rust-snappy.md) | Pure-Rust Snappy with raw and frame formats; measured within about 10% of C++ Snappy either way on x86_64. | C | ✅ Holds up |
| [**lz4_flex (pure-Rust LZ4)**](other-codecs/pseitz-lz4-flex.md) | Pure-Rust LZ4 that matches the C library when unsafe features are on; the default safe build is up to 20% slower than C. | A | 🟡 Holds up in part |
| [**ruzstd (pure-Rust zstd)**](other-codecs/killingspark-zstd-rs.md) | Pure-Rust zstd decoder and partial encoder; 1.4x to 3.5x slower than the C library behind the zstd crate. | C | ❌ Doesn't hold up |
