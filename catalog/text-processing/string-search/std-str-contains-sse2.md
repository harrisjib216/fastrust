# std str::contains SSE2 fast path for short needles (Rust 1.67)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [String and byte search](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Rust 1.67 made str::contains with short needles 6x to 9x faster in long-haystack benchmarks on x86_64 by replacing Two-Way with an SSE2 scan.

| | |
|---|---|
| **Beats** | str::contains(&str) before Rust 1.67, which used Two-Way for every needle length |
| **Claimed result** | 16-byte needle in a long haystack: 504 ns to 82 ns (5061 MB/s to 31109 MB/s); short needle in a long haystack: 677 ns to 73 ns; short in short: 27 ns to 12 ns; regression: bad_simd 84 ns to 97 ns |
| **Workload** | libcore str benchmarks: str::contains with 2- to 32-byte needles over short and long haystacks, including adversarial inputs. |
| **Metric** | time |
| **Hardware** | AMD Zen2 (model not stated) |
| **Toolchain** | -Ccodegen-units=1, nightly before 1.67 |
| **Measured** | 2022-10 |
| **Platforms** | x86_64 only (SSE2); other targets keep Two-Way |
| **Install** | built into std since 1.67 |
| **Links** | [documentation](https://doc.rust-lang.org/std/primitive.str.html#method.contains) · [benchmark](https://github.com/rust-lang/rust/pull/103779) |

## Verdict

The PR's benchmarks on Zen2 show contains with short needles in long haystacks going from 504 ns to 82 ns and from 677 ns to 73 ns, and it merged for Rust 1.67. One case got slower (bench_contains_bad_simd, 84 ns to 97 ns).

**Why tier B:** A rust-lang/rust change measured with the in-tree libcore benchmarks on a stated CPU family and codegen setting.

## Use it when

- Already on by default for str::contains with short needles on x86_64; no code change needed

## Skip it when

- Repeated searches for one needle over many haystacks or non-UTF-8 bytes, where memchr::memmem is still several times faster
- Long needles, which still use Two-Way

## How it is fast

Two-Way must compute a critical factorization of the needle before searching, which dominates for short needles, and advances at most a needle length per step. The new path, based on Wojciech Muła's SIMD substring algorithm, compares the needle's first and last bytes against 16 haystack positions at a time with SSE2 and verifies only the candidates, so it skips preprocessing and advances by vector width.

## What it costs

- One adversarial benchmark regressed (84 ns to 97 ns)

## Problems with the evidence

- The fast path is for contains; the PR title scopes it to str.contains(&str)
- Single machine and libtest's bench harness
- core cannot use runtime CPU feature detection, so the path is limited to SSE2

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> x86_64 SSE2 fast-path for str.contains(&str) and short needles
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> benchmarks taken on a Zen2, compiled with
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_16b_in_long                     ... bench:         504 ns/iter (+/- 14) = 5061 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_16b_in_long                     ... bench:          82 ns/iter (+/- 0) = 31109 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_short_long                      ... bench:         677 ns/iter (+/- 25) = 3768 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_short_long                      ... bench:          73 ns/iter (+/- 0) = 34945 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_short_short                     ... bench:          27 ns/iter (+/- 2) = 2074 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_short_short                     ... bench:          12 ns/iter (+/- 0) = 4666 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_bad_simd                        ... bench:          84 ns/iter (+/- 8) = 880 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> test str::bench_contains_bad_simd                        ... bench:          97 ns/iter (+/- 41) = 762 MB/s
>
> — [github.com/rust-lang/rust/pull/103779](https://github.com/rust-lang/rust/pull/103779)

> However, at the time of writing, it is only used for short needles
>
> — [github.com/BurntSushi/memchr/blob/master/README.md](https://github.com/BurntSushi/memchr/blob/master/README.md)

</details>
