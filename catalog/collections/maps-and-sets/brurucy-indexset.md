# indexset (two-level order-statistic B-tree)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Maps and sets](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟢 active

> Ordered set and map built from fixed-capacity Vec leaves; faster scans and rank lookups than std's BTreeMap, slower point lookups and removals.

| | |
|---|---|
| **Beats** | std::collections::BTreeSet and BTreeMap |
| **Claimed result** | Full traversal of 100,000 u64: 92.5 µs for BTreeSet vs 37.3 µs (about 2.5x, derived from page data); 128-entry range traversal 164.8 vs 117.3 ns. Std is faster on contains hit (21.8 vs 23.6 ns), map get hit (24.7 vs 38.7 ns) and remove hit (69.6 vs 133.1 ns). |
| **Workload** | Sets of 100,000 u64 or 64-byte records and maps of 100,000 16- or 64-byte entries: insert, contains/get, remove, positional get_index, full traversal and 128-entry range traversal. |
| **Metric** | time (Criterion median) |
| **Hardware** | Apple M2 Pro (12 cores, 16 GB RAM) |
| **OS** | macOS 26.5.2 |
| **Toolchain** | rustc 1.97.0-nightly (ad3a598ca 2026-05-03) |
| **Measured** | 2026-08-26 |
| **Adoption** | 120 stars |
| **Last commit** | 2026-08-27 |
| **Platforms** | All Rust targets; concurrent variant behind the concurrent feature |
| **Install** | cargo add indexset |
| **Links** | [repository](https://github.com/brurucy/indexset) · [documentation](https://docs.rs/indexset) · [benchmark](https://github.com/brurucy/indexset/blob/master/README.md) |

## Verdict

In the author's own table, indexset traverses 1.5x to 2.5x faster than std's B-tree and adds positional lookups std lacks, but std is faster on point lookups and about 2x faster on removals, and inserts are mixed. It wins for scan-heavy or rank/select workloads, not as a general replacement.

**Why tier A:** Public cargo bench targets, Criterion medians from a pinned commit, with CPU, OS and rustc version stated; first-party but reports std's wins in bold.

## Use it when

- Ordered collections that are mostly iterated or range-scanned.
- You need lookup by sorted position (get_index) or rank, which std's B-tree does not offer.

## Skip it when

- Point lookups and removals dominate: std's BTreeMap was faster on both.
- Values are large and you keep the default 1024 capacity: inserting 64-byte records took 564.3 ns vs 230.0 ns for std.

## How it is fast

The tree has two levels: leaves are Vecs of fixed capacity (1024 by default) and a Fenwick tree indexes them. Iteration walks contiguous Vecs with no pointer chasing, and a lookup is two binary searches. The Fenwick tree gives near-constant-time positional lookup and rank.

## What it costs

- Insertion is O(B) best case and O(B^2) worst case in leaf capacity B.
- Removals cost about twice std's.

## Problems with the evidence

- First-party numbers from one machine on a nightly compiler.
- Results swing with the leaf capacity setting (256 vs 1024).

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> | Full traversal | `u64` (8 B) | 92.5 µs | 37.5 µs | **37.3 µs** |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> | 128-entry range traversal | `u64` (8 B) | 164.8 ns | 127.4 ns | **117.3 ns** |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> | `contains` hit | `u64` (8 B) | **21.8 ns** | 23.9 ns | 23.6 ns |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> | `get` hit | `entry_16b` (16 B) | **24.7 ns** | 46.5 ns | 38.7 ns |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> | Remove hit | `u64` (8 B) | **69.6 ns** | 133.1 ns | 163.1 ns |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> | | `record_64b` (64 B) | **230.0 ns** | 264.3 ns | 564.3 ns |
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> start with 100,000 existing values
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

> 2026-08-26 on a 12-core Apple M2 Pro with 16 GB RAM, macOS 26.5.2, and
>
> — [github.com/brurucy/indexset/blob/master/README.md](https://github.com/brurucy/indexset/blob/master/README.md)

</details>
