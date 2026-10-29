# kanal

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Channels](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> MPMC channel with sync and async APIs that leads most of its own 2025 charts against std mpsc and crossbeam, though not bounded SPSC.

| | |
|---|---|
| **Beats** | std::sync::mpsc (Rust 1.85.1, crossbeam-based) and crossbeam-channel |
| **Claimed result** | Chart only: messages per second in seq, SPSC, MPSC and MPMC tests (AMD Ryzen 9 9950X, rustc 1.85.1, Mar 19, 2025); no numbers in text. |
| **Workload** | fereidani/rust-channel-benchmarks: seq, SPSC, MPSC and MPMC over bounded (size 0, 1, N) and unbounded channels with empty, usize and 8-pointer-sized messages; sync and async variants; compared with crossbeam-channel, std::mpsc, flume, async-channel, futures-channel and Go. |
| **Metric** | throughput |
| **Hardware** | AMD Ryzen 9 9950X, 16 cores |
| **OS** | Ubuntu 24.04, Linux 6.11.0-19-generic |
| **Toolchain** | rustc 1.85.1; Go 1.24.1 |
| **Measured** | 2025-03-19 |
| **Adoption** | 1.8k stars |
| **Last commit** | 2026-07-19 |
| **Platforms** | All std targets |
| **Install** | cargo add kanal |
| **Links** | [repository](https://github.com/fereidani/kanal) · [documentation](https://docs.rs/kanal) · [benchmark](https://github.com/fereidani/kanal) |

## Verdict

In its own 2025 chart, kanal leads most sequential, MPSC and MPMC tests, including against std::sync::mpsc on a current toolchain. It does not lead everywhere: crossbeam-channel and std::sync::mpsc are ahead in some bounded SPSC tests. All numbers are chart-only and first-party.

**Why tier D:** Public benchmark code with machine, rustc and OS stated, but the results are only a chart image.

## Use it when

- MPSC or MPMC traffic where its chart shows the largest lead
- Mixing sync and async endpoints on one channel

## Skip it when

- Bounded single-producer single-consumer pipelines, where crossbeam-channel and std::sync::mpsc lead parts of its own chart
- Code that cannot accept unsafe stack-to-stack copying in a dependency

## How it is fast

Messages no larger than a pointer are encoded into the pointer itself; larger ones are copied directly from the sender's stack to the receiver's, as Go does, so bounded(0) channels need no heap allocation. Channel state sits behind a short, specially tuned mutex whose hold time is predictable.

## What it costs

- Relies on unsafe direct memory copies between stacks.
- Still a 0.1.x crate.

## Problems with the evidence

- Chart image only; no numbers in text.
- First-party benchmark, one machine, no independent rerun found.
- Async results run inside Tokio and are not directly comparable with the thread-based sync tests, as the README itself notes.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> The fast sync and async channel that Rust deserves!
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

> Results are based on how many messages can be passed in each scenario per second.
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

> AMD Ryzen 9 9950X 16-Core Processor
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

> rustc 1.85.1 (4eb161250 2025-03-15)
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

> Date: Mar 19, 2025
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

> utilizing direct memory access to copy objects from the sender's stack or write directly to the receiver's stack
>
> — [github.com/fereidani/kanal](https://github.com/fereidani/kanal)

</details>
