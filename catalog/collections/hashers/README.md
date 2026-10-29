# Hashers for maps and sets

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md)</sub>

**The default:** std HashMap's RandomState (SipHash-1-3).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**foldhash (folded-multiply hasher, hashbrown's default)**](orlp-foldhash.md) | Portable, seeded hasher built on a folded multiply; about 5x faster than SipHash-1-3 across hash-table workloads. | A | ✅ Holds up |
| [**rapidhash (wyhash successor)**](hoxxep-rapidhash.md) | Portable rapidhash/wyhash-family hasher with randomized secrets; about 5x faster than SipHash-1-3 and slightly ahead of foldhash. | A | ✅ Holds up |
| [**aHash (AES-based keyed hasher)**](tkaitchuck-ahash.md) | Keyed, randomly seeded hasher for HashMap that uses AES instructions where available; several times faster than SipHash-1-3. | A | 🟡 Holds up in part |
| [**gxhash (AES/SIMD hasher)**](ogxd-gxhash.md) | AES- and SIMD-based hasher that is fastest on long keys but slower than foldhash or rapidhash on integers, and needs AES at build time. | B | 🟡 Holds up in part |
| [**rustc-hash (FxHasher, rewritten in 2.0)**](rust-lang-rustc-hash.md) | The rustc compiler's unkeyed FxHasher; fastest on integer keys, with no DoS resistance and known collision patterns. | B | 🟡 Holds up in part |
