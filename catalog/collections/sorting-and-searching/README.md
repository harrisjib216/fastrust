# Sorting and searching

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md)</sub>

**The default:** slice::sort, sort_unstable and binary_search.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std slice::sort (driftsort, Rust 1.81)**](std-slice-sort.md) | Since Rust 1.81, slice::sort is driftsort: about 2.4x faster on random u64 and far faster on inputs with many duplicates. | A | ✅ Holds up |
| [**std slice::sort_unstable (ipnsort, Rust 1.81)**](std-slice-sort-unstable.md) | Since Rust 1.81, slice::sort_unstable is ipnsort: a faster, smaller pdqsort successor with a sorting-network small-sort. | A | ✅ Holds up |
| [**voracious_radix_sort (MSD/LSD radix sorts)**](lakwet-voracious-sort.md) | Radix sorts for Rust primitives and keyed structs; about 3x faster than pre-1.81 sort_unstable on a million u64. | C | 🟡 Holds up in part |
| [**glidesort (stable hybrid merge/quick sort)**](orlp-glidesort.md) | Stable sort mixing powersort run merging with stable quicksort; its ideas now ship in std's driftsort, which narrows the gap. | D | 🟡 Holds up in part |
