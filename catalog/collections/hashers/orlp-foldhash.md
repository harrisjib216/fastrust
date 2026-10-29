# foldhash (folded-multiply hasher, hashbrown's default)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Hashers for maps and sets](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `Zlib` · 🟢 active

> Portable, seeded hasher built on a folded multiply; about 5x faster than SipHash-1-3 across hash-table workloads.

| | |
|---|---|
| **Beats** | std HashMap's RandomState (SipHash-1-3) |
| **Claimed result** | Geometric mean 6.21 vs 28.70 ns per operation against SipHash on Intel Xeon (about 4.6x, derived from page data) and 4.41 vs 21.94 on Apple M2 (about 5.0x, derived from page data); u64 lookup hit 3.04 vs 12.65 ns; independent rerun on AMD EPYC with target-cpu=native: 5.24 vs 25.75 ns geometric mean. |
| **Workload** | Hash-only, lookup miss and hit in a 1,000-element HashMap, and HashSet build from 10,000 inserts with about 90% duplicates, over 16 key types from u32 to 10 KB byte strings. |
| **Metric** | time (ns per operation) |
| **Hardware** | Intel Xeon Platinum 8481C; Apple M2; AMD EPYC 9R14 (independent rerun) |
| **Toolchain** | Rust 1.80.1, lto = "thin", with and without -C target-cpu=native (foldhash suite); lto = "fat", codegen-units = 1 (rapidhash rerun) |
| **Measured** | 2024-08 |
| **Adoption** | 387 stars |
| **Last commit** | 2026-06-29 |
| **Platforms** | All Rust targets (portable, no special CPU instructions); no_std without the std feature |
| **Install** | cargo add foldhash (or use hashbrown 0.15+, where it is the default hasher) |
| **Links** | [repository](https://github.com/orlp/foldhash) · [documentation](https://docs.rs/foldhash) · [benchmark](https://github.com/orlp/foldhash/blob/master/README.md) |

## Verdict

Across 16 key distributions and four operations, foldhash-f is about 4.6x to 5x faster than SipHash-1-3 by geometric mean in its own suite, and a competing author's suite reproduces a similar gap on AMD EPYC. The win holds without target-cpu=native.

**Why tier A:** Public benchmark code (benches/bench.rs) with hardware, Rust 1.80.1 and flags stated, and the same suite rerun independently by the rapidhash project.

## Use it when

- General-purpose HashMap and HashSet keys on any platform, without relying on AES or target-cpu flags.
- Integer, tuple and small struct keys, where it is within a few tenths of a nanosecond of FxHash without FxHash's collision cliffs.

## Skip it when

- Keys come from untrusted, interactive clients who can observe timing or iteration order: the README says it does not resist such attackers.
- You need hashes that are stable across versions or platforms.

## How it is fast

Each mixing step multiplies two 64-bit words into a 128-bit product and XORs the halves together, which mixes bits well in one multiply. Integers take a single folded multiply against random per-process secrets. The fast variant skips a final avalanche step because the benchmarks showed better bit quality did not make hash tables faster.

## What it costs

- DoS resistance: 'minimally DoS-resistant'. Random secrets stop fixed collision sets, but the author states it does not resist interactive attackers who can learn the secrets.
- Measurable statistical bias in foldhash::fast; use foldhash::quality for HyperLogLog or MinHash style algorithms.
- Hash output is not stable across versions.

## Problems with the evidence

- First-party numbers were taken with foldhash 0.1.0 in August 2024; 0.2.0 changed string hashing.
- In the rapidhash rerun, foldhash's string hashing is slower than rapidhash's (English words: 4.26 vs 1.60 ns hash-only), so foldhash is not the fastest choice for string keys.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> │        geometric_mean       ┆       6.21 ┆       7.01 ┆    7.56 ┆    8.74 ┆   28.70 │
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> │        geometric_mean       ┆       4.41 ┆       4.86 ┆    5.39 ┆    5.71 ┆   21.94 │
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> │            u64 ┆  lookuphit ┆       3.04 ┆       3.59 ┆    2.64 ┆    3.84 ┆   12.65 │
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> one with a 2023 Apple M2 CPU, one with a 2023 Intel Xeon Platinum 8481C server CPU, both with stable Rust 1.80.1.
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> resistance against interactive attackers
>
> — [github.com/orlp/foldhash/blob/master/README.md](https://github.com/orlp/foldhash/blob/master/README.md)

> │ geometric_mean ┆        4.68 ┆        5.34 ┆       5.24 ┆       5.91 ┆   5.01 ┆   5.98 ┆  5.63 ┆   25.75 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │ strenglishword ┆   hashonly ┆        1.60 ┆        2.52 ┆       4.26 ┆       4.32 ┆    1.43 ┆    2.64 ┆    2.61 ┆   13.15 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

</details>
