# Deflate, zlib and gzip

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Compression](../README.md)</sub>

**The default:** flate2 with its default miniz_oxide backend.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**zlib-rs (flate2 zlib-rs backend)**](trifectatechfoundation-zlib-rs.md) | Rust port of zlib-ng as a flate2 backend; inflates about 1.3x and deflates 1.9x to 2.6x faster than flate2's default miniz_oxide. | A | ✅ Holds up |
| [**fdeflate (safe-Rust inflate, PNG-tuned)**](image-rs-fdeflate.md) | Safe-Rust deflate decoder from image-rs; inflated 1.5x to 1.6x faster than miniz_oxide in a flate2 prototype. | A | 🟡 Holds up in part |
| [**zlib-ng via flate2 (libz-ng-sys backend)**](rust-lang-libz-sys-zlib-ng.md) | flate2's zlib-ng C backend: level-6 deflate 2.1x to 2.7x faster than miniz_oxide; inflate gains are small to none. | A | 🟡 Holds up in part |
| [**libdeflater (libdeflate bindings)**](libdeflater-libdeflater.md) | Bindings to libdeflate for whole-buffer gzip/zlib; 2019 README shows 2-3x over flate2's old C miniz default. | C | 🟡 Holds up in part |
