# JSON

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md)</sub>

**The default:** serde_json.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**flexon (SIMD JSON parser)**](cyruspyre-flexon.md) | Young SIMD JSON parser with serde support; an independent benchmark has it deserializing 1.5x-1.9x faster than serde_json. | A | ✅ Holds up |
| [**simd-json (SIMD JSON parser with a serde-style API)**](simd-lite-simd-json.md) | Rust port of simdjson with serde support; in an independent benchmark it deserializes 1.2x-1.5x and serializes about 1.6x faster than serde_json. | A | ✅ Holds up |
| [**sonic-rs (SIMD JSON parser and serializer)**](cloudwego-sonic-rs.md) | ByteDance's SIMD JSON library; its own runs parse 1.7x-2.3x (structs) and 3.4x-5.2x (untyped) faster than serde_json with target-cpu=native. | A | 🟡 Holds up in part |
| [**json-rust (json crate, DOM parser)**](maciejhirsz-json-rust.md) | Untyped JSON DOM crate that beat serde_json's Value in a 2021 run but not serde_json struct parsing; unmaintained since 2020. | A | ❌ Doesn't hold up |
