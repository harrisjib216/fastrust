# pajamax (synchronous gRPC server)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Protocol Buffers and RPC](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Thread-per-connection gRPC server with a gRPC-specific HTTP/2 parser; 3-30% faster than tonic in a third-party run, not the 10x its author cites.

| | |
|---|---|
| **Beats** | tonic (rust_tonic_mt entry in grpc_bench, unary RPC) |
| **Claimed result** | Author claim: up to 10X faster than Tonic. Author run (AMD EPYC 9754, server on 1 CPU) req/s pajamax vs tonic_mt: 183167 vs 62969 at C=10/S=1000, 144842 vs 48014 at C=100/S=1000 (2.91x, 3.02x) (derived from page data). Third-party run (Ryzen 9 5900X) req/s: 1 CPU 13288 vs 12793, 4 CPUs 49806 vs 48307, 24 CPUs 249189 vs 191674 (1.04x, 1.03x, 1.30x) (derived from page data) |
| **Workload** | grpc_bench unary RPC with the complex_proto request, ghz load generator; author run varies connections C (1-100) and concurrent streams S (1-100,000) with the server limited to 1 CPU; third-party run uses 100 connections and 10,000 concurrency with server and client limited to 1, 4 or 24 CPUs. |
| **Metric** | throughput |
| **Hardware** | Author run: AMD EPYC 9754, 16 CPUs, 32 GB, Ubuntu 22.04. Third-party run: AMD Ryzen 9 5900X, Ubuntu 24.04. |
| **OS** | Linux (Ubuntu 22.04 and 24.04), in Docker |
| **Toolchain** | Per grpc_bench Dockerfiles; not stated in the results. |
| **Measured** | 2025-12-09 |
| **Adoption** | 31 stars |
| **Last commit** | 2026-01-25 |
| **Platforms** | Not stated; benchmarked on Linux x86_64 |
| **Install** | cargo add pajamax |
| **Links** | [repository](https://github.com/WuBingzheng/pajamax) · [documentation](https://docs.rs/pajamax) · [benchmark](https://github.com/LesnyRumcajs/grpc_bench/discussions/547) |

## Verdict

The "up to 10X faster than Tonic" claim divides requests per second by measured CPU use; in raw requests per second the author's own run peaks at 2.9x-3.0x. A third-party grpc_bench run found pajamax 3-4% ahead of tonic on 1 and 4 CPUs and 30% ahead on 24.

**Why tier A:** Both runs use the public grpc_bench harness with hardware and parameters stated; one run is by pajamax's author, the other by an unrelated maintainer.

## Use it when

- Internal unary-only gRPC services behind a gateway, with few long-lived connections carrying many concurrent streams.
- Handlers are synchronous and fast.

## Skip it when

- You need streaming RPCs or gRPC headers such as grpc-timeout; pajamax supports unary calls only.
- The server faces many short-lived connections from the internet; one thread per connection does not scale there.
- Handlers call async libraries.

## How it is fast

Each connection gets its own OS thread running synchronous code, so there is no async runtime between the socket and the handler. Its HTTP/2 layer is written for gRPC only: it ignores headers other than :path and handles :path without allocating an owned String. Many streams on one connection let one read syscall carry many requests.

## What it costs

- Unary RPC only; no streaming, no gRPC headers such as grpc-timeout.
- One OS thread per connection.
- Very small user base (31 GitHub stars).

## Problems with the evidence

- The 10x figure is req/s per CPU, and the CPU readings for pajamax are implausibly low in places (10.49% CPU while serving 48,265 req/s), so the per-CPU ratio may overstate the gap.
- The author run limits the server to one CPU; the third-party run that found a 3-4% gap at 1 and 4 CPUs was published by the author of a competing library (wtx).
- grpc_bench marks pajamax's implementation as on hold (rust_pajamax_bench_onhold), and the maintainer's 2026-04 run does not include it.

## Evidence

<details>
<summary>19 quotes from the sources, checked 2026-10-28</summary>

> We measured that Pajamax is up to at most 10X faster than Tonic using
>
> — [github.com/WuBingzheng/pajamax/blob/3dc604f27ce6b3ee17fa1eaa1908ae6…](https://github.com/WuBingzheng/pajamax/blob/3dc604f27ce6b3ee17fa1eaa1908ae6bacd5b930/pajamax/src/lib.rs)

> | rust_pajamax       | 183167 |   43.55% |      3.90 ms |   6.91 ms |   9.00 ms |  13.62 ms |     1.66 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> | rust_tonic_mt      |  62969 |  105.96% |     15.74 ms |  60.15 ms |  95.24 ms | 148.14 ms |    12.07 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> | rust_pajamax       | 144842 |    98.6% |      5.25 ms |   9.80 ms |  14.04 ms |  25.94 ms |     9.03 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> | rust_tonic_mt      |  48014 |   104.5% |     20.70 ms |  74.04 ms |  90.44 ms | 144.46 ms |    14.62 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> | rust_pajamax       |  48265 |   10.49% |      1.38 ms |  10.16 ms |  12.79 ms |  22.11 ms |   634.67 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> In most cases, there is a significant difference in CPU usage between Pajamax and Tonic.
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> --C=10:
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> --C=100:
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> S=1000-----^7.1
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> S=1000-----^3.2
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> CPU: AMD EPYC 9754 128-Core Processor, 16 CPUs
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/521](https://github.com/LesnyRumcajs/grpc_bench/discussions/521)

> | rust_pajamax                |   13288 |      474.76 ms |      895.03 ms |         1.30 s |         1.74 s |    4.51% |      4.48 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |   12793 |      527.60 ms |      999.12 ms |         1.40 s |         2.18 s |   17.57% |     27.08 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_pajamax                |   49806 |      143.18 ms |      255.69 ms |      391.85 ms |      640.01 ms |   10.38% |       4.5 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |   48307 |      159.33 ms |      306.74 ms |      442.74 ms |      686.38 ms |   87.36% |     53.73 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_pajamax                |  249189 |       32.43 ms |       69.36 ms |      109.51 ms |      188.29 ms |   45.93% |      5.04 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> | rust_tonic_mt               |  191674 |       43.34 ms |      128.74 ms |      184.38 ms |      265.20 ms |  452.07% |     98.08 MiB |
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

> CPU: AMD Ryzen 9 5900X
>
> — [github.com/LesnyRumcajs/grpc_bench/discussions/547](https://github.com/LesnyRumcajs/grpc_bench/discussions/547)

</details>
