# glommio (io_uring thread-per-core runtime)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Async runtimes](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟡 stale

> Datadog's thread-per-core io_uring runtime; 1.4-2.3x Tokio's peak QPS at 4-16 cores in a third-party 2021 echo benchmark, slower on one core.

| | |
|---|---|
| **Beats** | Tokio 1.14 multi-threaded runtime with one worker per core (current_thread at 1 core), serving the same TCP ping-pong server |
| **Claimed result** | Best QPS, 100-byte ping-pong in monoio's benchmark, glommio vs Tokio: 16 cores 2064589 vs 908179, 4 cores 616687 vs 430921, 1 core 163709 vs 205726 (2.3x, 1.4x and 0.80x, derived from page data). Glommio's announcement cites outside research that thread-per-core 'can improve tail latencies of applications by up to 71%'. |
| **Workload** | TCP ping-pong echo server, 100-byte messages, client machines driving a fixed number of connections per core; best QPS across connection counts. |
| **Metric** | throughput |
| **Hardware** | Intel Xeon Gold 5118 @ 2.30GHz server, Intel X710 10GbE NIC |
| **OS** | Linux 5.15.4 (Arch) |
| **Toolchain** | rust nightly-2021-11-26; glommio 0.6.0, Tokio 1.14.0 |
| **Measured** | 2021-12-01 |
| **Adoption** | 3.7k stars |
| **Last commit** | 2025-04-21 |
| **Platforms** | Linux only (io_uring, kernel 5.8+) |
| **Install** | cargo add glommio |
| **Links** | [repository](https://github.com/DataDog/glommio) · [documentation](https://docs.rs/glommio) · [benchmark](https://github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark/raw_data.txt) |

## Verdict

In monoio's independent 2021 benchmark, glommio's peak QPS beat Tokio's by 1.4x at 4 cores and 2.3x at 16 cores, but it was 20% slower at 1 core and stayed behind monoio. Glommio itself publishes no runtime benchmark, and the repository has had no commit since April 2025.

**Why tier A:** Public third-party benchmark code and raw results with CPU, NIC, kernel and toolchain stated.

## Use it when

- Sharded storage or data-plane services on Linux with many cores
- Workloads that need per-queue latency classes and shares

## Skip it when

- Single-core or few-connection services, where Tokio was faster in this test
- Portable code: Linux 5.8+ only
- Projects that need an actively maintained runtime: no commits since 2025-04-21

## How it is fast

One executor per pinned core with its own io_uring rings (separate rings for latency-sensitive and regular I/O), so there is no cross-thread task migration or shared queue. Task queues get CPU shares, and latency-sensitive queues can preempt throughput work by polling the latency ring.

## What it costs

- Tasks are pinned to cores; uneven load cannot be rebalanced.
- Linux-only, io_uring-only.
- Own I/O types; the Tokio ecosystem needs adapters.

## Problems with the evidence

- The only numbers come from a competitor's (monoio's) benchmark, using glommio 0.6.0 and Tokio 1.14 from 2021.
- The 71% tail-latency figure in glommio's announcement is from outside research on thread-per-core designs, not a glommio measurement.
- One workload (echo) on one server model.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> 100B Best QPS Tokio Server 1C 205726 4*130 2C 228719 8*90 4C 430921 16*90 8C 774526 32*90 12C 892601 48*90 16C 908179 64*70
>
> — [github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark…](https://github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark/raw_data.txt)

> Glommio Server 1C 163709 4*100 2C 317929 8*100 4C 616687 16*100 8C 1116116 32*90 12C 1616305 48*90 16C 2064589 64*80
>
> — [github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark…](https://github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark/raw_data.txt)

> Glommio and the model are the same as Monoio, so it also has good horizontal scalability, but its peak performance is still a certain gap compared to Monoio.
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> Intel(R) Xeon(R) Gold 5118 CPU @ 2.30GHz
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> rust nightly-2021-11-26
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> Research recently demonstrated that a thread-per-core architecture can improve tail latencies of applications by up to 71%
>
> — [datadoghq.com/blog/engineering/introducing-glommio/](https://www.datadoghq.com/blog/engineering/introducing-glommio/)

> The minimum version at this time is 5.8.
>
> — [github.com/DataDog/glommio](https://github.com/DataDog/glommio)

</details>
