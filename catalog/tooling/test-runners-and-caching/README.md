# Test runners and build caching

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md)</sub>

**The default:** cargo test, and no shared build cache.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**cargo-nextest (test runner)**](nextest-rs-nextest.md) | Test runner that runs every test in its own process from one global pool; 1.4x-3.4x faster than cargo test on nine real projects. | A | ✅ Holds up |
| [**sccache (shared compilation cache)**](mozilla-sccache.md) | Compiler-output cache that wraps rustc; helps clean CI builds of dependencies but cannot cache binaries, proc macros or incremental crates. | D | 🟡 Holds up in part |
