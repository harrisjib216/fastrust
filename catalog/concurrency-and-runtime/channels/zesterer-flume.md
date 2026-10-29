# flume

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Channels](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0/MIT` · 🟢 active

> MPMC channel with sync and async APIs and no unsafe code; its 'always faster than std mpsc' chart predates std adopting crossbeam's design.

| | |
|---|---|
| **Beats** | std::sync::mpsc (the claim was charted against the pre-1.67 implementation) |
| **Claimed result** | README: 'Always faster than `std::sync::mpsc` and sometimes `crossbeam-channel`'; the only numbers are in a chart from crossbeam's benchmark suite (AMD Ryzen 7 3700x, Linux kernel 5.11.2). |
| **Workload** | crossbeam-channel benchmark suite (seq, spsc, mpsc, mpmc over bounded and unbounded channels). |
| **Metric** | time |
| **Hardware** | AMD Ryzen 7 3700X, 8 cores / 16 threads |
| **OS** | Linux 5.11.2, bfq scheduler |
| **Measured** | 2021 |
| **Adoption** | 3.1k stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | All std targets |
| **Install** | cargo add flume |
| **Links** | [repository](https://github.com/zesterer/flume) · [documentation](https://docs.rs/flume) · [benchmark](https://github.com/zesterer/flume) |

## Verdict

The README's 'always faster than std::sync::mpsc' was charted against the old std channel. Since Rust 1.67 std::sync::mpsc is crossbeam-channel's algorithm, which the same README says flume beats only 'sometimes', so the 'always' no longer holds as stated. No text numbers exist either way.

**Why tier D:** The only numbers are in a chart image from crossbeam's benchmark suite, run by the author on a stated machine.

## Use it when

- You want one channel type usable from both sync and async code
- A no-unsafe dependency policy
- MPMC or select-style waiting with timeouts on stable Rust

## Skip it when

- Single-consumer sync channels on Rust 1.67+, where std::sync::mpsc needs no dependency and no current numbers show flume ahead

## How it is fast

A single mutex-protected queue with a carefully kept short critical section, plus a signal depot for waking blocked senders and receivers. Avoiding unsafe code keeps the design simple; speed comes from minimal work under the lock rather than lock-free structures.

## What it costs

- Lock-based queue: under heavy contention throughput depends on the mutex.
- Optional spin feature trades CPU for latency on some platforms.

## Problems with the evidence

- Chart image only; no numbers in text.
- The chart predates Rust 1.67 (Linux 5.11.2 era), so its std mpsc is the old implementation.
- A 2025 chart in kanal's README (rustc 1.85.1) includes both flume and std::sync::mpsc and does not show flume ahead of std in the bounded SPSC tests (chart reading, not a quotable number).
- The README carries a 'casual maintenance' badge.

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> Always faster than `std::sync::mpsc` and sometimes `crossbeam-channel`
>
> — [github.com/zesterer/flume](https://github.com/zesterer/flume)

> Tests were performed on an AMD Ryzen 7 3700x with 8/16 cores running Linux kernel 5.11.2 with the bfq scheduler.
>
> — [github.com/zesterer/flume](https://github.com/zesterer/flume)

> No `unsafe` code anywhere in the codebase!
>
> — [github.com/zesterer/flume](https://github.com/zesterer/flume)

> in this release the implementation is switched out to be based on crossbeam-channel
>
> — [blog.rust-lang.org/2023/01/26/Rust-1.67.0/](https://blog.rust-lang.org/2023/01/26/Rust-1.67.0/)

</details>
