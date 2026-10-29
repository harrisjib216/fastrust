# UTF-8 and Unicode

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md)</sub>

**The default:** std::str::from_utf8 and char iteration.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**simdutf8 (SIMD UTF-8 validation)**](rusticstuff-simdutf8.md) | SIMD drop-in for std::str::from_utf8: up to 23x faster on valid non-ASCII text and up to 4x on ASCII for inputs of 64 bytes or more. | A | ✅ Holds up |
| [**std str::chars().count() word-at-a-time counting (Rust 1.60)**](std-str-chars-count.md) | Rust 1.60 made str::chars().count() about 4x to 5x faster on medium and large strings by counting a machine word at a time. | B | ✅ Holds up |
