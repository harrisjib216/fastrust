# memchr (memchr::memmem substring search)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [String and byte search](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `Unlicense OR MIT` · 🟢 active

> SIMD substring search for &[u8] that runs about 5x to 6x faster than std's str::find across 53 benchmarks.

| | |
|---|---|
| **Beats** | std substring search (str::find / str::contains: Two-Way plus the SSE2 short-needle path) on rustc 1.73.0-nightly |
| **Claimed result** | About 6.3x faster with a prebuilt searcher (geometric-mean speed ratio 1.03 vs 6.50 over 53 benchmarks) and about 5.1x with construction included (1.04 vs 5.26) (ratios derived from page data) |
| **Workload** | 53 substring-search benchmarks from memchr's rebar suite: prebuilt (searcher built once, then searched) and oneshot (built per search). |
| **Metric** | throughput |
| **Hardware** | x86_64 (CPU model not stated) |
| **Toolchain** | std side: rustc 1.73.0-nightly (180dffba1) |
| **Measured** | 2023-08-26 |
| **Adoption** | 1.6k stars |
| **Last commit** | 2026-08-10 |
| **Platforms** | x86_64 (SSE2; AVX2 detected at runtime with std), aarch64 and wasm32 SIMD; SWAR fallback elsewhere; no_std |
| **Install** | cargo add memchr |
| **Links** | [repository](https://github.com/BurntSushi/memchr) · [documentation](https://docs.rs/memchr) · [benchmark](https://github.com/BurntSushi/memchr/blob/master/README.md) |

## Verdict

Over 53 shared benchmarks memmem's prebuilt searcher has a geometric-mean speed ratio of 1.03 against 6.50 for std, and 1.04 against 5.26 when searcher construction is timed too. The std side already includes the Rust 1.67 SSE2 short-needle path, so the gap is not an artifact of an old std.

**Why tier C:** The rebar definitions and recorded CSV are public, but the CPU is identified only as x86_64 and the crate author ran the comparison.

## Use it when

- Searching many haystacks (lines, records) for the same needle: build a memmem::Finder once
- Searching bytes that are not known to be UTF-8, which saves a from_utf8 pass before using std

## Skip it when

- A single search over a short &str, where the difference is nanoseconds and std needs no dependency

## How it is fast

It picks per search between Rabin-Karp for tiny haystacks, a 'generic SIMD' prefilter that tests two bytes chosen as rare from a byte-frequency table, and Two-Way for long needles with the prefilter switched off when it stops paying. It can use AVX2 through runtime detection, which core's str search cannot, and memmem::Finder lets you build a searcher once and reuse it across haystacks.

## What it costs

- Pervasive unsafe code, covered by exhaustive, property-based and fuzz testing per the README
- One more dependency, though most projects already have it through regex

## Problems with the evidence

- Geometric means over 53 benchmarks; individual benchmark ratios vary
- The std 'prebuilt' case cannot reuse a searcher, so it approximates by finding all matches in one haystack
- CPU model and compiler flags not stated

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> rust/memchr/memmem/prebuilt  2.5.0                     1.03                            53
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

> rust/std/memmem/prebuilt     1.73.0-nightly 180dffba1  6.50                            53
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

> rust/memchr/memmem/oneshot  2.5.0                     1.04                            53
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

> rust/std/memmem/oneshot     1.73.0-nightly 180dffba1  5.26                            53
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

> The bottom line here is that `core`'s substring search implementation is limited to making use of SSE2, but not AVX.
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

</details>
