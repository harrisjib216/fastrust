# Concurrent maps

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md)</sub>

**The default:** Mutex<HashMap> or RwLock<HashMap>.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**DashMap (sharded RwLock hash map)**](xacrimon-dashmap.md) | Sharded concurrent HashMap with a HashMap-like &self API; scales far past RwLock<HashMap> under contention but is slower single-threaded. | A | ✅ Holds up |
| [**leapfrog (LeapMap concurrent hash map)**](robclu-leapfrog.md) | Lock-free concurrent map using leapfrog probing; about 12x RwLock<HashMap> throughput at 16 threads on reads, with value-type restrictions. | A | ✅ Holds up |
| [**evmap (left-right eventually consistent map)**](jonhoo-evmap.md) | Eventually consistent multi-value map built on left-right; lock-free reads that scale, at the cost of slow, batched writes. | A | 🟡 Holds up in part |
| [**flurry (port of Java's ConcurrentHashMap)**](jonhoo-flurry.md) | Rust port of Java's ConcurrentHashMap; scales on reads but has memory and write-throughput problems its author acknowledges. | A | 🟡 Holds up in part |
| [**papaya (lock-free read-optimized hash map)**](ibraheemdev-papaya.md) | Lock-free concurrent hash table where reads never block; aimed at read-heavy workloads and predictable latency. | D | 🟡 Holds up in part |
