# may_minihttp (HTTP/1.1 server on stackful coroutines)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Web frameworks](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Minimal HTTP/1.1 server on the may coroutine runtime; tops TechEmpower plaintext but leads axum by only 14-19% on JSON and fortunes.

| | |
|---|---|
| **Beats** | axum 0.7.9 on hyper 1.x and Tokio, TechEmpower Round 23 entries (axum for plaintext/JSON, axum-pg with tokio-postgres for fortunes) |
| **Claimed result** | TechEmpower Round 23 peak total requests per 15-second run, may-minihttp vs axum: plaintext 418596352 vs 185884960, JSON 46530940 vs 40646919, fortunes 19910679 vs 16713989; per second, derived from page data: 27.9M vs 12.4M req/s (2.25x), 3.10M vs 2.71M (1.14x), 1.33M vs 1.11M (1.19x) |
| **Workload** | TechEmpower Framework Benchmarks. plaintext: "Hello, World!" with HTTP/1.1 pipelining at 256 to 16,384 connections; JSON: one small object serialized per request at 16 to 512 connections; fortunes: fetch 12 rows from Postgres, add one, sort, render an escaped HTML template. Peak requests per second across the concurrency levels, 15-second runs. |
| **Metric** | throughput |
| **Hardware** | TechEmpower Round 23: ProLiant DL360 Gen10 Plus servers, Intel Xeon Gold 6330 @ 2.00GHz (56 cores), 64 GB, 40 Gbps Ethernet (separate server, database and load-generator machines) |
| **OS** | Linux, in Docker |
| **Toolchain** | may-minihttp: rust:1.79 image, RUSTFLAGS="-C target-cpu=native", opt-level 3, codegen-units 1, lto thin, panic abort, mimalloc. axum: rust:1.83, target-cpu=native, lto fat, codegen-units 1. |
| **Measured** | 2025-01-30 |
| **Adoption** | 943 stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | x86_64 Linux, Windows and macOS; AArch64 Linux and macOS (the may runtime's supported list) |
| **Install** | cargo add may_minihttp |
| **Links** | [repository](https://github.com/Xudong-Huang/may_minihttp) · [documentation](https://docs.rs/may_minihttp) · [benchmark](https://www.techempower.com/benchmarks/#section=data-r23&hw=ph&test=fortune) |

## Verdict

It leads axum by 2.25x only on pipelined plaintext. On JSON the gap is 14% and on fortunes, the test with a database and templates, 19%. It is also a minimal HTTP/1.1 server, not a framework with routing and extractors.

**Why tier A:** TechEmpower runs every implementation itself on stated hardware and publishes the source and Dockerfile for each entry.

## Use it when

- You control the clients and serve very high rates of small HTTP/1.1 requests, especially pipelined ones.
- You prefer synchronous-looking handler code over async/await and can accept the may runtime.

## Skip it when

- You need HTTP/2, TLS termination, routing, extractors or the tower middleware ecosystem.
- Your handlers call into async libraries built for Tokio.
- Your requests spend most of their time in a database or downstream service; the gap over axum shrinks to under 20% there.

## How it is fast

Handlers run on stackful coroutines from the may runtime, so blocking-style code needs no async state machines. Each read decodes every request already in the buffer, encodes all their responses into one buffer and sends them with a single write_all, so a pipelined batch costs one write. The TechEmpower entry adds mimalloc, a custom Postgres driver (may_postgres) and target-cpu=native.

## What it costs

- The may runtime documents patterns that are unsafe inside coroutines, such as touching thread-local storage, and each coroutine has a fixed stack that segfaults on overflow.
- HTTP/1.1 only, with a small API: no router, no middleware layer, no HTTP/2.
- Small ecosystem compared with Tokio, hyper and axum.

## Problems with the evidence

- TechEmpower implementations are written and tuned by each framework's community (custom allocators, target-cpu=native, LTO, thread-per-core setups), so they show a ceiling, not what a default application gets.
- The plaintext test uses HTTP pipelining, which almost no real client does. Servers that batch pipelined responses into one write post numbers several times higher than they reach on ordinary traffic. Fortunes is the most realistic test, and the database driver weighs on it as much as the framework does.
- Requests per second are derived from totalRequests divided by the 15-second duration in TechEmpower's raw results file. The interactive results page draws the same values with JavaScript.
- Round 23 (run 2025-01-30, published 2025-03-17) is the most recent official round, and the TechEmpower FrameworkBenchmarks repository is now archived (last push 2026-03-24), so no newer TechEmpower data is coming.
- Round 23's axum entry already runs one current-thread Tokio runtime per core with SO_REUSEPORT, which is not how axum ships by default.
- may_minihttp's own README benchmark compares it only with tokio_minihttp on one laptop thread (116181.73 vs 100650.94 req/s).
- The last crates.io release (0.1.11) is from 2024-09-22, although the repository is active.

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> "may-minihttp": [ { "latencyAvg": "5.12ms", "latencyStdev": "10.49ms", "latencyMax": "42.40ms", "totalRequests": 274817312, "startTime": 1738534022, "endTime": 1738534037 }, { "latencyAvg": "10.87ms", "latencyStdev": "13.33ms", "latencyMax": "46.10ms", "totalRequests": 390724528, "startTime": 1738534039, "endTime": 1738534055 }, { "latencyAvg": "8.17ms", "latencyStdev": "12.25ms", "latencyMax": "2…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "188.83us", "latencyStdev": "134.69us", "latencyMax": "19.43ms", "totalRequests": 170630176, "startTime": 1738792996, "endTime": 1738793011 }, { "latencyAvg": "756.90us", "latencyStdev": "487.97us", "latencyMax": "26.12ms", "totalRequests": 185884960
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "may-minihttp": [ { "latencyAvg": "574.96us", "latencyStdev": "153.35us", "latencyMax": "4.64ms", "totalRequests": 415049, "startTime": 1738533523, "endTime": 1738533538 }, { "latencyAvg": "518.32us", "latencyStdev": "183.27us", "latencyMax": "4.42ms", "totalRequests": 919385, "startTime": 1738533540, "endTime": 1738533555 }, { "latencyAvg": "49.61us", "latencyStdev": "36.96us", "latencyMax": "4.1…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "571.66us", "latencyStdev": "177.18us", "latencyMax": "1.15ms", "totalRequests": 417065, "startTime": 1738792863, "endTime": 1738792878 }, { "latencyAvg": "522.36us", "latencyStdev": "200.74us", "latencyMax": "1.29ms", "totalRequests": 911833, "startTime": 1738792880, "endTime": 1738792895 }, { "latencyAvg": "53.42us", "latencyStdev": "19.10us", "latencyMax": "1.61ms", "t…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "may-minihttp": [ { "latencyAvg": "1.31ms", "latencyStdev": "175.05us", "latencyMax": "5.05ms", "totalRequests": 181813, "startTime": 1738533889, "endTime": 1738533904 }, { "latencyAvg": "1.32ms", "latencyStdev": "214.64us", "latencyMax": "4.82ms", "totalRequests": 362477, "startTime": 1738533906, "endTime": 1738533921 }, { "latencyAvg": "1.16ms", "latencyStdev": "258.70us", "latencyMax": "5.38ms"…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum-pg": [ { "latencyAvg": "1.33ms", "latencyStdev": "180.99us", "latencyMax": "2.07ms", "totalRequests": 179234, "startTime": 1738790959, "endTime": 1738790974 }, { "latencyAvg": "1.31ms", "latencyStdev": "233.59us", "latencyMax": "2.35ms", "totalRequests": 365430, "startTime": 1738790976, "endTime": 1738790991 }, { "latencyAvg": "1.14ms", "latencyStdev": "293.02us", "latencyMax": "2.68ms", "to…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> RUN RUSTFLAGS="-C target-cpu=native" cargo build --release
>
> — [github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d…](https://github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d775a749ad060753e26e3a/frameworks/Rust/may-minihttp/may-minihttp.dockerfile)

> One of the fastest web frameworks available according to the
>
> — [github.com/Xudong-Huang/may_minihttp](https://github.com/Xudong-Huang/may_minihttp)

> It's considered **unsafe** with the following pattern:
>
> — [github.com/Xudong-Huang/may](https://github.com/Xudong-Huang/may)

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
