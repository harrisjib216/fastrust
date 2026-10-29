# monoio (io_uring thread-per-core runtime)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Async runtimes](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Thread-per-core io_uring runtime from ByteDance; 2-3x Tokio's peak QPS on a 4-16 core echo server in its 2021 benchmark.

| | |
|---|---|
| **Beats** | Tokio 1.14 multi-threaded runtime with one worker per core (current_thread at 1 core), serving the same TCP ping-pong server |
| **Claimed result** | Best QPS, 100-byte ping-pong, monoio vs Tokio: 1 core 232888 vs 205726, 4 cores 868525 vs 430921, 16 cores 2764152 vs 908179 (1.13x, 2.0x and 3.0x, derived from page data). Text: 'about twice that of Tokio' under 4 cores and 'close to 3 times' under 16 cores. |
| **Workload** | TCP ping-pong: client machines open a fixed number of connections per core, send 1 KB or 100-byte messages, and the server echoes them; best QPS across connection counts, servers on 1 to 20 cores, client and server on separate physical machines. |
| **Metric** | throughput |
| **Hardware** | Intel Xeon Gold 5118 @ 2.30GHz server, Intel X710 10GbE NIC |
| **OS** | Linux 5.15.4 (Arch) |
| **Toolchain** | rust nightly-2021-11-26; Tokio 1.14.0, Glommio 0.6.0, monoio 0.0.9 |
| **Measured** | 2021-12-01 |
| **Adoption** | 5.1k stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | Linux (io_uring on 5.6+, epoll otherwise) and macOS (kqueue) |
| **Install** | cargo add monoio |
| **Links** | [repository](https://github.com/monoio-rs/monoio) · [documentation](https://docs.rs/monoio) · [benchmark](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md) |

## Verdict

For a network server with many connections spread evenly over cores, monoio's peak QPS was 2.0x Tokio's at 4 cores and 3.0x at 16 cores; Tokio's QPS barely grew beyond 8 cores in this test (774526 at 8, 908179 at 16). At 1 core the gap was 1.13x, with few connections Tokio had lower latency, and the comparison is against Tokio 1.14 from 2021.

**Why tier A:** Public benchmark code (monoio-benchmark), raw results, server CPU, NIC, kernel and nightly toolchain all stated; first-party.

## Use it when

- Proxies, gateways and load balancers whose connections can be sharded evenly across cores
- Servers on 4 or more cores that saturate Tokio's multi-threaded scheduler

## Skip it when

- Unbalanced workloads: the README warns idle cores cannot help busy ones
- Few connections or single-core deployments, where Tokio matched or beat it on latency
- Code that depends on the Tokio ecosystem (hyper, tonic, tower) without adapters; monoio's ownership-based I/O traits differ

## How it is fast

Each core runs its own single-threaded executor and io_uring instance, and connections never move between threads, so there is no work-stealing synchronization and tasks need not be Send. I/O is submitted in batches through the ring instead of one readiness syscall per operation, and buffers are passed by ownership to the kernel.

## What it costs

- Tasks are pinned to their thread, so one hot connection can saturate a core while others idle.
- io_uring needs Linux 5.6+ and a raised memlock limit.
- Separate I/O traits limit reuse of Tokio-based libraries.

## Problems with the evidence

- Tokio 1.14 (2021) and nightly-2021-11-26; both runtimes have changed since and no newer comparison was found.
- First-party benchmark on the authors' production network; one workload (echo).
- Monoio's last crates.io release is 0.2.4 (2024-08) even though the repository is active.

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> Under single core, Monoio's performance is slightly better than Tokio; under 4 cores, the peak performance is about twice that of Tokio; under 16 cores, it is close to 3 times.
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> In the case of a single core and very few connections, Monoio's latency will be higher than Tokio, resulting in lower throughput than Tokio.
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> Intel(R) Xeon(R) Gold 5118 CPU @ 2.30GHz
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> rust nightly-2021-11-26
>
> — [github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md](https://github.com/monoio-rs/monoio/blob/master/docs/en/benchmark.md)

> 100B Best QPS Tokio Server 1C 205726 4*130 2C 228719 8*90 4C 430921 16*90 8C 774526 32*90 12C 892601 48*90 16C 908179 64*70
>
> — [github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark…](https://github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark/raw_data.txt)

> Monoio Server 1C 232888 4*150 2C 442257 8*150 4C 868525 16*140 8C 1588528 32*130 12C 2194588 48*120 16C 2764152 64*110
>
> — [github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark…](https://github.com/monoio-rs/monoio/blob/master/.github/resources/benchmark/raw_data.txt)

> name = "tokio" version = "1.14.0"
>
> — [github.com/monoio-rs/monoio-benchmark/blob/master/Cargo.lock](https://github.com/monoio-rs/monoio-benchmark/blob/master/Cargo.lock)

> If the workload is very unbalanced, it may cause performance degradation than Tokio since CPU cores may not be fully utilized.
>
> — [github.com/monoio-rs/monoio](https://github.com/monoio-rs/monoio)

</details>
