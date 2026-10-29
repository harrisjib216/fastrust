# ntex (web framework, actix-web fork)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Web frameworks](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> actix-style web framework that beats default-config axum 1.3-2.2x on HttpArena but is within 2-9% of thread-per-core axum on TechEmpower.

| | |
|---|---|
| **Beats** | axum on hyper and Tokio: TechEmpower Round 23 entry (axum 0.7.9, one current-thread runtime per core) and HttpArena 2026 standard-mode entry (axum 0.8, default multi-threaded runtime) |
| **Claimed result** | TechEmpower Round 23 peak total requests per 15 s, ntex vs axum: plaintext 190597520 vs 185884960, JSON 41510329 vs 40646919, fortunes (ntex-db vs axum-pg) 18155230 vs 16713989 (1.03x, 1.02x, 1.09x) (derived from page data). HttpArena, req/s ntex vs axum: baseline-4096 2168712 vs 1080638, json-tls-4096 1254763 vs 967831, json-comp-4096 589934 vs 268142, limited-conn-4096 1449176 vs 685038 (2.01x, 1.30x, 2.20x, 2.12x) (derived from page data) |
| **Workload** | TechEmpower Framework Benchmarks. plaintext: "Hello, World!" with HTTP/1.1 pipelining at 256 to 16,384 connections; JSON: one small object serialized per request at 16 to 512 connections; fortunes: fetch 12 rows from Postgres, add one, sort, render an escaped HTML template. Peak requests per second across the concurrency levels, 15-second runs. HttpArena: baseline is mixed GET/POST with query parsing at 4,096 connections; json-tls is JSON over HTTP/1.1 with TLS; json-comp adds gzip or brotli; limited-conn closes each connection after 10 requests. |
| **Metric** | throughput |
| **Hardware** | TechEmpower Round 23: ProLiant DL360 Gen10 Plus servers, Intel Xeon Gold 6330 @ 2.00GHz (56 cores), 64 GB, 40 Gbps Ethernet (separate server, database and load-generator machines). HttpArena: one AMD Threadripper PRO 3995WX workstation (64 cores) running server and load generator with pinned cores. |
| **OS** | Linux, in Docker |
| **Toolchain** | TechEmpower ntex: rust:latest image at the time, RUSTFLAGS="-C target-cpu=native", features tokio, mimalloc. HttpArena: each entry's own Dockerfile. |
| **Measured** | 2026-09 |
| **Adoption** | 2.5k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Linux, macOS and Windows (CI on all three); Tokio or compio runtimes |
| **Install** | cargo add ntex |
| **Links** | [repository](https://github.com/ntex-rs/ntex) · [documentation](https://docs.rs/ntex) · [benchmark](https://www.http-arena.com/) |

## Verdict

Against axum in its default multi-threaded setup (HttpArena) ntex is 1.3x to 2.2x faster. Against a thread-per-core axum (TechEmpower Round 23) the lead is 2-3% on plaintext and JSON and 9% on fortunes, so most of the gap is runtime configuration, not framework overhead.

**Why tier A:** Two public benchmarks with published implementation code and stated hardware: TechEmpower Round 23 and HttpArena.

## Use it when

- You serve many small requests per second and would otherwise run axum with Tokio's default multi-threaded scheduler.
- You want MQTT, AMQP or gRPC servers from the same project family.

## Skip it when

- You can run axum thread-per-core; TechEmpower shows that closes most of the gap.
- Your handlers are dominated by database or network waits.
- You rely on tower middleware or hyper-based crates.

## How it is fast

ntex runs one single-threaded event loop per core and keeps each connection on its worker, avoiding work-stealing and cross-thread wakeups. It uses its own buffer types (ntex-bytes) and a codec-based HTTP/1 implementation.

## What it costs

- Thread-per-core scheduling can leave cores idle when load is uneven across connections.
- Not compatible with tower or hyper middleware; major versions move quickly (both benchmarks used ntex 2, the current release is 4.4).
- MSRV 1.97 for the latest release.

## Problems with the evidence

- TechEmpower implementations are written and tuned by each framework's community (custom allocators, target-cpu=native, LTO, thread-per-core setups), so they show a ceiling, not what a default application gets.
- The plaintext test uses HTTP pipelining, which almost no real client does. Servers that batch pipelined responses into one write post numbers several times higher than they reach on ordinary traffic. Fortunes is the most realistic test, and the database driver weighs on it as much as the framework does.
- Requests per second are derived from totalRequests divided by the 15-second duration in TechEmpower's raw results file. The interactive results page draws the same values with JavaScript.
- Round 23 (run 2025-01-30, published 2025-03-17) is the most recent official round, and the TechEmpower FrameworkBenchmarks repository is now archived (last push 2026-03-24), so no newer TechEmpower data is coming.
- HttpArena runs the server and the load generator on the same 64-core workstation with pinned cores, so results reflect loopback networking.
- HttpArena results for each framework are committed separately (axum.json last updated 2026-09-11), not in one synchronized run.
- ntex's README makes no numeric performance claim; the reputation comes from benchmark rankings.

## Evidence

<details>
<summary>25 quotes from the sources, checked 2026-10-28</summary>

> "ntex": [ { "latencyAvg": "173.95us", "latencyStdev": "145.70us", "latencyMax": "19.01ms", "totalRequests": 177265040, "startTime": 1738495183, "endTime": 1738495198 }, { "latencyAvg": "709.34us", "latencyStdev": "474.85us", "latencyMax": "26.63ms", "totalRequests": 190597520
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "188.83us", "latencyStdev": "134.69us", "latencyMax": "19.43ms", "totalRequests": 170630176, "startTime": 1738792996, "endTime": 1738793011 }, { "latencyAvg": "756.90us", "latencyStdev": "487.97us", "latencyMax": "26.12ms", "totalRequests": 185884960
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "ntex": [ { "latencyAvg": "601.24us", "latencyStdev": "178.31us", "latencyMax": "4.51ms", "totalRequests": 397147, "startTime": 1738495049, "endTime": 1738495064 }, { "latencyAvg": "519.67us", "latencyStdev": "200.19us", "latencyMax": "4.32ms", "totalRequests": 916842, "startTime": 1738495066, "endTime": 1738495082 }, { "latencyAvg": "54.00us", "latencyStdev": "39.57us", "latencyMax": "8.08ms", "t…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum": [ { "latencyAvg": "571.66us", "latencyStdev": "177.18us", "latencyMax": "1.15ms", "totalRequests": 417065, "startTime": 1738792863, "endTime": 1738792878 }, { "latencyAvg": "522.36us", "latencyStdev": "200.74us", "latencyMax": "1.29ms", "totalRequests": 911833, "startTime": 1738792880, "endTime": 1738792895 }, { "latencyAvg": "53.42us", "latencyStdev": "19.10us", "latencyMax": "1.61ms", "t…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "ntex-db": [ { "latencyAvg": "1.38ms", "latencyStdev": "190.03us", "latencyMax": "4.63ms", "totalRequests": 172999, "startTime": 1738494405, "endTime": 1738494420 }, { "latencyAvg": "1.33ms", "latencyStdev": "212.09us", "latencyMax": "4.94ms", "totalRequests": 358755, "startTime": 1738494422, "endTime": 1738494437 }, { "latencyAvg": "1.19ms", "latencyStdev": "259.01us", "latencyMax": "5.32ms", "to…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "axum-pg": [ { "latencyAvg": "1.33ms", "latencyStdev": "180.99us", "latencyMax": "2.07ms", "totalRequests": 179234, "startTime": 1738790959, "endTime": 1738790974 }, { "latencyAvg": "1.31ms", "latencyStdev": "233.59us", "latencyMax": "2.35ms", "totalRequests": 365430, "startTime": 1738790976, "endTime": 1738790991 }, { "latencyAvg": "1.14ms", "latencyStdev": "293.02us", "latencyMax": "2.68ms", "to…
>
> — [techempower.com/benchmarks/results/round23/ph.json](https://www.techempower.com/benchmarks/results/round23/ph.json)

> "baseline-4096": { "framework": "ntex", "language": "Rust", "rps": 2168712,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/ntex.json)

> "baseline-4096": { "framework": "axum", "language": "Rust", "rps": 1080638,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "json-tls-4096": { "framework": "ntex", "language": "Rust", "rps": 1254763,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/ntex.json)

> "json-tls-4096": { "framework": "axum", "language": "Rust", "rps": 967831,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "json-comp-4096": { "framework": "ntex", "language": "Rust", "rps": 589934,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/ntex.json)

> "json-comp-4096": { "framework": "axum", "language": "Rust", "rps": 268142,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> "limited-conn-4096": { "framework": "ntex", "language": "Rust", "rps": 1449176,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/ntex.json)

> "limited-conn-4096": { "framework": "axum", "language": "Rust", "rps": 685038,
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/data/results/axum.json)

> Axum 0.8 on hyper with the multi-threaded Tokio runtime, default configuration.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/frameworks/axum/meta.json)

> ntex 2 on its own tokio-backed runtime, default configuration.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/frameworks/ntex/meta.json)

> The reference hardware for HttpArena results is a single-socket AMD Threadripper PRO 3995WX workstation.
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/site/content/docs/hardware/_index.md)

> Mixed GET/POST with query parsing (512/4K conns)
>
> — [github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6f…](https://github.com/MDA2AV/HttpArena/blob/d2df064173444ef3ca9e234bddf2263b6fc6f938/README.md)

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
