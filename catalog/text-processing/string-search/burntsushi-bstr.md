# bstr (byte strings with reusable Finder and line iteration)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Text processing](../README.md) › [String and byte search](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Byte-string extension traits for &[u8]; a line-by-line grep ran 2.8x faster than String plus str::contains, measured before Rust 1.67.

| | |
|---|---|
| **Beats** | std line-by-line search: BufRead::read_line into a String, then str::contains |
| **Claimed result** | 2.80x faster end to end (252.9 ms vs 707.7 ms) with Finder plus for_byte_line_with_terminator; 2.15x (329.6 ms vs 708.1 ms) with Finder alone; 1.29x (549.6 ms vs 710.2 ms) for the same contains call on bytes instead of a String |
| **Workload** | Print every line containing 'Sushi' from a Rust source file duplicated five times (/tmp/rust.5x.rs) read on stdin; hyperfine with 5 warmup runs. |
| **Metric** | time |
| **Toolchain** | cargo build --release (rustc version not stated) |
| **Measured** | 2022-09-07 |
| **Adoption** | 1.1k stars |
| **Last commit** | 2026-08-10 |
| **Platforms** | All Rust targets; no_std with or without alloc |
| **Install** | cargo add bstr |
| **Links** | [repository](https://github.com/BurntSushi/bstr) · [documentation](https://docs.rs/bstr) · [benchmark](https://blog.burntsushi.net/bstr/) |

## Verdict

The author's grep-style program ran 2.80x faster than the std version, but only 1.29x of that came from skipping UTF-8 validation; the rest came from building the searcher once and avoiding a copy. The test was run in September 2022, before Rust 1.67 added the SSE2 short-needle path to str::contains, which removes much of the per-call setup the post blamed for about 25% of the std program's runtime. The gap today is likely smaller; no newer measurement was found.

**Why tier C:** First-party blog post with full program listings and hyperfine output, but no CPU, OS or rustc version.

## Use it when

- Line-oriented processing of files that may not be valid UTF-8, such as logs or source trees
- The same needle searched across many lines or records

## Skip it when

- Data is already a &str and is searched once
- Downstream code needs guaranteed-valid UTF-8 strings

## How it is fast

bstr works on &[u8], so lines need no UTF-8 validation. Its Finder (memchr::memmem underneath) is built once and reused for every line instead of being rebuilt per contains call, and BufReadExt::for_byte_line_with_terminator hands out lines straight from the reader's buffer without copying them into a caller-owned buffer.

## What it costs

- Byte strings carry no UTF-8 guarantee; APIs that need &str still require validation
- Extra dependency (bstr pulls in memchr)

## Problems with the evidence

- Measured before Rust 1.67's str::contains SSE2 path for short needles; 'Sushi' is a 5-byte needle that path covers
- No hardware or rustc version stated
- One workload with a rare match; dense matches would shift time to output

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> Time (mean ± σ):     707.7 ms ±   3.0 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> Time (mean ± σ):     252.9 ms ±   1.1 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> 2.80 ± 0.02 times faster than
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> Time (mean ± σ):     708.1 ms ±   2.4 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> Time (mean ± σ):     329.6 ms ±   2.9 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> 2.15 ± 0.02 times faster than
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> Time (mean ± σ):     710.2 ms ±   7.7 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> Time (mean ± σ):     549.6 ms ±   2.9 ms
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> 1.29 ± 0.02 times faster than
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

> a significant portion of the runtime of our program (~25% from a quick glance at a profile) is the construction of the substring searcher
>
> — [blog.burntsushi.net/bstr/](https://blog.burntsushi.net/bstr/)

</details>
