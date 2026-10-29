# Text processing

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Regular expressions](regex/README.md)

**The default:** The regex crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**regex 1.9+ (regex-automata meta engine rewrite)**](regex/rust-lang-regex-regex-1-9-meta-engine.md) | The regex crate's 1.9 rewrite onto regex-automata: the default Rust regex engine searches about 1.5x faster on average than 1.7. | C | ✅ Holds up |
| [**regex-lite (small, fast-building regex)**](regex/rust-lang-regex-regex-lite.md) | Drop-in subset of the regex crate that builds in about 40% of the time and adds about a sixth of the binary size, at the cost of much slower searches. | C | ✅ Holds up |
| [**pcre2 (Rust bindings to PCRE2 with JIT)**](regex/burntsushi-rust-pcre2.md) | Rust bindings to PCRE2; the JIT wins some rebar workloads but is about 2x slower than the regex crate overall. | C | ❌ Doesn't hold up |

## [String and byte search](string-search/README.md)

**The default:** str::find, contains and split.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std str::contains SSE2 fast path for short needles (Rust 1.67)**](string-search/std-str-contains-sse2.md) | Rust 1.67 made str::contains with short needles 6x to 9x faster in long-haystack benchmarks on x86_64 by replacing Two-Way with an SSE2 scan. | B | ✅ Holds up |
| [**memchr (memchr::memmem substring search)**](string-search/burntsushi-memchr.md) | SIMD substring search for &[u8] that runs about 5x to 6x faster than std's str::find across 53 benchmarks. | C | ✅ Holds up |
| [**bstr (byte strings with reusable Finder and line iteration)**](string-search/burntsushi-bstr.md) | Byte-string extension traits for &[u8]; a line-by-line grep ran 2.8x faster than String plus str::contains, measured before Rust 1.67. | C | 🟡 Holds up in part |

## [UTF-8 and Unicode](utf8-and-unicode/README.md)

**The default:** std::str::from_utf8 and char iteration.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**simdutf8 (SIMD UTF-8 validation)**](utf8-and-unicode/rusticstuff-simdutf8.md) | SIMD drop-in for std::str::from_utf8: up to 23x faster on valid non-ASCII text and up to 4x on ASCII for inputs of 64 bytes or more. | A | ✅ Holds up |
| [**std str::chars().count() word-at-a-time counting (Rust 1.60)**](utf8-and-unicode/std-str-chars-count.md) | Rust 1.60 made str::chars().count() about 4x to 5x faster on medium and large strings by counting a machine word at a time. | B | ✅ Holds up |
