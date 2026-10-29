# wtx (gRPC server)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Protocol Buffers and RPC](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MPL-2.0` · 🟢 active

> Collection of web transports with its own HTTP/2 and gRPC stack; beat tonic in its author's grpc_bench run, lost by ~1.8x in the maintainer's.

| | |
|---|---|
| **Beats** | tonic (rust_tonic_mt and rust_tonic_st entries in grpc_bench, unary RPC) |
| **Claimed result** | wtx author run (Ryzen 9 5900X) req/s wtx vs tonic_mt: 1 CPU 17666 vs 12793, 4 CPUs 62108 vs 48307, 24 CPUs 262037 vs 191674 (1.38x, 1.29x, 1.37x) (derived from page data). Maintainer run (Xeon Gold 6548N) req/s: 1 CPU tonic_st 102754 vs wtx 54652, 2 CPUs tonic_mt 130370 vs wtx 72563 (tonic 1.88x and 1.80x faster) (derived from page data) |
| **Workload** | grpc_bench unary RPC with the complex_proto request and the ghz load generator. Author run: 100 connections, 10,000 concurrency, 10-second runs. Maintainer run: 50 connections, 1,000 concurrency, 120-second runs after 30-second warmup, server limited to 1 or 2 CPUs. |
| **Metric** | throughput |
| **Hardware** | Author run: AMD Ryzen 9 5900X, Ubuntu 24.04. Maintainer run: Intel Xeon Gold 6548N (16 vCPUs), 64 GB, dedicated DigitalOcean server, Ubuntu 24.04. |
| **OS** | Linux (Ubuntu 24.04), in Docker |
| **Toolchain** | Per grpc_bench Dockerfiles; not stated in the results. |
| **Measured** | 2026-04-23 |
| **Adoption** | 405 stars |
| **Last commit** | 2026-10-23 |
| **Platforms** | Not stated; benchmarked on Linux x86_64 |
| **Install** | cargo add wtx |
| **Links** | [repository](https://github.com/c410-f3r/wtx) · [documentation](https://docs.rs/wtx) · [benchmark](https://github.com/LesnyRumcajs/grpc_bench/discussions/559) |

## Verdict

In a run by wtx's author, wtx beat tonic by 1.29x to 1.38x. In the grpc_bench maintainer's newer run, tonic served 1.8x to 1.9x more requests than wtx at 1 and 2 CPUs. With one result each way, the claim holds only on the author's setup.

**Why tier A:** Both runs use the public grpc_bench harness with hardware and parameters stated; one is by wtx's author and one by the benchmark's maintainer.

## Use it when

- You already use wtx for HTTP/2 or WebSocket and want gRPC from the same crate.
- You can benchmark it on your own workload first.

## Skip it when

- You pick it for gRPC speed alone; the independent run had tonic well ahead.
- You rely on the tonic and tower ecosystem (interceptors, tonic-build, reflection).

## How it is fast

wtx implements HTTP/2 and gRPC itself instead of using hyper and h2, with its own buffer management. The author's run shows lower latency percentiles than tonic at the same load.

## What it costs

- MPL-2.0 license (file-level copyleft), unlike tonic's MIT.
- Smaller ecosystem than tonic; no tower integration.

## Problems with the evidence

- In the maintainer's run wtx used only 43% of its one CPU while tonic used 102%, so wtx may have been limited by something other than CPU (configuration, flow control or the client), not by its own speed.
- The two runs differ in hardware, connection count, concurrency and duration, so they cannot be merged into one ratio.
- The maintainer run lists wtx only at 1 and 2 CPUs.

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> | rust_wtx                    |   17666 |      320.13 ms |      598.88 ms |      696.26 ms |      896.41 ms |   13.57% |     28.28 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |   12793 |      527.60 ms |      999.12 ms |         1.40 s |         2.18 s |   17.57% |     27.08 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_wtx                    |   62108 |      112.87 ms |      199.91 ms |      289.31 ms |      404.17 ms |    68.3% |     48.25 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |   48307 |      159.33 ms |      306.74 ms |      442.74 ms |      686.38 ms |   87.36% |     53.73 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_wtx                    |  262037 |       29.67 ms |       54.06 ms |       73.90 ms |      114.26 ms |   353.0% |     107.6 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |  191674 |       43.34 ms |      128.74 ms |      184.38 ms |      265.20 ms |  452.07% |     98.08 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> CPU: AMD Ryzen 9 5900X
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_st               |  102754 |        9.42 ms |       11.75 ms |       12.52 ms |       14.47 ms |  102.08% |     14.97 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

> | rust_wtx                    |   54652 |       18.13 ms |       42.04 ms |       42.67 ms |       44.29 ms |   43.09% |     27.05 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

> | rust_tonic_mt               |  130370 |        7.03 ms |       14.47 ms |       19.07 ms |       27.70 ms |  194.08% |     14.35 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

> | rust_wtx                    |   72563 |       13.60 ms |       41.88 ms |       42.52 ms |       43.81 ms |   88.64% |     32.24 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

> INTEL XEON GOLD 6548N (16) @ 2.800GHz
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

> GRPC_CLIENT_CONNECTIONS=50
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/559](https://github.com/LesnyRumcajs/grpc_bench/discussions/559)

</details>
