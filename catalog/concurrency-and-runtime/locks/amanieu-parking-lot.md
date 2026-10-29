# parking_lot (Mutex and RwLock)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Locks](README.md)</sub>

**Tier B** (independent measurement) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Compact adaptive Mutex and RwLock whose old '5x faster than std' claim predates std's futex locks; today std wins under heavy contention.

| | |
|---|---|
| **Beats** | std::sync::Mutex and RwLock on Linux; the README claim was measured against the pre-1.62 pthread-based std locks |
| **Claimed result** | README claim (pthread era): parking_lot::Mutex '1.5x faster than `std::sync::Mutex` when uncontended, and up to 5x faster when contended', RwLock 'up to 50x faster in some cases'. lock-bench extreme contention against the futex std Mutex: 313.074149ms vs 27.645506ms (64C/128T Threadripper); 403.756547ms vs 113.030539ms (INTEL 120 cores); 206.836045ms vs 30.795793ms, and 40.800542ms with the wake-after-release prototype. Light contention: 695.039µs vs 784.189µs; no contention: 940.333µs vs 1.189231ms (parking_lot ahead). |
| **Workload** | matklad/lock-bench: 32 threads incrementing counters behind cache-padded mutexes, 10,000 operations per thread, 100 rounds; 2 locks for extreme contention, 1,000 for light, 1,000,000 for none. Runs from March 2022, November 2023 and May 2025. |
| **Metric** | time |
| **Hardware** | AMD Threadripper 64C/128T (2022); Intel 120-core CPU (2023); not stated (2025) |
| **OS** | Linux |
| **Toolchain** | Not stated; std side is the futex Mutex from Rust 1.62 onward |
| **Measured** | 2022-03-21 |
| **Adoption** | 3.4k stars |
| **Last commit** | 2026-10-21 |
| **Platforms** | All std targets; wasm32-unknown-unknown needs nightly with atomics enabled |
| **Install** | cargo add parking_lot |
| **Links** | [repository](https://github.com/Amanieu/parking_lot) · [documentation](https://docs.rs/parking_lot) · [benchmark](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631) |

## Verdict

The '1.5x faster uncontended, up to 5x faster contended' claim dates from when std wrapped pthread mutexes. Against the futex-based std Mutex (Rust 1.62+), three lock-bench runs from 2022 to 2025 have parking_lot 3.6x to 11x slower under extreme contention, with only a 13-27% edge at light or no contention. The author removed the speed claim from the README in October 2026.

**Why tier B:** Independent of the crate's author: a rust-lang libs measurement plus third-party reruns of the public lock-bench filed on the parking_lot tracker.

## Use it when

- You need features std lacks: upgradable or recursive read locks, downgrade, fair unlock, raw lock/unlock without a guard, no poisoning
- Low-contention locking on Linux, where it was 13-27% faster than the futex std Mutex in the 2022 run
- Targets where std still uses pthread locks (macOS), though no current comparison was found

## Skip it when

- Heavily contended locks on Linux: std's futex Mutex was 3.6x to 11x faster in three independent extreme-contention runs
- You added it only for speed on Rust 1.62 or newer

## How it is fast

Each lock is one byte (Mutex) or one word (RwLock) with an inline fast path of a single atomic operation. Contended threads spin a few times, then park in a global hash table keyed by lock address, modeled on WebKit's WTF::ParkingLot. Before 0.13 the unlocking thread woke a waiter while still holding internal state, which lengthened the critical section under contention; 0.13 (2026-10-21) moved the wake until after release.

## What it costs

- 0.13 raised the MSRV to Rust 1.95 and changed the lock_api raw-lock traits.
- Automatic eventual fairness and hardware lock elision were removed in 0.13.
- Adds a dependency for locks std already provides.

## Problems with the evidence

- parking_lot 0.13.0 (2026-10-21) ships a wake-after-release change for contended unlocks; its only published numbers are from the precursor prototype, which was still 1.3x slower than std in lock-bench.
- lock-bench's critical section is a single increment, the worst case for any lock's wake-up path.
- The README claim was removed on 2026-10-02 in commit 'Correct documentation and feature metadata'; the README text never gave benchmark details or results.
- No post-futex comparison of RwLock was found.

## Evidence

<details>
<summary>17 quotes from the sources, checked 2026-10-28</summary>

> When tested on x86_64 Linux, `parking_lot::Mutex` was found to be 1.5x faster than `std::sync::Mutex` when uncontended, and up to 5x faster when contended from multiple threads.
>
> — [github.com/Amanieu/parking_lot/blob/7a9e143ee9abcbe1d093c9a6cca3976…](https://github.com/Amanieu/parking_lot/blob/7a9e143ee9abcbe1d093c9a6cca3976ecfa097cf/README.md)

> the standard library `RwLock`, and even up to 50x faster in some cases.
>
> — [github.com/Amanieu/parking_lot/blob/7a9e143ee9abcbe1d093c9a6cca3976…](https://github.com/Amanieu/parking_lot/blob/7a9e143ee9abcbe1d093c9a6cca3976ecfa097cf/README.md)

> Notable features of the primitives provided by this library include:
>
> — [github.com/Amanieu/parking_lot](https://github.com/Amanieu/parking_lot)

> Signficantly improved `Mutex` and `RwLock` performance under contention by deferring waking up parked threads until after the lock is released.
>
> — [github.com/Amanieu/parking_lot/blob/master/CHANGELOG.md](https://github.com/Amanieu/parking_lot/blob/master/CHANGELOG.md)

> using a 64C/128T AMD Threadripper
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 27.645506ms min 26.492286ms max 28.900528ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> parking_lot::Mutex avg 313.074149ms min 302.646339ms max 319.321742ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 784.189µs min 663.889µs max 1.156756ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> parking_lot::Mutex avg 695.039µs min 573.318µs max 921.173µs
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> this new std::sync::Mutex avg 1.189231ms min 807.39µs max 1.50386ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> parking_lot::Mutex avg 940.333µs min 643.591µs max 1.14756ms
>
> — [github.com/rust-lang/rust/pull/95035#issuecomment-1073966631](https://github.com/rust-lang/rust/pull/95035#issuecomment-1073966631)

> std::sync::Mutex avg 113.030539ms min 105.760154ms max 131.87258ms
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> parking_lot::Mutex avg 403.756547ms min 326.026509ms max 533.260014ms
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> observed on INTEL 120 cores CPU
>
> — [github.com/Amanieu/parking_lot/issues/418](https://github.com/Amanieu/parking_lot/issues/418)

> std::sync::Mutex avg 30.795793ms min 28.369313ms max 33.668656ms
>
> — [github.com/Amanieu/parking_lot/pull/461](https://github.com/Amanieu/parking_lot/pull/461)

> parking_lot::Mutex (this PR) avg 40.800542ms min 37.16543ms max 44.621677ms
>
> — [github.com/Amanieu/parking_lot/pull/461](https://github.com/Amanieu/parking_lot/pull/461)

> parking_lot::Mutex (master) avg 206.836045ms min 183.902676ms max 213.697023ms
>
> — [github.com/Amanieu/parking_lot/pull/461](https://github.com/Amanieu/parking_lot/pull/461)

</details>
