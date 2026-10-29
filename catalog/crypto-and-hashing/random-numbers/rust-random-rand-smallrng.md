# rand SmallRng (Xoshiro256++ on 64-bit)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md) › [Random numbers](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> rand's non-cryptographic SmallRng: about 2.7x to 3.6x faster than the ChaCha12 generator behind StdRng and rand::rng().

| | |
|---|---|
| **Beats** | rand's StdRng and ThreadRng / rand::rng() (ChaCha12) |
| **Claimed result** | u64 output: 11 GB/s vs 4.1 GB/s for ChaCha12 (Ryzen 9 9950X3D, rand book); one u64: 1.13 ns vs 4.11 ns, fill 1 KiB: 7.05 GB/s vs 2.02 GB/s (Apple M1 Max) |
| **Workload** | u64 generation; second source also measures u32 draws and filling a 1 KiB buffer. |
| **Metric** | throughput |
| **Hardware** | AMD Ryzen 9 9950X3D (rand book); Apple M1 Max (rapidrand) |
| **Adoption** | 2.1k stars |
| **Last commit** | 2026-09-20 |
| **Platforms** | All targets; Xoshiro256++ on 64-bit, Xoshiro128++ on 32-bit |
| **Install** | cargo add rand (use rand::rngs::SmallRng) |
| **Links** | [repository](https://github.com/rust-random/rand) · [documentation](https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html) · [benchmark](https://rust-random.github.io/book/guide-rngs.html) |

## Verdict

Rand's guide lists Xoshiro256++, SmallRng's algorithm on 64-bit targets, at 11 GB/s against 4.1 GB/s for ChaCha12 on a Ryzen 9 9950X3D, and an independent M1 Max run measured 1.13 ns vs 4.11 ns per u64. SmallRng is not the fastest small generator: wyrand crates such as fastrand are about 2x faster again.

**Why tier B:** The rand book's figures are first-party and rough, but an independent criterion run on a stated CPU agrees.

## Use it when

- Simulations, games and randomized algorithms already written against rand's traits and distributions
- Seeding many independent fast generators from one ThreadRng

## Skip it when

- Anything security-relevant: use rand::rng() or StdRng
- Reproducible streams across rand versions: SmallRng is documented as non-portable; use rand_xoshiro directly
- Maximum speed outside the rand ecosystem: fastrand measured 0.51 ns vs 1.13 ns per u64

## How it is fast

Xoshiro256++ updates 32 bytes of state with a few shifts, rotates and XORs and adds two words for output, so each u64 costs a handful of instructions. ChaCha12 runs 12 rounds of a cipher block function and buffers the output.

## What it costs

- Not cryptographically secure
- Algorithm may change between rand versions

## Problems with the evidence

- The rand book calls its numbers rough and limited to u64 output
- The book gives no number for SmallRng itself, only for Xoshiro256++, which the docs name as its current 64-bit algorithm

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> Xoshiro256++
>
> — [rust-random.github.io/book/guide-rngs.html](https://rust-random.github.io/book/guide-rngs.html)

> 11 GB/s
>
> — [rust-random.github.io/book/guide-rngs.html](https://rust-random.github.io/book/guide-rngs.html)

> ChaCha12
>
> — [rust-random.github.io/book/guide-rngs.html](https://rust-random.github.io/book/guide-rngs.html)

> 4.1 GB/s
>
> — [rust-random.github.io/book/guide-rngs.html](https://rust-random.github.io/book/guide-rngs.html)

> Here, performance is measured roughly for u64 outputs on an AMD Ryzen 9 9950X3D
>
> — [rust-random.github.io/book/guide-rngs.html](https://rust-random.github.io/book/guide-rngs.html)

> Xoshiro256PlusPlus on 64-bit platforms and Xoshiro128PlusPlus on 32-bit
>
> — [docs.rs/rand/latest/rand/rngs/struct.SmallRng.html](https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html)

> The current algorithm used is the ChaCha block cipher with 12 rounds.
>
> — [docs.rs/rand/latest/rand/rngs/struct.StdRng.html](https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html)

> | [rand](https://crates.io/crates/rand) `SmallRng`                           |  1.13 ns | 1.20 ns |    7.05 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> | [rand](https://crates.io/crates/rand) `StdRng` (ChaCha12)                  |  4.11 ns | 2.26 ns |    2.02 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

> | [fastrand](https://crates.io/crates/fastrand) `Rng`                        |  0.51 ns | 0.52 ns |   21.49 GB/s |
>
> — [github.com/hoxxep/rapidrand/blob/main/README.md](https://github.com/hoxxep/rapidrand/blob/main/README.md)

</details>
