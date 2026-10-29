# In-memory caches

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md)</sub>

**The default:** A HashMap with hand-written eviction, or the lru crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**quick_cache (S3-FIFO concurrent cache)**](arthurprs-quick-cache.md) | Sharded S3-FIFO cache: up to 5.5x the hit ratio of Mutex<LRU> at similar per-op cost, about 4x faster than Mini Moka. | C | ✅ Holds up |
| [**moka (TinyLFU concurrent cache, with Mini Moka)**](moka-rs-moka.md) | Caffeine-style cache with TTL and listeners; 4.5x the hit ratio of Mutex<LRU> on a search trace, but 7.3x the per-op cost. | B | 🟡 Holds up in part |
| [**stretto (Rust port of Ristretto)**](al8n-stretto.md) | TinyLFU cache ported from Go's Ristretto; wins hit ratio only on small caches over skewed keys, and 0.8 measured broken. | C | 🟡 Holds up in part |
