# gxhash (AES/SIMD hasher)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Hashers for maps and sets](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> AES- and SIMD-based hasher that is fastest on long keys but slower than foldhash or rapidhash on integers, and needs AES at build time.

| | |
|---|---|
| **Beats** | std HashMap's RandomState (SipHash-1-3) |
| **Claimed result** | Claimed: fastest non-cryptographic hash of its class for all input sizes (charts only). Independent, AMD EPYC with target-cpu=native: geometric mean 5.01 vs 25.75 ns for SipHash (about 5.1x, derived from page data) but behind rapidhash-f at 4.68; average rank 4.31 vs 2.56 for rapidhash-f; tenkilobyte keys hash-only 119.67 vs 351.00 ns for rapidhash-f and 2103.16 for SipHash; u64 lookup hit 2.89 vs 2.03 ns for FxHash. |
| **Workload** | foldhash suite as run by rapidhash: hash-only, lookups and set build over 16 key distributions, from u32 to 10 KB byte strings. |
| **Metric** | time (ns per operation) |
| **Hardware** | AMD EPYC 9R14 (independent); project charts from GitHub Actions x86_64 and ARM runners |
| **Toolchain** | lto = "fat", codegen-units = 1, -C target-cpu=native (required to compile gxhash) |
| **Measured** | 2025-08 |
| **Adoption** | 1.0k stars |
| **Last commit** | 2026-09-26 |
| **Platforms** | x86_64 with AES-NI and SSE2, aarch64 with AES and NEON; no fallback, fails to build elsewhere or without target-cpu=native / target-feature=+aes,+sse2; no_std without the std feature |
| **Install** | cargo add gxhash (build with RUSTFLAGS="-C target-cpu=native") |
| **Links** | [repository](https://github.com/ogxd/gxhash) · [documentation](https://docs.rs/gxhash) · [benchmark](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt) |

## Verdict

gxhash beats SipHash-1-3 by about 5x in independent hash-table tests and is the fastest tested hasher on long inputs. Its claim to be the fastest for all input sizes does not hold: on integers and short keys, rapidhash, foldhash and FxHash are faster. The released 3.x line also has seed-independent collisions.

**Why tier B:** The project's own numbers exist only as SVG charts; the numbers used here come from the independent rapidhash suite, which has public code and stated hardware.

## Use it when

- Keys are long strings or byte buffers (hundreds of bytes or more) and you control the target CPU.
- Deterministic hashes across platforms for a given major version are useful (the deterministic feature).

## Skip it when

- Keys are integers, tuples or short strings: rapidhash, foldhash and FxHash were faster in the independent suite.
- Keys are attacker-controlled: the 3.x releases have seed-independent multicollisions (issue #83).
- You ship portable binaries or libraries: the crate does not compile without AES and SSE2 or NEON target features.
- Safety-critical code: the README warns of aggressive unsafe tricks.

## How it is fast

gxhash processes input in 16-byte SIMD lanes with high instruction-level parallelism and mixes with hardware AES rounds. It reads past the end of short inputs with unsafe loads to avoid branching on length. On long inputs this gives much higher throughput than scalar multiply-based hashers.

## What it costs

- DoS resistance: seeded, but issue #83 shows inputs that collide for every seed in 3.x; a later report (#131) finds seed-dependent collisions above 128 bytes in 4.0.
- Requires target-cpu=native or explicit target features, which ties binaries to a CPU family.
- Uses unsafe out-of-bounds reads as an optimization.

## Problems with the evidence

- The project's own benchmark numbers exist only in SVG charts with no text values.
- The independent suite tested gxhash 3.4.1; crates.io's latest is 3.5.0, and a 4.0 rewrite that fixes the seed-independent collisions was merged but not yet released when checked.
- The independent suite is run by the author of a competing hasher (rapidhash).

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> Up to this date, GxHash is the fastest non-cryptographic hashing algorithm of its class, for all input sizes.
>
> — [github.com/ogxd/gxhash/blob/main/README.md](https://github.com/ogxd/gxhash/blob/main/README.md)

> Gxhash requires aes and sse2 intrinsics. Make sure the processor supports it and build with RUSTFLAGS="-C target-cpu=native"
>
> — [github.com/ogxd/gxhash/blob/main/README.md](https://github.com/ogxd/gxhash/blob/main/README.md)

> │ geometric_mean ┆        4.68 ┆        5.34 ┆       5.24 ┆       5.91 ┆   5.01 ┆   5.98 ┆  5.63 ┆   25.75 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │       avg_rank ┆        2.56 ┆        4.36 ┆       3.45 ┆       5.38 ┆   4.31 ┆   3.36 ┆  4.61 ┆    7.97 │
>
> — [github.com/hoxxep/rapidhash/blob/master/README.md](https://github.com/hoxxep/rapidhash/blob/master/README.md)

> │    tenkilobyte ┆   hashonly ┆      351.00 ┆      344.43 ┆     383.38 ┆     380.80 ┆  119.67 ┆ 1716.77 ┆  310.33 ┆ 2103.16 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> │            u64 ┆  lookuphit ┆        2.21 ┆        2.73 ┆       2.21 ┆       2.71 ┆    2.89 ┆    2.03 ┆    2.81 ┆   14.74 │
>
> — [github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_…](https://github.com/hoxxep/rapidhash/blob/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.txt)

> gxhash = "3.4.1"
>
> — [github.com/hoxxep/rapidhash/blob/master/rapidhash-bench/Cargo.toml](https://github.com/hoxxep/rapidhash/blob/master/rapidhash-bench/Cargo.toml)

> Hash has arbitrary seed-independent multicollisions, is not DoS resistant
>
> — [github.com/ogxd/gxhash/issues/83](https://github.com/ogxd/gxhash/issues/83)

</details>
