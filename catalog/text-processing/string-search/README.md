# String and byte search

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md)</sub>

**The default:** str::find, contains and split.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std str::contains SSE2 fast path for short needles (Rust 1.67)**](std-str-contains-sse2.md) | Rust 1.67 made str::contains with short needles 6x to 9x faster in long-haystack benchmarks on x86_64 by replacing Two-Way with an SSE2 scan. | B | ✅ Holds up |
| [**memchr (memchr::memmem substring search)**](burntsushi-memchr.md) | SIMD substring search for &[u8] that runs about 5x to 6x faster than std's str::find across 53 benchmarks. | C | ✅ Holds up |
| [**bstr (byte strings with reusable Finder and line iteration)**](burntsushi-bstr.md) | Byte-string extension traits for &[u8]; a line-by-line grep ran 2.8x faster than String plus str::contains, measured before Rust 1.67. | C | 🟡 Holds up in part |
