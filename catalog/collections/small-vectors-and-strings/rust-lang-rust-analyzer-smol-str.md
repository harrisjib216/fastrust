# smol_str (inline immutable string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Small vectors and strings](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Immutable string type that stores up to 23 bytes inline and clones in O(1); far fewer allocations than String for short tokens.

| | |
|---|---|
| **Beats** | String for short, mostly immutable strings |
| **Claimed result** | Allocation events 2017 with String vs 23 with SmolStr; peak memory 82.7 KB vs 73.9 KB for 1000 records. |
| **Workload** | Deserializing 1,000 JSON records of US city names with serde_json, counting allocations with a tracking global allocator. |
| **Metric** | memory (allocation count, peak bytes) |
| **Hardware** | Intel Xeon E5-1650 v2 (microbenchmark charts) |
| **OS** | Linux |
| **Toolchain** | debug build (cargo build, unoptimized); smol_str 0.1.15 |
| **Measured** | 2020-07 |
| **Adoption** | 497 stars |
| **Last commit** | 2026-10-21 |
| **Platforms** | All Rust targets; no_std with alloc |
| **Install** | cargo add smol_str |
| **Links** | [repository](https://github.com/rust-lang/rust-analyzer) · [documentation](https://docs.rs/smol_str) · [benchmark](https://fasterthanli.me/articles/small-strings-in-rust) |

## Verdict

An independent test confirms the allocation win: loading 1,000 US city records, SmolStr cut allocation events from 2,017 to 23 because almost every name fits inline. The test ran in a debug build with 2020 versions and published speed results only as charts, so the evidence covers allocations and memory, not time.

**Why tier B:** Independent article with numbers in text; allocation counts and peak memory only, measured in a debug build.

## Use it when

- Many short, immutable strings such as identifiers, tokens or names that are created once and cloned often.

## Skip it when

- Strings are mutated after creation: SmolStr is immutable; compact_str or String fit better.
- Strings are mostly longer than the inline limit and rarely cloned: the Arc adds a small constant cost over String.

## How it is fast

SmolStr is the size of a String but stores short strings (and runs of newlines followed by spaces) inline, so creating them allocates nothing. Longer strings live in an Arc<str>, so clone is a reference-count increment instead of a copy.

## What it costs

- Immutable API; any change builds a new string.
- Heap strings are Arc-backed, adding atomic reference counting.

## Problems with the evidence

- Measured in a debug build; allocation counts do not depend on optimization level, but no optimized timing numbers are in text.
- Article dates from 2020 and used smol_str 0.1.15; the current crate is 0.3.x and stores up to 23 bytes inline.
- The standalone repository is archived; the crate now lives in the rust-analyzer repository, which is where the last-commit date comes from. Stars are the archived repository's.
- rosetta-rs/string-rosetta-rs dropped smol_str from its comparison in favor of ecow.

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> and the number of allocations fell from 2017 to just 23!
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> peak bytes | 82.7 KB
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> peak bytes | 73.9 KB
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> Read 1000 records
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> Intel(R) Xeon(R) CPU E5-1650 v2 @ 3.50GHz
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

</details>
