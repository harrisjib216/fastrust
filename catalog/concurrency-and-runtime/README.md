# Concurrency and runtime

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Locks](locks/README.md)

**The default:** std::sync::Mutex and RwLock.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std::sync::Mutex and RwLock (futex-based, Rust 1.62)**](locks/std-sync-mutex.md) | Since Rust 1.62 std's Mutex, RwLock and Condvar on Linux are futex-based; the Mutex ran 2-4x faster than the pthread one it replaced in lock-bench. | B | ✅ Holds up |
| [**spin (spin::Mutex)**](locks/codeberg-org-zesterer-spin.md) | Spin-based Mutex, RwLock and Once for no_std; with std available it trails std's futex Mutex under contention and ties or wins only when idle. | B | 🟡 Holds up in part |
| [**parking_lot (Mutex and RwLock)**](locks/amanieu-parking-lot.md) | Compact adaptive Mutex and RwLock whose old '5x faster than std' claim predates std's futex locks; today std wins under heavy contention. | B | ❌ Doesn't hold up |

## [Channels](channels/README.md)

**The default:** std::sync::mpsc.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std::sync::mpsc (crossbeam-channel implementation, Rust 1.67)**](channels/std-sync-mpsc.md) | Since Rust 1.67 std::sync::mpsc wraps a copy of crossbeam-channel, closing most of the gap that made crossbeam the default recommendation. | D | ✅ Holds up |
| [**crossbeam-channel**](channels/crossbeam-rs-crossbeam-crossbeam-channel.md) | Multi-producer multi-consumer channels with select!; its speed lead over std::sync::mpsc ended when std adopted its implementation in Rust 1.67. | D | 🟡 Holds up in part |
| [**flume**](channels/zesterer-flume.md) | MPMC channel with sync and async APIs and no unsafe code; its 'always faster than std mpsc' chart predates std adopting crossbeam's design. | D | 🟡 Holds up in part |
| [**kanal**](channels/fereidani-kanal.md) | MPMC channel with sync and async APIs that leads most of its own 2025 charts against std mpsc and crossbeam, though not bounded SPSC. | D | 🟡 Holds up in part |

## [Async runtimes](async-runtimes/README.md)

**The default:** Tokio's multi-threaded runtime.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**glommio (io_uring thread-per-core runtime)**](async-runtimes/datadog-glommio.md) | Datadog's thread-per-core io_uring runtime; 1.4-2.3x Tokio's peak QPS at 4-16 cores in a third-party 2021 echo benchmark, slower on one core. | A | 🟡 Holds up in part |
| [**monoio (io_uring thread-per-core runtime)**](async-runtimes/monoio-rs-monoio.md) | Thread-per-core io_uring runtime from ByteDance; 2-3x Tokio's peak QPS on a 4-16 core echo server in its 2021 benchmark. | A | 🟡 Holds up in part |
| [**async-std**](async-runtimes/async-rs-async-std.md) | Discontinued std-like async runtime whose 2019 launch microbenchmarks showed up to 1.95x faster task spawning than Tokio. | C | 🟡 Holds up in part |
| [**Tokio multi-threaded scheduler rewrite (2019)**](async-runtimes/tokio-rs-tokio-scheduler-2019.md) | Tokio's 2019 scheduler rewrite, the basis of its multi-threaded runtime since 0.2; 12x on one microbenchmark, 34% on a Hyper server. | C | 🟡 Holds up in part |

## [Data parallelism](parallelism/README.md)

**The default:** std::thread with hand-split work.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**rayon (work-stealing data parallelism)**](parallelism/rayon-rs-rayon.md) | Work-stealing parallel iterators and join; its published speedups are against sequential code, not against hand-split std::thread. | C | 🟡 Holds up in part |

## [Lazy statics and one-time init](lazy-and-once/README.md)

**The default:** lazy_static.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**once_cell (sync::Lazy)**](lazy-and-once/matklad-once-cell.md) | Lazy statics without a macro, now mostly in std as OnceLock and LazyLock; access speed matches lazy_static, so pick it for API, not speed. | B | ❌ Doesn't hold up |

## [Memory allocators](allocators/README.md)

**The default:** The system allocator (glibc malloc, musl malloc, or the macOS allocator).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**mimalloc (global allocator)**](allocators/purpleprotocol-mimalloc-rust.md) | Rust global allocator for Microsoft's mimalloc: 25x faster than musl's malloc in a 48-core test, 4-54% faster than glibc on most real programs. | A | ✅ Holds up |
| [**snmalloc-rs (snmalloc global allocator)**](allocators/microsoft-snmalloc.md) | Rust bindings for Microsoft's message-passing allocator; 7.2x faster than glibc on cross-thread frees and 4-18% faster on real programs. | A | ✅ Holds up |
| [**rpmalloc (rpmalloc-rs global allocator)**](allocators/embarkstudios-rpmalloc-rs.md) | Rust wrapper for rpmalloc, a thread-caching allocator; upstream beats glibc on most 2026 benchmarks, but the crate bundles a 2022-era snapshot. | A | 🟡 Holds up in part |
| [**tikv-jemallocator (jemalloc global allocator)**](allocators/tikv-jemallocator.md) | Maintained jemalloc bindings that rustc itself uses; 13-14% faster rustc builds on macOS, mixed results against modern glibc. | B | 🟡 Holds up in part |
