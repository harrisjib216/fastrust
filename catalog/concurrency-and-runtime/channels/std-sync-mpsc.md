# std::sync::mpsc (crossbeam-channel implementation, Rust 1.67)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Channels](README.md)</sub>

**Tier D** (directional only) · ✅ Holds up · Ships with Rust · `Rust` · 🟢 active

> Since Rust 1.67 std::sync::mpsc wraps a copy of crossbeam-channel, closing most of the gap that made crossbeam the default recommendation.

| | |
|---|---|
| **Beats** | std::sync::mpsc before Rust 1.67: the original pre-1.0 implementation that upgraded between oneshot, stream and shared flavors at runtime |
| **Claimed result** | No numbers in text: the release post says the new implementation 'improves the performance and maintainability', and the pull request points to crossbeam-channel's chart-only benchmarks as 'more performant in some basic benchmarks'. |
| **Workload** | crossbeam-channel benchmark suite: seq, spsc, mpsc, mpmc and select tests over bounded (capacity 0, 1, N) and unbounded channels, N = 5,000,000 messages, T = 4 threads. |
| **Metric** | time |
| **Hardware** | Intel Core i7-5500U, 2 cores / 4 threads (crossbeam-channel chart) |
| **Toolchain** | rustc 1.63.0 (crossbeam-channel chart, 2022-08-24) |
| **Measured** | 2022-08-24 |
| **Platforms** | All std targets |
| **Install** | built into std since 1.67 |
| **Links** | [documentation](https://doc.rust-lang.org/std/sync/mpsc/) · [benchmark](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md) |

## Verdict

Rust 1.67 replaced the channel internals with a copy of crossbeam-channel, and the release post says this improves performance. The only supporting numbers are in crossbeam-channel's benchmark chart, where it led the old std channel in every test, so the direction is clear but the size of the gain is not stated in text.

**Why tier D:** No numbers in text: the release post and pull request state the improvement, and the benchmark they point to is a chart image.

## Use it when

- Single-consumer message passing between threads; it is already the default
- Projects that want to drop a crossbeam-channel dependency used only for mpsc

## Skip it when

- You need several consumers or select over many channels on stable Rust: std::sync::mpmc is still unstable, so crossbeam-channel or another crate is still required
- async code: use an async channel

## How it is fast

The channel picks its data structure when it is created (an array-based bounded queue, a linked list of blocks for unbounded, or a zero-capacity rendezvous) and keeps it, instead of upgrading flavors at runtime. Senders and receivers use lock-free operations on the queue and park only when they must wait, with brief backoff spinning first.

## What it costs

- Brief spinning before parking trades some CPU for latency.
- The public API is unchanged, so MPMC and select remain unavailable on stable.

## Problems with the evidence

- Numbers exist only in chart images (crossbeam-channel's plot, rustc 1.63.0 on a 2-core laptop).
- No rust-lang measurement of the old vs new std channel was found; rustc-perf results on the PR were reported as neutral for the compiler, which uses channels lightly.
- A reviewer on the PR raised the concern that spin-then-park behavior can raise CPU use compared with sleeping immediately.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> in this release the implementation is switched out to be based on crossbeam-channel
>
> — [blog.rust-lang.org/2023/01/26/Rust-1.67.0/](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0/)

> This release contains no API changes, but the new implementation fixes a number of bugs and improves the performance and maintainability of the implementation.
>
> — [blog.rust-lang.org/2023/01/26/Rust-1.67.0/](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0/)

> sync::mpsc is now implemented as a thin wrapper around sync::mpmc
>
> — [github.com/rust-lang/rust/pull/93563](https://github.com/rust-lang/rust/pull/93563)

> the new implementation has also been shown to be more performant in some basic benchmarks
>
> — [github.com/rust-lang/rust/pull/93563](https://github.com/rust-lang/rust/pull/93563)

> Machine: Intel(R) Core(TM) i7-5500U (2 physical cores, 4 logical cores)
>
> — [github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/ben…](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md)

> rustc 1.63.0 (4b91a6ea7 2022-08-08)
>
> — [github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/ben…](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md)

</details>
