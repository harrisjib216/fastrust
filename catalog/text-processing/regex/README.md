# Regular expressions

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md)</sub>

**The default:** The regex crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**regex 1.9+ (regex-automata meta engine rewrite)**](rust-lang-regex-regex-1-9-meta-engine.md) | The regex crate's 1.9 rewrite onto regex-automata: the default Rust regex engine searches about 1.5x faster on average than 1.7. | C | ✅ Holds up |
| [**regex-lite (small, fast-building regex)**](rust-lang-regex-regex-lite.md) | Drop-in subset of the regex crate that builds in about 40% of the time and adds about a sixth of the binary size, at the cost of much slower searches. | C | ✅ Holds up |
| [**pcre2 (Rust bindings to PCRE2 with JIT)**](burntsushi-rust-pcre2.md) | Rust bindings to PCRE2; the JIT wins some rebar workloads but is about 2x slower than the regex crate overall. | C | ❌ Doesn't hold up |
