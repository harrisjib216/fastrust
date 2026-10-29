# async-std

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Async runtimes](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0 OR MIT` · 🟡 stale

> Discontinued std-like async runtime whose 2019 launch microbenchmarks showed up to 1.95x faster task spawning than Tokio.

| | |
|---|---|
| **Beats** | Tokio master of early November 2019 (already on the rewritten multi-threaded scheduler) |
| **Claimed result** | Task microbenchmarks, Tokio vs async-std ns/iter: chained_spawn 123,921 vs 119,706 (x 1.04), ping_pong 401,712 vs 289,069 (x 1.39), spawn_many 5,326,354 vs 3,149,276 (x 1.69), yield_many 7,640,958 vs 3,919,748 (x 1.95): 'up to twice as fast as tokio when spawning tasks'. Contended mutex: async_std::sync::Mutex 893,650 vs tokio::sync::Mutex 2,614,997 ns/iter. |
| **Workload** | Tokio's own scheduler microbenchmarks (chained spawn, ping-pong, spawn many, yield many) ported to async-std, plus async mutex contention and creation benchmarks. |
| **Metric** | time |
| **Toolchain** | Nightly pinned in the async-std-1.0-bench tag of matklad/tokio (version not stated in the post) |
| **Measured** | 2019-11-11 |
| **Adoption** | 4.1k stars |
| **Last commit** | 2025-08-15 |
| **Platforms** | Linux, macOS, Windows |
| **Install** | cargo add async-std (discontinued; maintainers recommend smol) |
| **Links** | [repository](https://github.com/async-rs/async-std) · [documentation](https://docs.rs/async-std) · [benchmark](https://async.rs/blog/announcing-async-std-1-0/) |

## Verdict

The 1.0 announcement's microbenchmarks show async-std 1.04x to 1.95x faster than Tokio at spawning tasks and its Mutex about 2.9x faster under contention. These are first-party microbenchmarks with no end-to-end test, nothing newer exists, and the project has been discontinued with its maintainers recommending smol.

**Why tier C:** First-party numbers with public benchmark sources and a pinned toolchain tag, but no hardware stated.

## Use it when

- Maintaining existing async-std code until it is moved to smol

## Skip it when

- New projects: the README says async-std has been discontinued and recommends smol
- Choosing a runtime for speed: the only numbers are 2019 microbenchmarks

## How it is fast

A work-stealing executor derived from crossbeam's deque work with a global injector queue, and an async Mutex that wakes waiters with less bookkeeping than Tokio's semaphore-based Mutex of the time.

## What it costs

- Discontinued: no further fixes expected.
- Smaller library ecosystem than Tokio.

## Problems with the evidence

- Microbenchmarks only, first-party, no hardware stated.
- Tokio's Mutex and scheduler have been reworked since 2019.
- The post itself notes that low-contention applications will not benefit from the Mutex result.

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> chained_spawn 123,921 119,706 x 1.04
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> ping_pong 401,712 289,069 x 1.39
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> spawn_many 5,326,354 3,149,276 x 1.69
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> yield_many 7,640,958 3,919,748 x 1.95
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> async-std is up to twice as fast as tokio when spawning tasks.
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> contention ... bench: 893,650 ns/iter (+/- 44,336)
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> contention ... bench: 2,614,997 ns/iter (+/- 167,533)
>
> — [async.rs/blog/announcing-async-std-1-0/](https://async.rs/blog/announcing-async-std-1-0/)

> `async-std` has been discontinued; use `smol` instead
>
> — [github.com/async-rs/async-std](https://github.com/async-rs/async-std)

</details>
