# crossbeam-channel

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Channels](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Multi-producer multi-consumer channels with select!; its speed lead over std::sync::mpsc ended when std adopted its implementation in Rust 1.67.

| | |
|---|---|
| **Beats** | std::sync::mpsc before Rust 1.67; since 1.67 std::sync::mpsc wraps a copy of this implementation |
| **Claimed result** | Chart only: crossbeam's plot (i7-5500U, rustc 1.63.0) shows crossbeam-channel ahead of the old std mpsc in every test; no numbers in text. |
| **Workload** | seq, spsc, mpsc, mpmc, select_rx and select_both over bounded (capacity 0, 1, N) and unbounded channels, N = 5,000,000 messages, T = 4 threads; compared with std mpsc, futures-channel, flume and Go channels. |
| **Metric** | time |
| **Hardware** | Intel Core i7-5500U, 2 cores / 4 threads |
| **OS** | Linux (Go comparison built with linux/amd64) |
| **Toolchain** | rustc 1.63.0; Go 1.19 |
| **Measured** | 2022-08-24 |
| **Adoption** | 8.6k stars |
| **Last commit** | 2026-10-01 |
| **Platforms** | All std targets |
| **Install** | cargo add crossbeam-channel |
| **Links** | [repository](https://github.com/crossbeam-rs/crossbeam) · [documentation](https://docs.rs/crossbeam-channel) · [benchmark](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md) |

## Verdict

crossbeam-channel led the old std channel in every test of its benchmark chart, but Rust 1.67 copied its implementation into std, so for single-consumer use the default now runs the same algorithm. What remains is features (multiple consumers, select!, timers on stable), not speed over std.

**Why tier D:** Benchmark code and machine are public, but the results are a chart image with no numbers in text.

## Use it when

- Multiple consumers on stable Rust
- select! over several channels, or after/tick/never timer channels
- Toolchains older than Rust 1.67

## Skip it when

- Plain single-consumer channels on Rust 1.67+, where std::sync::mpsc is the same code

## How it is fast

Each channel flavor is fixed at creation: a bounded array queue with per-slot stamps, an unbounded linked list of blocks, or a zero-capacity handoff. Operations are lock-free on the fast path, with exponential backoff before a thread parks.

## What it costs

- Extra dependency for functionality std now covers in the single-consumer case.
- Spinning before parking costs some CPU.

## Problems with the evidence

- Results are a chart image only; no numbers in text.
- The chart was made with rustc 1.63.0, before std adopted the implementation, on a 2-core laptop.
- A 2022 third-party evaluation (ytakano/async_bench) also found crossbeam-channel fastest, again chart-only and before Rust 1.67.

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> Machine: Intel(R) Core(TM) i7-5500U (2 physical cores, 4 logical cores)
>
> — [github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/ben…](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md)

> rustc 1.63.0 (4b91a6ea7 2022-08-08)
>
> — [github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/ben…](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md)

> Commit: `7070018` (2022-08-24)
>
> — [github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/ben…](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/benchmarks/README.md)

> sync::mpsc is now implemented as a thin wrapper around sync::mpmc
>
> — [github.com/rust-lang/rust/pull/93563](https://github.com/rust-lang/rust/pull/93563)

> is the fastest. Use this for multi-threaded applications.
>
> — [github.com/ytakano/async_bench](https://github.com/ytakano/async_bench)

</details>
