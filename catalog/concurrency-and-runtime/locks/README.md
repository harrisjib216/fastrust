# Locks

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** std::sync::Mutex and RwLock.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std::sync::Mutex and RwLock (futex-based, Rust 1.62)**](std-sync-mutex.md) | Since Rust 1.62 std's Mutex, RwLock and Condvar on Linux are futex-based; the Mutex ran 2-4x faster than the pthread one it replaced in lock-bench. | B | ✅ Holds up |
| [**spin (spin::Mutex)**](codeberg-org-zesterer-spin.md) | Spin-based Mutex, RwLock and Once for no_std; with std available it trails std's futex Mutex under contention and ties or wins only when idle. | B | 🟡 Holds up in part |
| [**parking_lot (Mutex and RwLock)**](amanieu-parking-lot.md) | Compact adaptive Mutex and RwLock whose old '5x faster than std' claim predates std's futex locks; today std wins under heavy contention. | B | ❌ Doesn't hold up |
