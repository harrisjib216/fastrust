# regex-lite (small, fast-building regex)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [Regular expressions](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Drop-in subset of the regex crate that builds in about 40% of the time and adds about a sixth of the binary size, at the cost of much slower searches.

| | |
|---|---|
| **Beats** | regex 1.9.0 with every feature disabled except std (build time and binary size); regex 1.12.2 for regex compile and search time |
| **Claimed result** | Build: 0.73s vs 1.93s compile time and 94KB vs 565KB relative size increase (regex-lite 0.1.0 vs regex 1.9.0 with only std); Regex::new: compile-time geometric mean 2.73 vs 11.85 in rebar (lower is better). Search is slower: geometric mean 156.49 vs 3.08; literal sherlock-en 74.4 MB/s vs 29.4 GB/s. |
| **Workload** | Compile time and binary-size increase of a program depending on the crate; rebar compile-time and search-time benchmarks (10 and 28 benchmarks for regex-lite). |
| **Metric** | build time, binary size |
| **Adoption** | 4.0k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | All Rust targets |
| **Install** | cargo add regex-lite |
| **Links** | [repository](https://github.com/rust-lang/regex) · [documentation](https://docs.rs/regex-lite) · [benchmark](https://github.com/rust-lang/regex/blob/master/regex-lite/README.md) |

## Verdict

The author's experiment shows regex-lite building in 0.73s against 1.93s and adding 94KB against 565KB, and rebar ranks it far ahead on Regex::new time (2.73 vs 11.85). Those are the claims it makes; it does not claim faster searches, and rebar shows it is much slower there (search geometric mean 156.49 vs 3.08).

**Why tier C:** First-party numbers: one build experiment with no machine or rustc stated, plus the author's own rebar runs without hardware details.

## Use it when

- CLI tools, build scripts or WASM bundles where binary size and compile time matter more than regex throughput
- Programs that build many regexes and run each over short inputs

## Skip it when

- Searching large haystacks: rebar's literal benchmark runs at 74.4 MB/s vs 29.4 GB/s for regex
- Patterns that need Unicode classes or Unicode case folding, which regex-lite leaves out

## How it is fast

It skips the meta engine, DFAs, literal prefilters and most Unicode tables and runs every search on a single PikeVM, so there is much less code to compile and link. Less analysis at Regex::new also makes pattern construction faster. It keeps the linear-time search guarantee.

## What it costs

- Search throughput is one to three orders of magnitude lower than regex on literal-heavy workloads
- No robust Unicode support

## Problems with the evidence

- The build-time numbers come from one experiment on an unstated machine
- The rebar summary ranks each engine over the benchmarks it can run, and regex-lite skips the Unicode-heavy ones

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> For `regex`, I disabled all features except for `std`:
>
> — [github.com/rust-lang/regex/blob/master/regex-lite/README.md](https://github.com/rust-lang/regex/blob/master/regex-lite/README.md)

> 1.93s compile time, 565KB relative size increase
>
> — [github.com/rust-lang/regex/blob/master/regex-lite/README.md](https://github.com/rust-lang/regex/blob/master/regex-lite/README.md)

> 0.73s compile time, 94KB relative size increase
>
> — [github.com/rust-lang/regex/blob/master/regex-lite/README.md](https://github.com/rust-lang/regex/blob/master/regex-lite/README.md)

> | rust/regex/lite | 0.1.8 | 2.73 | 10 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 1.12.2 | 11.85 | 14 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex/lite | 0.1.8 | 156.49 | 28 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 1.12.2 | 3.08 | 38 |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex/lite | 74.4 MB/s | 52.5 MB/s | 117.3 MB/s | - | 162.3 MB/s |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

> | rust/regex | 29.4 GB/s | 10.5 GB/s | 35.6 GB/s | 8.4 GB/s | 47.7 GB/s |
>
> — [github.com/BurntSushi/rebar/blob/master/README.md](https://github.com/BurntSushi/rebar/blob/master/README.md)

</details>
