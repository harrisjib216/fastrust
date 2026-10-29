# spin (spin::Mutex)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Locks](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Spin-based Mutex, RwLock and Once for no_std; with std available it trails std's futex Mutex under contention and ties or wins only when idle.

| | |
|---|---|
| **Beats** | std::sync::Mutex on Linux (futex-based since 1.62; pthread-based in the 2020 run) |
| **Claimed result** | Belief tested: 'For short critical sections, spinlocks perform better'. lock-bench vs the futex std Mutex (64C/128T Threadripper): extreme contention 42.427277ms vs 27.645506ms, heavy 2.129469ms vs 1.822707ms, light 604.818µs vs 784.189µs, none 765.141µs vs 1.189231ms. Extreme contention elsewhere: 161.125953ms vs 113.030539ms (INTEL 120 cores); 63.898884ms vs 30.795793ms. With 32 threads on 4 core/8 hyperthreads (pthread-era std), heavy contention: spin avg 55ms, max 161ms vs std avg 21ms, max 23ms. |
| **Workload** | matklad/lock-bench: 32 threads incrementing counters behind cache-padded locks, 10,000 operations per thread, 100 rounds; 2, 64, 1,000 or 1,000,000 locks for extreme, heavy, light and no contention. Runs from January 2020, March 2022, November 2023 and May 2025. |
| **Metric** | time |
| **Hardware** | 4-core/8-thread desktop (2020); AMD Threadripper 64C/128T (2022); Intel 120-core CPU (2023); not stated (2025) |
| **OS** | Linux |
| **Toolchain** | cargo run --release (2020 run); not stated for later runs |
| **Measured** | 2022-03-21 |
| **Adoption** | 593 stars |
| **Last commit** | 2026-08-17 |
| **Platforms** | Any target with atomics; no_std |
| **Install** | cargo add spin |
| **Links** | [repository](https://codeberg.org/zesterer/spin) · [documentation](https://docs.rs/spin) · [benchmark](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631) |

## Verdict

The idea that a spinlock beats a sleeping mutex for short critical sections holds only with light contention and spare cores: there spin::Mutex was 1.3x to 1.55x faster than the futex std Mutex. Under heavy or extreme contention it was 1.2x to 2.1x slower in three runs, and with more threads than cores it was 2.6x slower with a worst case seven times the mutex's. The crate's own README sends std users back to std::sync.

**Why tier B:** Independent of the crate's authors: matklad's published lock-bench, a rust-lang libs rerun, and third-party reruns, all with the benchmark code public.

## Use it when

- no_std code (kernels, firmware) with no OS to sleep on
- Threads pinned one per core with very short critical sections and light contention
- Interrupt-free contexts where blocking is not allowed

## Skip it when

- Any program that can use std: std::sync::Mutex already spins briefly before sleeping
- More runnable threads than cores: a preempted holder leaves every waiter spinning
- Heavily contended locks, where it was 1.2x to 2.6x slower in every run

## How it is fast

Acquiring the lock is a compare-and-swap loop on one atomic flag with no syscall, so an uncontended or briefly held lock never touches the kernel. Waiting threads keep spinning on the CPU instead of sleeping. That avoids wake-up latency when every waiter has its own core, and wastes CPU when the lock holder has been preempted.

## What it costs

- Burns CPU while waiting and has no fairness by default (ticket locks are available as an option).
- Priority inversion risk on preemptive systems: a low-priority holder can be starved by spinning waiters.

## Problems with the evidence

- The favorable numbers (light and no contention) come from one 64-core machine running 32 threads, so every thread had a core.
- lock-bench's critical section is one increment; longer critical sections make spinning worse.
- The GitHub repository (zesterer/spin-rs) is archived and development moved to Codeberg; the star count is from GitHub.
- The spin crate itself makes no speed claim over std; the belief tested here is the common one matklad's post set out to check.

## Evidence

<details>
<summary>19 quotes from the sources, checked 2026-10-28</summary>

> For short critical sections, spinlocks perform better
>
> — [matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html](https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html)

> We use 32 threads (on 4 core/8 hyperthreads CPU)
>
> — [matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html](https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html)

> std::sync::Mutex avg 21ms min 11ms max 23ms
>
> — [matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html](https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html)

> spin::Mutex avg 55ms min 7ms max 161ms
>
> — [matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html](https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html)

> using a 64C/128T AMD Threadripper
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 27.645506ms min 26.492286ms max 28.900528ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> spin::Mutex avg 42.427277ms min 39.371153ms max 44.045329ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 1.822707ms min 1.674482ms max 2.081187ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> spin::Mutex avg 2.129469ms min 2.024791ms max 2.266706ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 784.189µs min 663.889µs max 1.156756ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> spin::Mutex avg 604.818µs min 501.494µs max 841.242µs
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 1.189231ms min 807.39µs max 1.50386ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> spin::Mutex avg 765.141µs min 565.754µs max 983.661µs
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> std::sync::Mutex avg 113.030539ms min 105.760154ms max 131.87258ms
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> spin::Mutex avg 161.125953ms min 151.708034ms max 177.132377ms
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> observed on INTEL 120 cores CPU
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> std::sync::Mutex avg 30.795793ms min 28.369313ms max 33.668656ms
>
> — [github.com/Amanieu/parking_lot/pull/461](https://github.com/Amanieu/parking_lot/pull/461)

> spin::Mutex avg 63.898884ms min 58.45244ms max 74.323676ms
>
> — [github.com/Amanieu/parking_lot/pull/461](https://github.com/Amanieu/parking_lot/pull/461)

> If you have access to `std`, it's likely that the primitives in `std::sync` will serve you better except in very specific circumstances.
>
> — [github.com/zesterer/spin-rs/blob/master/README.md](https://github.com/zesterer/spin-rs/blob/master/README.md)

</details>
