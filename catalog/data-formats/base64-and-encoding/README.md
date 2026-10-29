# Base64, hex and encoding

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md)</sub>

**The default:** The base64 and hex crates.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**const-hex**](danipopes-const-hex.md) | Drop-in replacement for the hex crate with SIMD and const-eval, measured 6x-99x faster than hex and up to 158x faster than std formatting. | A | ✅ Holds up |
| [**faster-hex**](nervosnetwork-faster-hex.md) | SIMD hex encoding and decoding, measured 6x-71x faster than the hex crate by a third party. | A | ✅ Holds up |
| [**base64 0.23 (Simd engine)**](marshallpierce-rust-base64-simd-engine.md) | The base64 crate's own SIMD engine (0.23+): 3.7x faster encode and up to 7.1x faster decode than its scalar engine, if you opt in. | C | ✅ Holds up |
| [**hex-simd**](nugine-simd-hex-simd.md) | SIMD hex encoder/decoder measured 18x-23x faster than the hex crate on 1-4 KiB inputs by its author. | C | ✅ Holds up |
| [**base64-simd**](nugine-simd-base64-simd.md) | SIMD base64 encoder/decoder; 6x-8.5x faster than the base64 crate in 2023, a gap base64 0.23's own SIMD engine has mostly closed. | C | 🟡 Holds up in part |
