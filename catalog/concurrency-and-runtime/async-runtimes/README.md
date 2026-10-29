# Async runtimes

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** Tokio's multi-threaded runtime.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**glommio (io_uring thread-per-core runtime)**](datadog-glommio.md) | Datadog's thread-per-core io_uring runtime; 1.4-2.3x Tokio's peak QPS at 4-16 cores in a third-party 2021 echo benchmark, slower on one core. | A | 🟡 Holds up in part |
| [**monoio (io_uring thread-per-core runtime)**](monoio-rs-monoio.md) | Thread-per-core io_uring runtime from ByteDance; 2-3x Tokio's peak QPS on a 4-16 core echo server in its 2021 benchmark. | A | 🟡 Holds up in part |
| [**async-std**](async-rs-async-std.md) | Discontinued std-like async runtime whose 2019 launch microbenchmarks showed up to 1.95x faster task spawning than Tokio. | C | 🟡 Holds up in part |
| [**Tokio multi-threaded scheduler rewrite (2019)**](tokio-rs-tokio-scheduler-2019.md) | Tokio's 2019 scheduler rewrite, the basis of its multi-threaded runtime since 0.2; 12x on one microbenchmark, 34% on a Hyper server. | C | 🟡 Holds up in part |
