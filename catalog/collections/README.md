# Collections

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Hashers for maps and sets](hashers/README.md)

**The default:** std HashMap's RandomState (SipHash-1-3).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**foldhash (folded-multiply hasher, hashbrown's default)**](hashers/orlp-foldhash.md) | Portable, seeded hasher built on a folded multiply; about 5x faster than SipHash-1-3 across hash-table workloads. | A | ✅ Holds up |
| [**rapidhash (wyhash successor)**](hashers/hoxxep-rapidhash.md) | Portable rapidhash/wyhash-family hasher with randomized secrets; about 5x faster than SipHash-1-3 and slightly ahead of foldhash. | A | ✅ Holds up |
| [**aHash (AES-based keyed hasher)**](hashers/tkaitchuck-ahash.md) | Keyed, randomly seeded hasher for HashMap that uses AES instructions where available; several times faster than SipHash-1-3. | A | 🟡 Holds up in part |
| [**gxhash (AES/SIMD hasher)**](hashers/ogxd-gxhash.md) | AES- and SIMD-based hasher that is fastest on long keys but slower than foldhash or rapidhash on integers, and needs AES at build time. | B | 🟡 Holds up in part |
| [**rustc-hash (FxHasher, rewritten in 2.0)**](hashers/rust-lang-rustc-hash.md) | The rustc compiler's unkeyed FxHasher; fastest on integer keys, with no DoS resistance and known collision patterns. | B | 🟡 Holds up in part |

## [Maps and sets](maps-and-sets/README.md)

**The default:** std HashMap, HashSet and BTreeMap.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std HashMap and HashSet (hashbrown SwissTable, Rust 1.36)**](maps-and-sets/std-collections-hashmap.md) | Since Rust 1.36, std's HashMap is the hashbrown SwissTable, about 2x faster on lookups than the old Robin Hood table with the same hasher. | C | ✅ Holds up |
| [**hashbrown crate (SwissTable with foldhash as default hasher)**](maps-and-sets/rust-lang-hashbrown.md) | The SwissTable crate behind std's HashMap; on today's toolchains its speed edge over std is only its foldhash default hasher. | A | 🟡 Holds up in part |
| [**indexset (two-level order-statistic B-tree)**](maps-and-sets/brurucy-indexset.md) | Ordered set and map built from fixed-capacity Vec leaves; faster scans and rank lookups than std's BTreeMap, slower point lookups and removals. | A | 🟡 Holds up in part |

## [Small vectors and strings](small-vectors-and-strings/README.md)

**The default:** Vec and String.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std Vec growth skips capacities 1 and 2 (Rust 1.45)**](small-vectors-and-strings/std-vec-min-non-zero-cap.md) | Since Rust 1.45, a Vec grown by push starts at capacity 4 (8 for bytes) instead of 1, removing two reallocations for small Vecs. | B | ✅ Holds up |
| [**smol_str (inline immutable string)**](small-vectors-and-strings/rust-lang-rust-analyzer-smol-str.md) | Immutable string type that stores up to 23 bytes inline and clones in O(1); far fewer allocations than String for short tokens. | B | 🟡 Holds up in part |
| [**compact_str (CompactString, 24-byte inline string)**](small-vectors-and-strings/parkmycar-compact-str.md) | Mutable String replacement that stores strings up to 24 bytes inline in the same 24 bytes; avoids allocations for short strings. | D | 🟡 Holds up in part |
| [**smartstring (SmartString inline string)**](small-vectors-and-strings/bodil-smartstring.md) | Archived inline-string String replacement; it saves allocations, but an independent benchmark drops it from access tests for being slow. | D | ❌ Doesn't hold up |

## [Sorting and searching](sorting-and-searching/README.md)

**The default:** slice::sort, sort_unstable and binary_search.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std slice::sort (driftsort, Rust 1.81)**](sorting-and-searching/std-slice-sort.md) | Since Rust 1.81, slice::sort is driftsort: about 2.4x faster on random u64 and far faster on inputs with many duplicates. | A | ✅ Holds up |
| [**std slice::sort_unstable (ipnsort, Rust 1.81)**](sorting-and-searching/std-slice-sort-unstable.md) | Since Rust 1.81, slice::sort_unstable is ipnsort: a faster, smaller pdqsort successor with a sorting-network small-sort. | A | ✅ Holds up |
| [**voracious_radix_sort (MSD/LSD radix sorts)**](sorting-and-searching/lakwet-voracious-sort.md) | Radix sorts for Rust primitives and keyed structs; about 3x faster than pre-1.81 sort_unstable on a million u64. | C | 🟡 Holds up in part |
| [**glidesort (stable hybrid merge/quick sort)**](sorting-and-searching/orlp-glidesort.md) | Stable sort mixing powersort run merging with stable quicksort; its ideas now ship in std's driftsort, which narrows the gap. | D | 🟡 Holds up in part |

## [Concurrent maps](concurrent-maps/README.md)

**The default:** Mutex<HashMap> or RwLock<HashMap>.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**DashMap (sharded RwLock hash map)**](concurrent-maps/xacrimon-dashmap.md) | Sharded concurrent HashMap with a HashMap-like &self API; scales far past RwLock<HashMap> under contention but is slower single-threaded. | A | ✅ Holds up |
| [**leapfrog (LeapMap concurrent hash map)**](concurrent-maps/robclu-leapfrog.md) | Lock-free concurrent map using leapfrog probing; about 12x RwLock<HashMap> throughput at 16 threads on reads, with value-type restrictions. | A | ✅ Holds up |
| [**evmap (left-right eventually consistent map)**](concurrent-maps/jonhoo-evmap.md) | Eventually consistent multi-value map built on left-right; lock-free reads that scale, at the cost of slow, batched writes. | A | 🟡 Holds up in part |
| [**flurry (port of Java's ConcurrentHashMap)**](concurrent-maps/jonhoo-flurry.md) | Rust port of Java's ConcurrentHashMap; scales on reads but has memory and write-throughput problems its author acknowledges. | A | 🟡 Holds up in part |
| [**papaya (lock-free read-optimized hash map)**](concurrent-maps/ibraheemdev-papaya.md) | Lock-free concurrent hash table where reads never block; aimed at read-heavy workloads and predictable latency. | D | 🟡 Holds up in part |
