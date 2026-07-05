# Introduction
A load tester written in Rust.

## Table of Contents
- Installation
- Usage


## Getting Started

Long Parameters
```bash
cargo run -- --endpoint <endpoint> --virtual_users <virtual_users> --duration_s <duration_s> --method <method>
```

Short Parameters
```bash
cargo run -- -e <endpoint> -v <virtual_users> -d <duration_s> -m <method>
```

| Flag | Controls |
|---|---|
| `--virtual-users` | How many requests are in flight concurrently, sustained for the whole test |
| `--duration-s` | How long that concurrent load is sustained |

### Examples

**Spike test**
Burst of concurrent traffic, short duration:
```bash
cargo run -- -v 500 -d 5
```
500 concurrent requests in flight, sustained for 5 seconds.

**Sustained load test**
Moderate concurrency, long duration:
```bash
cargo run -- -v 20 -d 1800
```
20 concurrent requests in flight, sustained for 30 minutes — useful for catching
slow leaks or degradation that only shows up under prolonged load.

> **Note:** `total requests = virtual_users × (duration / avg_response_time)` <br>
> Faster server responses mean more total requests for the same VU count and duration,
> since each virtual user completes more loop iterations in the same time window.