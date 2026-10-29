# fastrand (wyrand PRNG)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Random numbers](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟢 active

> Small wyrand-based RNG: 0.51 ns per u64 against 4.11 ns for rand's StdRng and 1.13 ns for SmallRng on an M1 Max.

| | |
|---|---|
| **Beats** | rand's StdRng (ChaCha12, the algorithm behind ThreadRng and rand::rng()) and rand's SmallRng |
| **Claimed result** | One u64: 0.51 ns vs 4.11 ns for StdRng (ChaCha12) and 1.13 ns for SmallRng; fill 1 KiB: 21.49 GB/s vs 2.02 GB/s and 7.05 GB/s (Apple M1 Max). Ryzen 1700: around 820-830ps per u64 vs 1.16ns for SmallRng. |
| **Workload** | Single-threaded criterion: one u64 draw, one u32 draw, filling a 1 KiB buffer; second source: unbounded u64 generation. |
| **Metric** | latency |
| **Hardware** | Apple M1 Max; AMD Ryzen 1700 at 3.7GHz |
| **OS** | Windows 10 x64 for the Ryzen run; not stated for the M1 Max run |
| **Measured** | 2026-07 |
| **Adoption** | 603 stars |
| **Last commit** | 2026-07-19 |
| **Platforms** | All targets; no_std without the global thread-local functions |
| **Install** | cargo add fastrand |
| **Links** | [repository](https://github.com/smol-rs/fastrand) · [documentation](https://docs.rs/fastrand) · [benchmark](https://github.com/hoxxep/rapidrand/blob/main/README.md) |

## Verdict

A third-party criterion run on an M1 Max measured fastrand's Rng at 0.51 ns per u64 against 4.11 ns for StdRng and 1.13 ns for SmallRng, and 21.49 GB/s filling 1 KiB against 2.02 GB/s and 7.05 GB/s. A second third-party run on a Ryzen 1700 also put it ahead of SmallRng.

**Why tier B:** Two benchmarks by authors of other RNG crates, independent of fastrand and rand, with CPUs stated; rustc versions not stated.

## Use it when

- Simulations, games, tests, shuffling and sampling where predictability does not matter
- Crates that want a tiny, dependency-free RNG

## Skip it when

- Keys, tokens, nonces or anything security-relevant: wyrand is not cryptographic
- Code built on rand's Rng traits and distributions, which fastrand does not implement

## How it is fast

Wyrand keeps 8 bytes of state. Each draw adds a constant to the state and scrambles it with one 64x64-to-128-bit multiply and an XOR of the two halves, with no buffering, reseeding or cipher rounds, against the ChaCha12 block function behind StdRng and ThreadRng.

## What it costs

- Not cryptographically secure; state is recoverable from a few outputs
- Own API, not rand-compatible

## Problems with the evidence

- The M1 Max run measured fastrand::Rng instances; the global fastrand:: functions add a thread-local access
- Both sources are written by authors of competing wyrand crates (rapidrand, turborand)
- ThreadRng itself was not measured; StdRng runs the same ChaCha12 algorithm

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> Single-threaded criterion benchmarks from `rapidrand-bench` on an M1 Max.
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> | [fastrand](https://crates.io/crates/fastrand) `Rng`                        |  0.51 ns | 0.52 ns |   21.49 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> | [rand](https://crates.io/crates/rand) `SmallRng`                           |  1.13 ns | 1.20 ns |    7.05 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> | [rand](https://crates.io/crates/rand) `StdRng` (ChaCha12)                  |  4.11 ns | 2.26 ns |    2.02 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> tested on Windows 10 x64 on an AMD Ryzen 1700 clocked at 3.7Ghz with 32GB RAM at 3066Mhz.
>
> — [github.com/Bluefinger/turborand/blob/main/README.md](https://github.com/Bluefinger/turborand/blob/main/README.md)

> consistently performing around 820-830ps for generating a `u64` value. `SmallRng` performs around 1.16ns
>
> — [github.com/Bluefinger/turborand/blob/main/README.md](https://github.com/Bluefinger/turborand/blob/main/README.md)

> The currently selected algorithm is ChaCha (12-rounds).
>
> — [docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)

</details>
