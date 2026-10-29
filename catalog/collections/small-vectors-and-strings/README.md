# Small vectors and strings

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md)</sub>

**The default:** Vec and String.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std Vec growth skips capacities 1 and 2 (Rust 1.45)**](std-vec-min-non-zero-cap.md) | Since Rust 1.45, a Vec grown by push starts at capacity 4 (8 for bytes) instead of 1, removing two reallocations for small Vecs. | B | ✅ Holds up |
| [**smol_str (inline immutable string)**](rust-lang-rust-analyzer-smol-str.md) | Immutable string type that stores up to 23 bytes inline and clones in O(1); far fewer allocations than String for short tokens. | B | 🟡 Holds up in part |
| [**compact_str (CompactString, 24-byte inline string)**](parkmycar-compact-str.md) | Mutable String replacement that stores strings up to 24 bytes inline in the same 24 bytes; avoids allocations for short strings. | D | 🟡 Holds up in part |
| [**smartstring (SmartString inline string)**](bodil-smartstring.md) | Archived inline-string String replacement; it saves allocations, but an independent benchmark drops it from access tests for being slow. | D | ❌ Doesn't hold up |
