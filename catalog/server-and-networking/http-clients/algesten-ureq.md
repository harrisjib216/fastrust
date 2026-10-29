# ureq (blocking HTTP client)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [HTTP clients](README.md)</sub>

**Tier B** (independent measurement) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Blocking HTTP client with a small dependency tree; no measured speed advantage over reqwest for sequential requests.

| | |
|---|---|
| **Beats** | reqwest 0.11.18 on a current-thread Tokio runtime, 1,000 sequential keep-alive GETs to a local server |
| **Claimed result** | hyperfine mean for 1,000 requests: rust-ureq 108.4 ± 2.3 ms, rust-reqwest 108.7 ± 2.2 ms, rust-hyper 107.6 ± 1.3 ms; rust-hyper 1.01 ± 0.02 times faster than rust-ureq and than rust-reqwest |
| **Workload** | One process sends 1,000 sequential HTTP/1.1 GET requests with keep-alive and TCP_NODELAY to a server on 127.0.0.1 and prints each body; hyperfine, 10 warmup runs and 100 measured runs, process start-up included. |
| **Metric** | time |
| **Hardware** | GitHub Actions ubuntu-latest runner (CPU not stated) |
| **OS** | Ubuntu (GitHub Actions ubuntu-latest) |
| **Toolchain** | cargo build --release with opt-level 3; rustc version not stated. ureq 2.7.1, reqwest 0.11.18, hyper 0.14.27. |
| **Measured** | 2023 |
| **Adoption** | 2.2k stars |
| **Last commit** | 2026-10-27 |
| **Platforms** | Any platform with std networking; rustls by default, native-tls optional |
| **Install** | cargo add ureq |
| **Links** | [repository](https://github.com/algesten/ureq) · [documentation](https://docs.rs/ureq) · [benchmark](https://github.com/orhun/zig-http-benchmarks) |

## Verdict

ureq does not claim to be faster than reqwest, and the one public head-to-head shows no difference: 108.4 ms vs 108.7 ms for 1,000 requests, inside each other's error bars. Choose ureq for its small blocking API and dependency tree, not for speed.

**Why tier B:** Third-party benchmark (written to compare a Zig HTTP client) with public code and hyperfine output, run on a GitHub Actions ubuntu-latest runner; no CPU model or rustc version stated.

## Use it when

- A CLI tool, build script or small service makes a few HTTP calls and you want no async runtime and fewer dependencies.

## Skip it when

- You expect it to make requests faster; the measured difference from reqwest is zero within error.
- You need many concurrent requests or HTTP/2 multiplexing; ureq is blocking and HTTP/1.1 only.

## How it is fast

ureq does blocking I/O on the calling thread, so there is no async runtime to start and no task scheduling per request. For sequential requests to one host the time is dominated by the network round trip and the server, which is why ureq, reqwest and raw hyper finish within about 1% of each other.

## What it costs

- Blocking only: each in-flight request occupies a thread.
- No HTTP/2.

## Problems with the evidence

- Microbenchmark against localhost: 1,000 sequential requests with tiny bodies, so it measures per-request client overhead, not throughput under concurrency.
- Versions are from 2023 (ureq 2.7.1, reqwest 0.11.18); ureq 3.x and reqwest 0.13 have not been benchmarked this way.
- Process start-up is part of each measured run.

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> | `rust-ureq`           |  108.4 ± 2.3 |    105.7 |    123.1 |  1.01 ± 0.02 |
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/README.md)

> | `rust-reqwest`        |  108.7 ± 2.2 |    105.4 |    123.7 |  1.01 ± 0.02 |
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/README.md)

> | `rust-hyper`          |  107.6 ± 1.3 |    104.4 |    114.8 |         1.00 |
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/README.md)

> rust-hyper ran 1.01 ± 0.02 times faster than rust-ureq 1.01 ± 0.02 times faster than rust-reqwest
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/README.md)

> The results are coming from a GitHub runner (`ubuntu-latest`)
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/README.md)

> for i in 1..=1000 {
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/rust-ureq/src/main.rs)

> ureq = { version = "2.7.1", default-features = false }
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/rust-ureq/Cargo.toml)

> reqwest = { version = "0.11.18", default-features = false }
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/rust-reqwest/Cargo.toml)

> "--warmup" "10" "--runs" "100"
>
> — [github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd7…](https://github.com/orhun/zig-http-benchmarks/blob/65d56a5a2b611c07854467fd747162fa0310e142/bench.sh)

</details>
