# std::sync::Mutex and RwLock (futex-based, Rust 1.62)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Locks](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · 🟢 active

> Since Rust 1.62 std's Mutex, RwLock and Condvar on Linux are futex-based; the Mutex ran 2-4x faster than the pthread one it replaced in lock-bench.

| | |
|---|---|
| **Beats** | std::sync::Mutex before Rust 1.62 on Linux: a heap-allocated pthread_mutex_t from glibc or musl |
| **Claimed result** | lock-bench, 32 threads on a 64C/128T Threadripper, new futex Mutex vs the old glibc pthread Mutex: extreme contention 27.645506ms vs 58.506588ms, heavy 1.822707ms vs 8.10534ms, light 784.189µs vs 2.802436ms, no contention 1.189231ms vs 2.97886ms (2.1x, 4.4x, 3.6x and 2.5x faster, derived from page data). The old musl Mutex took 89.146548ms in the extreme case. Size: the pthread mutex 'is 40 bytes' and was boxed; the new Mutex 'only needs 5 bytes'. |
| **Workload** | matklad/lock-bench: 32 threads each increment randomly chosen counters, each behind its own cache-padded Mutex, 10,000 times per round, 100 rounds. The number of locks sets the contention level: 2 (extreme), 64 (heavy), 1,000 (light), 1,000,000 (none). Critical section is a single integer increment. |
| **Metric** | time |
| **Hardware** | AMD Threadripper, 64 cores / 128 threads (model not stated) |
| **OS** | Linux (x86_64-unknown-linux-gnu, plus a musl build of the old std for comparison) |
| **Toolchain** | Nightly build of rust-lang/rust PR #95035 vs the std it replaced (March 2022); build flags not stated in the comment |
| **Measured** | 2022-03-21 |
| **Platforms** | Futex-based on Linux (1.62) and Windows (1.78), and today also on Android, FreeBSD, OpenBSD, DragonFly and wasm with atomics; macOS and other Unix targets still use pthread mutexes |
| **Install** | built into std since 1.62 (Linux) |
| **Links** | [documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html) · [benchmark](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631) |

## Verdict

On the pull request that made the change, its author, Mara Bos of the Rust library team, ran matklad's lock-bench at four contention levels. The futex-based Mutex finished 2.1x to 4.4x faster than the glibc pthread version it replaced, and the lock state shrank from a boxed 40-byte pthread mutex to 5 bytes. This is the default every Rust program on Linux has had since 1.62.

**Why tier B:** The rust-lang project measuring its own std change with a public benchmark on a stated machine; one run, and the build flags are not given in the comment.

## Use it when

- Shared mutable state in any Rust program on Linux; it is already the default
- Short critical sections under contention, where the old pthread lock was 2-4x slower in this benchmark
- Locks in statics: Mutex::new and RwLock::new are const since Rust 1.63

## Skip it when

- Code that needs lock features std does not offer (upgradable reads, fair unlock, raw locking); that is a features question, not a speed one
- async code that holds a lock across an .await point; use an async-aware lock there

## How it is fast

The Mutex is one 32-bit atomic with three states: unlocked, locked, and locked with waiters. Locking and unlocking without contention is a single atomic operation; under contention a thread spins briefly, then sleeps in the kernel with FUTEX_WAIT, and unlock only makes the FUTEX_WAKE syscall when the state says someone is waiting. There is no heap allocation, so the lock is a plain value that can sit in a static or a cache-padded array.

## What it costs

- No fairness guarantee: a thread that just unlocked can re-acquire before a sleeping waiter wakes.
- Poisoning on panic remains, which adds an unwrap or into_inner at each lock site.
- On macOS and other non-futex Unix targets std still uses pthread locks, so these numbers do not carry over there.

## Problems with the evidence

- One run on one 64-core machine, with an increment as the critical section; real programs spend more time inside the lock.
- The comment does not state build flags; lock-bench's own instructions use cargo run --release.
- The same lock-bench extreme-contention case was rerun by third parties in 2023 (Intel 120-core) and 2025, with the futex std Mutex again ahead of parking_lot and spin (see those records).

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> using a 64C/128T AMD Threadripper
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> The 'extreme contention' scenario (32 2 10000 100):
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 27.645506ms min 26.492286ms max 28.900528ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> current std::sync::Mutex (gnu) avg 58.506588ms min 45.996402ms max 64.50891ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> current std::sync::Mutex (musl) avg 89.146548ms min 85.399849ms max 90.148675ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 1.822707ms min 1.674482ms max 2.081187ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> current std::sync::Mutex (gnu) avg 8.10534ms min 7.774234ms max 9.279657ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 784.189µs min 663.889µs max 1.156756ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> current std::sync::Mutex (gnu) avg 2.802436ms min 2.483913ms max 3.781072ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 1.189231ms min 807.39µs max 1.50386ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> current std::sync::Mutex (gnu) avg 2.97886ms min 2.079744ms max 3.96414ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> The mutex implementation, for example, is 40 bytes and cannot be moved.
>
> — [blog.rust-lang.org/2022/06/30/Rust-1.62.0/](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0/)

> In 1.62.0 Mutex only needs 5 bytes for its internal state on Linux, though this may change in future versions.
>
> — [blog.rust-lang.org/2022/06/30/Rust-1.62.0/](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0/)

</details>
