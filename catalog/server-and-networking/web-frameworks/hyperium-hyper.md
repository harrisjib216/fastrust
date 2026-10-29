# hyper (raw server, without axum)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Web frameworks](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Serving straight from hyper beats axum 1.4-3.5x on trivial handlers, but the gap is 6% on JSON and reverses on TechEmpower fortunes.

| | |
|---|---|
| **Beats** | axum on the same hyper and Tokio stack (TechEmpower Round 23: axum 0.7.9; HttpArena 2026: axum 0.8 standard mode) |
| **Claimed result** | TechEmpower Round 23 peak total requests per 15 s, hyper vs axum: plaintext 262027104 vs 185884960, JSON 43284157 vs 40646919, fortunes (hyper-db vs axum-pg) 15999674 vs 16713989 (1.41x, 1.06x, 0.96x) (derived from page data). HttpArena, req/s hyper vs axum: baseline-4096 3003360 vs 1080638, pipelined-4096 16218012 vs 6474608, limited-conn-4096 2362875 vs 685038 (2.78x, 2.50x, 3.45x) (derived from page data) |
| **Workload** | TechEmpower Framework Benchmarks. plaintext: "Hello, World!" with HTTP/1.1 pipelining at 256 to 16,384 connections; JSON: one small object serialized per request at 16 to 512 connections; fortunes: fetch 12 rows from Postgres, add one, sort, render an escaped HTML template. Peak requests per second across the concurrency levels, 15-second runs. HttpArena: mixed GET/POST with query parsing, 16x pipelined requests, and connections closed after 10 requests, at 4,096 connections. |
| **Metric** | throughput |
| **Hardware** | TechEmpower Round 23: ProLiant DL360 Gen10 Plus servers, Intel Xeon Gold 6330 @ 2.00GHz (56 cores), 64 GB, 40 Gbps Ethernet (separate server, database and load-generator machines). HttpArena: one AMD Threadripper PRO 3995WX workstation (64 cores). |
| **OS** | Linux, in Docker |
| **Toolchain** | TechEmpower hyper entry: hyper 0.14 on rust:1.67, target-cpu=native, lto, codegen-units 1. axum entry: axum 0.7.9 (hyper 1.x) on rust:1.83. HttpArena: each entry's own Dockerfile. |
| **Measured** | 2026-09 |
| **Adoption** | 16.4k stars |
| **Last commit** | 2026-10-13 |
| **Platforms** | Any platform Tokio supports |
| **Install** | cargo add hyper hyper-util |
| **Links** | [repository](https://github.com/hyperium/hyper) · [documentation](https://docs.rs/hyper) · [benchmark](https://www.http-arena.com/) |

## Verdict

Dropping axum for raw hyper pays off for trivial handlers: 1.41x on TechEmpower plaintext and 2.5-3.5x on HttpArena. On TechEmpower JSON the gap is 6%, and on fortunes axum was 4% faster, so the cost of axum's routing and extractors disappears once a handler does real work.

**Why tier A:** Two public benchmarks with published implementation code and stated hardware: TechEmpower Round 23 and HttpArena.

## Use it when

- A proxy, health-check or static endpoint does almost no work per request and you need every request per second.
- You only have a handful of routes and are willing to parse paths and bodies yourself.

## Skip it when

- Handlers touch a database, serialize real payloads or call other services; TechEmpower shows axum level or ahead there.
- You need routing, extractors or middleware; rebuilding them by hand gives back the gain.

## How it is fast

A raw hyper service skips axum's router, extractors, response conversion and the tower Service layers. That overhead is a fixed cost per request, so it matters only when the handler itself does almost nothing.

## What it costs

- Routing, body limits, extractors, error handling and middleware all become your code.
- hyper 1.x moved the server and client conveniences into hyper-util, so a raw server needs more glue than in 0.14.

## Problems with the evidence

- TechEmpower implementations are written and tuned by each framework's community (custom allocators, target-cpu=native, LTO, thread-per-core setups), so they show a ceiling, not what a default application gets.
- The plaintext test uses HTTP pipelining, which almost no real client does. Servers that batch pipelined responses into one write post numbers several times higher than they reach on ordinary traffic. Fortunes is the most realistic test, and the database driver weighs on it as much as the framework does.
- Requests per second are derived from totalRequests divided by the 15-second duration in TechEmpower's raw results file. The interactive results page draws the same values with JavaScript.
- Round 23 (run 2025-01-30, published 2025-03-17) is the most recent official round, and the TechEmpower FrameworkBenchmarks repository is now archived (last push 2026-03-24), so no newer TechEmpower data is coming.
- The TechEmpower hyper entry still uses hyper 0.14, while the axum entry runs on hyper 1.x, so Round 23 compares different hyper versions.
- HttpArena classifies hyper as an engine entry and axum as a standard-mode framework entry; the axum entry deserializes query strings and bodies through typed extractors.
- axum's README says its performance is comparable to hyper; HttpArena's numbers contradict that for trivial handlers, TechEmpower JSON and fortunes support it.

## Evidence

<details>
<summary>24 quotes from the sources, checked 2026-10-28</summary>

> "hyper": [ { "latencyAvg": "149.33us", "latencyStdev": "92.13us", "latencyMax": "6.37ms", "totalRequests": 216276064, "startTime": 1738604432, "endTime": 1738604448 }, { "latencyAvg": "541.04us", "latencyStdev": "390.09us", "latencyMax": "24.95ms", "totalRequests": 262027104
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "188.83us", "latencyStdev": "134.69us", "latencyMax": "19.43ms", "totalRequests": 170630176, "startTime": 1738792996, "endTime": 1738793011 }, { "latencyAvg": "756.90us", "latencyStdev": "487.97us", "latencyMax": "26.12ms", "totalRequests": 185884960
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "hyper": [ { "latencyAvg": "575.18us", "latencyStdev": "157.71us", "latencyMax": "1.15ms", "totalRequests": 414538, "startTime": 1738604299, "endTime": 1738604314 }, { "latencyAvg": "489.54us", "latencyStdev": "201.18us", "latencyMax": "1.26ms", "totalRequests": 972629, "startTime": 1738604316, "endTime": 1738604331 }, { "latencyAvg": "51.43us", "latencyStdev": "18.63us", "latencyMax": "2.01ms", "…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "571.66us", "latencyStdev": "177.18us", "latencyMax": "1.15ms", "totalRequests": 417065, "startTime": 1738792863, "endTime": 1738792878 }, { "latencyAvg": "522.36us", "latencyStdev": "200.74us", "latencyMax": "1.29ms", "totalRequests": 911833, "startTime": 1738792880, "endTime": 1738792895 }, { "latencyAvg": "53.42us", "latencyStdev": "19.10us", "latencyMax": "1.61ms", "t…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "hyper-db": [ { "latencyAvg": "1.34ms", "latencyStdev": "252.78us", "latencyMax": "14.54ms", "totalRequests": 178672, "startTime": 1738603913, "endTime": 1738603928 }, { "latencyAvg": "1.31ms", "latencyStdev": "298.95us", "latencyMax": "15.46ms", "totalRequests": 365656, "startTime": 1738603930, "endTime": 1738603945 }, { "latencyAvg": "1.25ms", "latencyStdev": "446.18us", "latencyMax": "28.33ms",…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum-pg": [ { "latencyAvg": "1.33ms", "latencyStdev": "180.99us", "latencyMax": "2.07ms", "totalRequests": 179234, "startTime": 1738790959, "endTime": 1738790974 }, { "latencyAvg": "1.31ms", "latencyStdev": "233.59us", "latencyMax": "2.35ms", "totalRequests": 365430, "startTime": 1738790976, "endTime": 1738790991 }, { "latencyAvg": "1.14ms", "latencyStdev": "293.02us", "latencyMax": "2.68ms", "to…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> hyper = { version = "0.14", features = ["server", "http1"], default-features = false }
>
> — [github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d…](https://github.com/TechEmpower/FrameworkBenchmarks/blob/523534bb61450e3522d775a749ad060753e26e3a/frameworks/Rust/hyper/Cargo.toml)

> "baseline-4096": { "framework": "hyper", "language": "Rust", "rps": 3003360,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/hyper.json)

> "baseline-4096": { "framework": "axum", "language": "Rust", "rps": 1080638,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "pipelined-4096": { "framework": "hyper", "language": "Rust", "rps": 16218012,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/hyper.json)

> "pipelined-4096": { "framework": "axum", "language": "Rust", "rps": 6474608,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "limited-conn-4096": { "framework": "hyper", "language": "Rust", "rps": 2362875,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/hyper.json)

> "limited-conn-4096": { "framework": "axum", "language": "Rust", "rps": 685038,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> Axum 0.8 on hyper with the multi-threaded Tokio runtime, default configuration.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/frameworks/axum/meta.json)

> The reference hardware for HttpArena results is a single-socket AMD Threadripper PRO 3995WX workstation.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/content/docs/hardware/_index.md)

> and adds very little overhead.
>
> — [github.com/tokio-rs/axum](https://github.com/tokio-rs/axum)

> Leading in performance
>
> — [github.com/hyperium/hyper](https://github.com/hyperium/hyper)

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
