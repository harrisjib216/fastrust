# std str::chars().count() word-at-a-time counting (Rust 1.60)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [UTF-8 and Unicode](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Rust 1.60 made str::chars().count() about 4x to 5x faster on medium and large strings by counting a machine word at a time.

| | |
|---|---|
| **Beats** | str::chars().count() before Rust 1.60 |
| **Claimed result** | 4x to 5x faster per the author; en_large: 1,598 ns to 281 ns (3369 MB/s to 19160 MB/s); zh_huge: 82,622 ns to 15,053 ns; en_small unchanged (14 ns vs 15 ns) |
| **Workload** | libcore char_count benchmarks: English, Russian, Chinese and emoji text at small, medium, large and huge sizes. |
| **Metric** | throughput |
| **Measured** | 2021-10 |
| **Platforms** | All targets (portable code, no SIMD intrinsics) |
| **Install** | built into std since 1.60 |
| **Links** | [documentation](https://doc.rust-lang.org/std/str/struct.Chars.html) · [benchmark](https://github.com/rust-lang/rust/pull/90414) |

## Verdict

The PR measured roughly 4x to 5x faster character counting on medium to huge text in English, Russian, Chinese and emoji, with small inputs unchanged, and it shipped in Rust 1.60.

**Why tier B:** A rust-lang/rust change measured with the in-tree libcore benchmarks; the CPU is not stated.

## Use it when

- Counting chars in a &str on Rust 1.60 or later: call chars().count() directly

## Skip it when

- You need grapheme clusters or display width, which char count does not give

## How it is fast

Counting chars in valid UTF-8 equals counting bytes that are not continuation bytes (10xxxxxx). The new code reads the string a usize at a time and counts non-continuation bytes with bit operations, summing several words before folding the per-byte counts, instead of decoding one char at a time.

## Problems with the evidence

- CPU not stated; libtest bench harness
- Small strings see no change (en_small 14 ns before, 15 ns after)

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> is somewhat consistently around 4x to 5x faster than the old implementation
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::en_large::case00_cur_libcore         ... bench:         281 ns/iter (+/- 6) = 19160 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::en_large::case01_old_libcore         ... bench:       1,598 ns/iter (+/- 19) = 3369 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::zh_huge::case00_cur_libcore          ... bench:      15,053 ns/iter (+/- 2,640) = 20067 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::zh_huge::case01_old_libcore          ... bench:      82,622 ns/iter (+/- 3,602) = 3656 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::en_small::case00_cur_libcore         ... bench:          15 ns/iter (+/- 0) = 2333 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

> test str::char_count::en_small::case01_old_libcore         ... bench:          14 ns/iter (+/- 0) = 2500 MB/s
>
> — [github.com/rust-lang/rust/pull/90414](https://github.com/rust-lang/rust/pull/90414)

</details>
