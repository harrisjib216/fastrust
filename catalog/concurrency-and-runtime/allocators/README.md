# Memory allocators

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** The system allocator (glibc malloc, musl malloc, or the macOS allocator).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**mimalloc (global allocator)**](purpleprotocol-mimalloc-rust.md) | Rust global allocator for Microsoft's mimalloc: 25x faster than musl's malloc in a 48-core test, 4-54% faster than glibc on most real programs. | A | ✅ Holds up |
| [**snmalloc-rs (snmalloc global allocator)**](microsoft-snmalloc.md) | Rust bindings for Microsoft's message-passing allocator; 7.2x faster than glibc on cross-thread frees and 4-18% faster on real programs. | A | ✅ Holds up |
| [**rpmalloc (rpmalloc-rs global allocator)**](embarkstudios-rpmalloc-rs.md) | Rust wrapper for rpmalloc, a thread-caching allocator; upstream beats glibc on most 2026 benchmarks, but the crate bundles a 2022-era snapshot. | A | 🟡 Holds up in part |
| [**tikv-jemallocator (jemalloc global allocator)**](tikv-jemallocator.md) | Maintained jemalloc bindings that rustc itself uses; 13-14% faster rustc builds on macOS, mixed results against modern glibc. | B | 🟡 Holds up in part |
