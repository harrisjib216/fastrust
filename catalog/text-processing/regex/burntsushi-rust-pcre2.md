# pcre2 (Rust bindings to PCRE2 with JIT)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [Regular expressions](README.md)</sub>

**Tier C** (first-party numbers) · ❌ Doesn't hold up · Open source · `Rust + C` · `Unlicense OR MIT` · 🟡 stale

> Rust bindings to PCRE2; the JIT wins some rebar workloads but is about 2x slower than the regex crate overall.

| | |
|---|---|
| **Beats** | regex 1.12.2 |
| **Claimed result** | Slower overall: search geometric mean 6.00 (pcre2/jit) vs 3.08 (rust/regex), lower is better. Per-benchmark wins: unstructured-to-json 1656.4 MB/s vs 110.4 MB/s; sherlock-casei-ru 18.0 GB/s vs 8.4 GB/s. Losses: cloud-flare-redos 44.0 MB/s vs 563.8 MB/s. Without JIT, PCRE2 builds patterns fastest (compile geometric mean 1.41 vs 11.85) but searches slowest (114.75). |
| **Workload** | rebar curated set: 34 search benchmarks shared by pcre2/jit, plus compile-time benchmarks. |
| **Metric** | throughput |
| **Measured** | 2025-12-19 |
| **Adoption** | 64 stars |
| **Last commit** | 2025-09-25 |
| **Platforms** | Wherever the PCRE2 C library builds (through pcre2-sys) |
| **Install** | cargo add pcre2 |
| **Links** | [repository](https://github.com/BurntSushi/rust-pcre2) · [documentation](https://docs.rs/pcre2) · [benchmark](https://github.com/BurntSushi/rebar/blob/master/README.md) |

## Verdict

In the December 2025 rebar run, PCRE2's JIT has a search geometric mean of 6.00 against 3.08 for the regex crate, so it is slower overall. It wins specific workloads (unstructured-to-json, Unicode character data, some case-insensitive literals) and loses badly on others, such as the Cloudflare ReDoS patterns.

**Why tier C:** Public harness with engine versions, but no hardware stated, and the regex crate's author also maintains rebar and these bindings.

## Use it when

- You need features the regex crate lacks, such as backreferences and look-around
- Workloads like rebar's unstructured-to-json, where the JIT measured 1656.4 MB/s vs 110.4 MB/s

## Skip it when

- Untrusted patterns or haystacks: PCRE2 is a backtracker with no linear-time guarantee; one cloud-flare-redos variant ran at 671.2 KB/s vs 79.6 GB/s for regex
- As a general speed upgrade over the regex crate

## How it is fast

PCRE2's JIT compiles each pattern to native machine code for its backtracking matcher, which removes interpreter overhead and helps on patterns with few literals to prefilter on. It does not have the regex crate's lazy DFA or multi-literal SIMD prefilters.

## What it costs

- C dependency and unsafe FFI
- Exponential worst cases on some patterns
- Thin wrapper; the README says it covers little of PCRE2 and maintenance bandwidth is limited

## Problems with the evidence

- Geometric means over different benchmark subsets (34 for pcre2/jit, 38 for rust/regex)
- No CPU, OS or flags stated

## Evidence

<details>
<summary>12 quotes from the sources, checked 2026-10-28</summary>

> | pcre2/jit | 10.47 2025-10-21 | 6.00 | 34 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 1.12.2 | 3.08 | 38 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | pcre2 | 10.47 2025-10-21 | 114.75 | 33 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | pcre2 | 10.47 2025-10-21 | 1.41 | 10 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 1.12.2 | 11.85 | 14 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | pcre2/jit | **1656.4 MB/s** | 8.87us |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 110.4 MB/s | 20.13us |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | pcre2/jit | 26.3 GB/s | 18.8 GB/s | 32.8 GB/s | **18.0 GB/s** | 41.7 GB/s |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 29.4 GB/s | 10.5 GB/s | 35.6 GB/s | 8.4 GB/s | 47.7 GB/s |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | pcre2/jit | 44.0 MB/s | 46.8 MB/s | 671.2 KB/s |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | **563.8 MB/s** | 1677.2 MB/s | **79.6 GB/s** |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> There are no specific plans in place to build out the wrapper further
>
> — [github.com/BurntSushi/rust-pcre2/blob/master/README.md](https://github.com/BurntSushi/rust-pcre2/blob/master/README.md)

</details>
