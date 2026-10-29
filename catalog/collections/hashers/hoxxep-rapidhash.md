# rapidhash (wyhash successor)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Hashers for maps and sets](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Portable rapidhash/wyhash-family hasher with randomized secrets; about 5x faster than SipHash-1-3 and slightly ahead of foldhash.

| | |
|---|---|
| **Beats** | std HashMap's RandomState (SipHash-1-3) |
| **Claimed result** | Geometric mean 4.67 vs 23.66 ns per operation for SipHash on AMD EPYC without native flags (about 5.1x, derived from page data) and 4.29 vs 22.17 on Apple M1 Max; against foldhash-f 4.67 vs 5.27 (about 1.13x, derived from page data); English-word keys hash-only 1.60 vs 4.26 ns for foldhash-f and 13.15 for SipHash. |
| **Workload** | foldhash's suite (hash-only, lookup miss and hit in a 1,000-element map, set build) over 16 key distributions, plus raw throughput charts. |
| **Metric** | time (ns per operation) |
| **Hardware** | AMD EPYC 9R14, Intel Xeon Platinum 8488C, AWS Graviton3, Apple M1 Max |
| **Toolchain** | opt-level 3, lto = "fat", codegen-units = 1, run with and without -C target-cpu=native |
| **Measured** | 2025-08 |
| **Adoption** | 289 stars |
| **Last commit** | 2026-07-03 |
| **Platforms** | All Rust targets, portable (no AES or SIMD requirement), optimized for x86_64 and aarch64; no_std; const-evaluable |
| **Install** | cargo add rapidhash |
| **Links** | [repository](https://github.com/hoxxep/rapidhash) · [documentation](https://docs.rs/rapidhash) · [benchmark](https://github.com/hoxxep/rapidhash/blob/master/README.md) |

## Verdict

In its own suite, rapidhash's fast variant is about 5x faster than SipHash-1-3 by geometric mean on four CPUs, without needing target-cpu=native. Its lead over foldhash is real but modest overall (about 9% to 13% by geometric mean across the four CPUs) and large only on strings.

**Why tier A:** Public benchmark harness and run script (docs/run_benchmarks.sh), four named CPUs, release profile flags stated; numbers are first-party.

## Use it when

- General-purpose HashMap keys in libraries or binaries that must stay portable, especially string keys.
- You also want a stable, versioned hash for persistence (the rapidhash_v3 functions), separate from the in-memory hasher.

## Skip it when

- Keys are attacker-controlled and the attacker can interact with the process: protection is described as minimal.
- Keys are plain integers and you control them: FxHash still ranks ahead on Graviton3 (average rank 2.11 vs 2.27).

## How it is fast

It uses the same folded 64x64->128-bit multiply as foldhash and wyhash, with foldhash's integer path, and processes strings in wide multiply-mix chunks from the rapidhash algorithm. No AES or SIMD is needed, so the same speed is available in portable builds.

## What it costs

- DoS resistance: 'minimally DoS resistant'. The Rust crate randomizes secrets as well as the seed; the C++ rapidhash and wyhash with static secrets have seed-independent collisions (one led to a Node.js CVE).
- In-memory hash output may change between minor versions.

## Problems with the evidence

- All numbers are first-party; the suite is foldhash's, but competitors' versions were pinned in rapidhash's bench crate (for example ahash 0.8.11, gxhash 3.4.1, fxhash 0.2.1).
- The 'fxhash' column is the old fxhash 0.2.1 crate, not rustc-hash 2.x.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> │ geometric_mean ┆        4.67 ┆        5.38 ┆       5.27 ┆       5.99 ┆   6.13 ┆  6.50 ┆   23.66 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │ geometric_mean ┆        4.29 ┆        4.82 ┆       4.83 ┆       5.24 ┆   5.50 ┆  5.94 ┆   22.17 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │       avg_rank ┆        2.27 ┆        3.88 ┆       3.08 ┆       4.66 ┆   2.11 ┆  5.05 ┆    6.97 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> significant throughput improvement over wyhash and foldhash.
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │ strenglishword ┆   hashonly ┆        1.60 ┆        2.52 ┆       4.26 ┆       4.32 ┆    1.43 ┆    2.64 ┆    2.61 ┆   13.15 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> Wyhash and rapidhash, two of the fastest non-cryptographic hash functions available today, have their own seed-independent collision vulnerability in the C++ implementations.
>
> — [liams.website/articles/seed-independent-collisions-on-wyhash-and-ra…](https://liams.website/articles/seed-independent-collisions-on-wyhash-and-rapidhash)

</details>
