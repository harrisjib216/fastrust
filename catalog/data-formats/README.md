# Data formats

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [JSON](json/README.md)

**The default:** serde_json.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**flexon (SIMD JSON parser)**](json/cyruspyre-flexon.md) | Young SIMD JSON parser with serde support; an independent benchmark has it deserializing 1.5x-1.9x faster than serde_json. | A | ✅ Holds up |
| [**simd-json (SIMD JSON parser with a serde-style API)**](json/simd-lite-simd-json.md) | Rust port of simdjson with serde support; in an independent benchmark it deserializes 1.2x-1.5x and serializes about 1.6x faster than serde_json. | A | ✅ Holds up |
| [**sonic-rs (SIMD JSON parser and serializer)**](json/cloudwego-sonic-rs.md) | ByteDance's SIMD JSON library; its own runs parse 1.7x-2.3x (structs) and 3.4x-5.2x (untyped) faster than serde_json with target-cpu=native. | A | 🟡 Holds up in part |
| [**json-rust (json crate, DOM parser)**](json/maciejhirsz-json-rust.md) | Untyped JSON DOM crate that beat serde_json's Value in a 2021 run but not serde_json struct parsing; unmaintained since 2020. | A | ❌ Doesn't hold up |

## [Binary serialization](binary-serialization/README.md)

**The default:** bincode through serde, or serde_json as the wire or disk format.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**bitcode (compact binary Encode/Decode)**](binary-serialization/softbearstudios-bitcode.md) | Binary format with its own derive that serializes 4x-6x and deserializes 1.4x-2.9x faster than bincode 1.3 via serde, with smaller output. | A | ✅ Holds up |
| [**rkyv (zero-copy deserialization)**](binary-serialization/rkyv-rkyv.md) | Zero-copy archive format: reads data in place in nanoseconds instead of deserializing, and serializes plain-data arrays 35x faster than bincode 1.3. | A | ✅ Holds up |
| [**wincode (bincode-compatible serializer)**](binary-serialization/anza-xyz-wincode.md) | Drop-in for bincode 1's wire format that serializes 3x-33x and deserializes 1.1x-26x faster than bincode 1.3.3 through serde. | A | ✅ Holds up |
| [**speedy (binary serialization)**](binary-serialization/koute-speedy.md) | Fast custom-derive binary serializer, 3x-34x faster than bincode 1.3 to serialize, but unmaintained and dropped from the main benchmark. | A | 🟡 Holds up in part |
| [**borsh (Binary Object Representation Serializer for Hashing)**](binary-serialization/near-borsh-rs.md) | Canonical binary format from NEAR for hashing and consensus; benchmarks at the same speed as bincode 1.3, not faster. | A | ❌ Doesn't hold up |

## [Base64, hex and encoding](base64-and-encoding/README.md)

**The default:** The base64 and hex crates.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**const-hex**](base64-and-encoding/danipopes-const-hex.md) | Drop-in replacement for the hex crate with SIMD and const-eval, measured 6x-99x faster than hex and up to 158x faster than std formatting. | A | ✅ Holds up |
| [**faster-hex**](base64-and-encoding/nervosnetwork-faster-hex.md) | SIMD hex encoding and decoding, measured 6x-71x faster than the hex crate by a third party. | A | ✅ Holds up |
| [**base64 0.23 (Simd engine)**](base64-and-encoding/marshallpierce-rust-base64-simd-engine.md) | The base64 crate's own SIMD engine (0.23+): 3.7x faster encode and up to 7.1x faster decode than its scalar engine, if you opt in. | C | ✅ Holds up |
| [**hex-simd**](base64-and-encoding/nugine-simd-hex-simd.md) | SIMD hex encoder/decoder measured 18x-23x faster than the hex crate on 1-4 KiB inputs by its author. | C | ✅ Holds up |
| [**base64-simd**](base64-and-encoding/nugine-simd-base64-simd.md) | SIMD base64 encoder/decoder; 6x-8.5x faster than the base64 crate in 2023, a gap base64 0.23's own SIMD engine has mostly closed. | C | 🟡 Holds up in part |

## [Numbers to and from text](numbers-to-and-from-text/README.md)

**The default:** std's Display/format! and str::parse.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**ryu (Ryū float-to-string)**](numbers-to-and-from-text/dtolnay-ryu.md) | Ryū shortest float-to-string: 2.2x-2.4x faster than std's float Display, now overtaken by zmij. | A | ✅ Holds up |
| [**std float parsing (Eisel-Lemire, Rust 1.55)**](numbers-to-and-from-text/std-num-f64-from-str.md) | Since Rust 1.55, str::parse::<f64> uses Eisel-Lemire and runs within about 1.2x of fast-float, down from 7x-8x behind. | A | ✅ Holds up |
| [**zmij (Żmij float-to-string)**](numbers-to-and-from-text/dtolnay-zmij.md) | Rust port of Victor Zverovich's Żmij float printer: 8x-11x faster than std's float Display and about 4x faster than ryu. | A | ✅ Holds up |
| [**lexical-core (number parsing and formatting)**](numbers-to-and-from-text/alexhuszagh-rust-lexical.md) | Configurable number parsing and formatting; 2.8x faster than std at printing floats, but no longer the fastest and only 1.1x-1.5x ahead at parsing. | A | 🟡 Holds up in part |
| [**itoa (integer-to-string)**](numbers-to-and-from-text/dtolnay-itoa.md) | Integer-to-decimal formatting without core::fmt::Formatter; std 1.98's NumBuffer::format_into now offers the same path built in. | D | 🟡 Holds up in part |
| [**fast-float (Eisel-Lemire float parsing)**](numbers-to-and-from-text/aldanor-fast-float-rust.md) | Port of Lemire's fast_float that beat std's float parsing 2-8x on Rust 1.49; std absorbed the algorithm in 1.55 and the gap is now 1.0x-1.3x. | A | ❌ Doesn't hold up |

## [Parsers and CSV](parsers-and-csv/README.md)

**The default:** The csv crate, nom, and hand-written str::split parsing.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**simd-csv (SIMD CSV reader)**](parsers-and-csv/medialab-simd-csv.md) | CSV reader that pairs a state machine with SIMD byte searching; its author measures up to 4x-6x over the csv crate on long-cell data. | C | ✅ Holds up |
| [**winnow (parser combinators)**](parsers-and-csv/winnow-rs-winnow.md) | Fork of nom that parsed canada.json 2.5x faster and built 3x faster than nom 8 in its author's comparison suite. | C | 🟡 Holds up in part |
