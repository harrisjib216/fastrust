# Tokio multi-threaded scheduler rewrite (2019)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Async runtimes](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Tokio's 2019 scheduler rewrite, the basis of its multi-threaded runtime since 0.2; 12x on one microbenchmark, 34% on a Hyper server.

| | |
|---|---|
| **Beats** | Tokio's previous work-stealing thread pool (tokio-threadpool, the Tokio 0.1 scheduler) |
| **Claimed result** | 'Some benchmarks saw a 10x speed up!' Microbenchmarks old to new: chained_spawn 2,019,796 to 168,854 ns/iter, ping_pong 1,279,948 to 562,659, spawn_many 10,283,608 to 7,320,737, yield_many 21,450,748 to 14,638,563 (12.0x, 2.3x, 1.4x and 1.5x, derived from page data). Hyper hello world: 113923.19 to 152258.70 requests/sec, 'an increase of 34%'; Tonic 'about a 10% speed up'. |
| **Workload** | Tokio scheduler microbenchmarks (recursive spawning, ping-pong messaging, spawning many tasks, frequent yields), plus a Hyper hello-world server under wrk -t1 -c50 -d10. |
| **Metric** | time |
| **Measured** | 2019-10-13 |
| **Adoption** | 33.5k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Linux, macOS, Windows and BSDs |
| **Install** | cargo add tokio --features full |
| **Links** | [repository](https://github.com/tokio-rs/tokio) · [documentation](https://docs.rs/tokio) · [benchmark](https://tokio.rs/blog/2019-10-scheduler) |

## Verdict

The rewrite did make the scheduler faster and became the default multi-threaded runtime. The '10x' headline comes from one spawn microbenchmark (12x); the other microbenchmarks improved 1.4x to 2.3x, a Hyper hello-world server 34%, and Tonic about 10%.

**Why tier C:** First-party blog post with benchmark output but no hardware, toolchain or build flags.

## Use it when

- The default for async Rust: the numbers describe the scheduler you already get from Tokio's multi-threaded runtime

## Skip it when

- Comparing against runtimes benchmarked before late 2019, which measured the old scheduler

## How it is fast

Each worker owns a local run queue that idle workers steal from, and the number of workers searching for work at once is throttled so idle threads do not all contend. A task woken by a message goes into a 'next task' slot and runs next, while the message is still in cache. Task structures were reworked to cut allocations and atomic reference counting.

## What it costs

- Work stealing requires tasks to be Send, unlike thread-per-core runtimes.

## Problems with the evidence

- No hardware, OS or compiler flags stated.
- The 10x figure is from a single microbenchmark; end-to-end gains were 10-34%.
- Benchmarks are from 2019; the scheduler has changed again since (for example cooperative budgeting in 2020).

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> Some benchmarks saw a 10x speed up!
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test chained_spawn ... bench: 2,019,796 ns/iter (+/- 302,168)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test ping_pong ... bench: 1,279,948 ns/iter (+/- 154,365)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test spawn_many ... bench: 10,283,608 ns/iter (+/- 1,284,275)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test yield_many ... bench: 21,450,748 ns/iter (+/- 1,201,337)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test chained_spawn ... bench: 168,854 ns/iter (+/- 8,339)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test ping_pong ... bench: 562,659 ns/iter (+/- 34,410)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test spawn_many ... bench: 7,320,737 ns/iter (+/- 264,620)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> test yield_many ... bench: 14,638,563 ns/iter (+/- 1,573,678)
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> Requests/sec: 113923.19
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> Requests/sec: 152258.70
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> an increase of 34% requests per second just from switching schedulers!
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

> saw about a 10% speed up
>
> — [tokio.rs/blog/2019-10-scheduler](https://tokio.rs/blog/2019-10-scheduler)

</details>
