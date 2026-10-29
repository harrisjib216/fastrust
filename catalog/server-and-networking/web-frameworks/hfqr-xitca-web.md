# xitca-web (HTTP library and web framework)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Web frameworks](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0` · 🟢 active

> Rust web framework with its own HTTP stack; ahead of axum on pipelined plaintext in TechEmpower, level on JSON and fortunes.

| | |
|---|---|
| **Beats** | axum 0.7.9 on hyper 1.x and Tokio, TechEmpower Round 23 entries (axum for plaintext/JSON, axum-pg for fortunes) |
| **Claimed result** | TechEmpower Round 23 peak total requests per 15-second run, xitca-web vs axum: plaintext 276548000 vs 185884960, JSON 40804952 vs 40646919, fortunes 17200685 vs 16713989; per second, derived from page data: 18.4M vs 12.4M req/s (1.49x), 2.72M vs 2.71M (1.00x), 1.15M vs 1.11M (1.03x) |
| **Workload** | TechEmpower Framework Benchmarks. plaintext: "Hello, World!" with HTTP/1.1 pipelining at 256 to 16,384 connections; JSON: one small object serialized per request at 16 to 512 connections; fortunes: fetch 12 rows from Postgres, add one, sort, render an escaped HTML template. Peak requests per second across the concurrency levels, 15-second runs. |
| **Metric** | throughput |
| **Hardware** | TechEmpower Round 23: ProLiant DL360 Gen10 Plus servers, Intel Xeon Gold 6330 @ 2.00GHz (56 cores), 64 GB, 40 Gbps Ethernet (separate server, database and load-generator machines) |
| **OS** | Linux, in Docker |
| **Toolchain** | xitca-web: rust:1.81 image, features io-uring,pg,router,template, rustflags "-C target-cpu=native --cfg tokio_unstable", lto, codegen-units 1, panic abort. axum: rust:1.83, target-cpu=native, lto fat, codegen-units 1. |
| **Measured** | 2025-01-30 |
| **Adoption** | 1.0k stars |
| **Last commit** | 2026-10-22 |
| **Platforms** | Benchmarked on Linux x86_64 with io_uring |
| **Install** | cargo add xitca-web |
| **Links** | [repository](https://github.com/HFQR/xitca-web) · [documentation](https://docs.rs/xitca-web) · [benchmark](https://www.techempower.com/benchmarks/#section=data-r23&hw=ph&test=fortune) |

## Verdict

xitca-web beats axum by 1.49x only on pipelined plaintext. On JSON the two are level (1.00x) and on fortunes xitca-web is 3% ahead, which is within run-to-run noise for TechEmpower.

**Why tier A:** TechEmpower runs every implementation itself on stated hardware and publishes the source and Dockerfile for each entry.

## Use it when

- You want an all-in-one stack with HTTP/3 and are fine leaving the hyper and tower ecosystem.
- Your traffic is dominated by tiny pipelined requests, the one case where TechEmpower shows a clear lead.

## Skip it when

- You are choosing it for speed on ordinary JSON or database-backed endpoints: Round 23 shows no meaningful gap over a tuned axum.
- You depend on tower middleware or crates written against hyper's types.

## How it is fast

xitca-web ships its own HTTP/1, HTTP/2 and HTTP/3 implementation instead of building on hyper. The TechEmpower entry enables io_uring for socket I/O and target-cpu=native, and is built with LTO, one codegen unit and panic=abort.

## What it costs

- Smaller ecosystem and user base than axum or actix-web.
- MSRV 1.98 for the latest release, newer than most frameworks require.

## Problems with the evidence

- TechEmpower implementations are written and tuned by each framework's community (custom allocators, target-cpu=native, LTO, thread-per-core setups), so they show a ceiling, not what a default application gets.
- The plaintext test uses HTTP pipelining, which almost no real client does. Servers that batch pipelined responses into one write post numbers several times higher than they reach on ordinary traffic. Fortunes is the most realistic test, and the database driver weighs on it as much as the framework does.
- Requests per second are derived from totalRequests divided by the 15-second duration in TechEmpower's raw results file. The interactive results page draws the same values with JavaScript.
- Round 23 (run 2025-01-30, published 2025-03-17) is the most recent official round, and the TechEmpower FrameworkBenchmarks repository is now archived (last push 2026-03-24), so no newer TechEmpower data is coming.
- The Round 23 xitca-web entry uses io_uring, a Linux-only I/O path, while axum uses epoll through Tokio.
- The separate xitca-web-unrealistic entry, which TechEmpower classifies as stripped, posts higher numbers and should not be compared with axum.
- xitca-web's README makes no numeric performance claim; the reputation comes from TechEmpower rankings.

## Evidence

<details>
<summary>14 quotes from the sources, checked 2026-10-28</summary>

> "xitca-web": [ { "latencyAvg": "2.84ms", "latencyStdev": "8.10ms", "latencyMax": "42.48ms", "totalRequests": 196095856, "startTime": 1738273499, "endTime": 1738273514 }, { "latencyAvg": "595.97us", "latencyStdev": "1.36ms", "latencyMax": "43.37ms", "totalRequests": 276548000
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "188.83us", "latencyStdev": "134.69us", "latencyMax": "19.43ms", "totalRequests": 170630176, "startTime": 1738792996, "endTime": 1738793011 }, { "latencyAvg": "756.90us", "latencyStdev": "487.97us", "latencyMax": "26.12ms", "totalRequests": 185884960
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "xitca-web": [ { "latencyAvg": "511.00us", "latencyStdev": "246.03us", "latencyMax": "1.13ms", "totalRequests": 466904, "startTime": 1738273000, "endTime": 1738273015 }, { "latencyAvg": "50.77us", "latencyStdev": "25.23us", "latencyMax": "0.96ms", "totalRequests": 9578150, "startTime": 1738273017, "endTime": 1738273032 }, { "latencyAvg": "55.56us", "latencyStdev": "19.89us", "latencyMax": "802.00u…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "571.66us", "latencyStdev": "177.18us", "latencyMax": "1.15ms", "totalRequests": 417065, "startTime": 1738792863, "endTime": 1738792878 }, { "latencyAvg": "522.36us", "latencyStdev": "200.74us", "latencyMax": "1.29ms", "totalRequests": 911833, "startTime": 1738792880, "endTime": 1738792895 }, { "latencyAvg": "53.42us", "latencyStdev": "19.10us", "latencyMax": "1.61ms", "t…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "xitca-web": [ { "latencyAvg": "1.33ms", "latencyStdev": "202.07us", "latencyMax": "2.09ms", "totalRequests": 179632, "startTime": 1738273366, "endTime": 1738273381 }, { "latencyAvg": "1.28ms", "latencyStdev": "227.63us", "latencyMax": "2.29ms", "totalRequests": 374006, "startTime": 1738273383, "endTime": 1738273398 }, { "latencyAvg": "1.12ms", "latencyStdev": "292.52us", "latencyMax": "2.53ms", "…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum-pg": [ { "latencyAvg": "1.33ms", "latencyStdev": "180.99us", "latencyMax": "2.07ms", "totalRequests": 179234, "startTime": 1738790959, "endTime": 1738790974 }, { "latencyAvg": "1.31ms", "latencyStdev": "233.59us", "latencyMax": "2.35ms", "totalRequests": 365430, "startTime": 1738790976, "endTime": 1738790991 }, { "latencyAvg": "1.14ms", "latencyStdev": "293.02us", "latencyMax": "2.68ms", "to…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> RUN cargo build --release --bin xitca-web --features io-uring,pg,router,template
>
> — [github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d…](https://github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d775a749ad060753e26e3a/frameworks/Rust/xitca-web/xitca-web.dockerfile)

> "name": "Continuous Benchmarking Run 2025-01-30 18:47:41"
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "duration": 15, "rawData": {
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> Intel Xeon Gold 6330 CPU @ 2.00GHz (56 cores)
>
> — [techempower.com/blog/2025/03/17/framework-benchmarks-round-23/](https://www.techempower.com/blog/2025/03/17/framework-benchmarks-round-23/)

> Mellanox Technologies MT28908 Family [ConnectX-6] 40Gbps Ethernet
>
> — [techempower.com/blog/2025/03/17/framework-benchmarks-round-23/](https://www.techempower.com/blog/2025/03/17/framework-benchmarks-round-23/)

> Requests will be sent using HTTP pipelining.
>
> — [github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information…](https://github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information-Framework-Tests-Overview)

> exercises the ORM, database connectivity, dynamic-size collections, sorting, server-side templates, XSS countermeasures, and character encoding
>
> — [github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information…](https://github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information-Framework-Tests-Overview)

> Start a single-threaded tokio runtime on multiple threads.
>
> — [github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d…](https://github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d775a749ad060753e26e3a/frameworks/Rust/axum/src/server.rs)

</details>
