# rayon (work-stealing data parallelism)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Data parallelism](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Work-stealing parallel iterators and join; its published speedups are against sequential code, not against hand-split std::thread.

| | |
|---|---|
| **Beats** | Sequential code on one thread; the catalog default (std::thread with hand-split work) was not measured |
| **Claimed result** | Quicksort on a 4-core Macbook Pro, Rayon vs sequential ('4x is basically the best you could expect'): 'a speedup of 3.10x' at 1024K elements without a sequential cutoff; '0.41x for a 1K array'. |
| **Workload** | Parallel quicksort using rayon::join on arrays from 1K to 1024K elements, with and without falling back to sequential sort below 5K elements. |
| **Metric** | time |
| **Hardware** | 4-core MacBook Pro (model not stated) |
| **Measured** | 2015-12-18 |
| **Adoption** | 13.4k stars |
| **Last commit** | 2026-10-27 |
| **Platforms** | All std targets with threads |
| **Install** | cargo add rayon |
| **Links** | [repository](https://github.com/rayon-rs/rayon) · [documentation](https://docs.rs/rayon) · [benchmark](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/) |

## Verdict

The authors' published numbers compare a 2015 Rayon quicksort with sequential code, not with hand-split std::thread work: 3.10x on 4 cores for large arrays, and 0.41x (slower) for 1K elements. No measurement of Rayon against manually chunked scoped threads was found, so its advantage over the default is load balancing and convenience rather than a measured speedup.

**Why tier C:** First-party blog post from 2015 with the machine named loosely and no toolchain or flags.

## Use it when

- Uneven per-item cost, where static chunking leaves threads idle
- Many small parallel regions, where spawning threads each time costs more than reusing a pool
- Recursive divide-and-conquer algorithms

## Skip it when

- Tiny inputs: the 1K-element quicksort ran at 0.41x of sequential speed without a cutoff
- Uniform work that splits evenly into one chunk per core, where hand-split scoped threads have no scheduler overhead

## How it is fast

rayon::join pushes one closure onto the current worker's deque and runs the other; idle workers steal pending halves, so work splits only when a core is free. A global pool is created once and reused, avoiding per-call thread spawns, and parallel iterators split ranges adaptively.

## What it costs

- A global thread pool that other libraries may also use, which can oversubscribe cores when mixed with other pools.
- Blocking inside Rayon tasks stalls workers.

## Problems with the evidence

- Numbers are from 2015, before Rayon 1.0, on 4 cores.
- The baseline is sequential code; no published comparison against std::thread::scope with hand-split chunks was found.
- A tuned version with a sequential cutoff reached about 4x in the post's table, which is HTML-only and not quoted here.

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> on my 4-core Macbook Pro (hence, 4x is basically the best you could expect)
>
> — [smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-paralle…](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)

> If the array is 512K or 1024K
>
> — [smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-paralle…](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)

> pushing, but we still get a speedup of 3.10x.
>
> — [smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-paralle…](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)

> Still, I would like to do better than 0.41x for a 1K array (and I think we can).
>
> — [smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-paralle…](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)

</details>
