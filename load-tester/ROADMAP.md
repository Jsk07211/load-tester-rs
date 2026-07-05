# Roadmap
Items listed according to planned implementation schedule.

## Basic Functionality
- [x] Parse arguments
- [x] Create test server
- [x] Figure out how to send GET request
- [x] Concurrency model — handled by tokio's runtime (defaults to # of CPUs)
- [x] Spawn concurrent virtual user tasks
- [x] Aggressively spam test server

## Logging
- [x] Add statistics
  - [x] p90 / p95 / p99 latency
  - [x] Average requests per second
  - [x] Total requests sent
  - [x] Total successful requests
  - [x] Percentage of successful requests

## HTTP Methods Support
- [x] Create POST endpoint to test
- [ ] Test POST endpoint
- [ ] Accept custom payloads
- [ ] Generate dummy payloads according to custom shape
- [ ] Accept custom headers
- [ ] Allow ingestion from config file instead of just CLI flags

# Robustness
- [ ] Handle request errors without crashing (currently discarded)
- [ ] Configurable per-request timeout (guard against hung requests)
  - [ ] Gentle shutdown when exceeding timeout after test duration ends
- [ ] Graceful handling of unreachable / invalid endpoint
- [ ] Hard concurrency cap (`max_in_flight`) - rejecting-and-counting
  - [ ] Add saturated/rejected counter

# Spawning Behaviour
- [x] Closed model: Specifying # of virtual users
- [ ] Open model: Specifying RPS
    - [ ] Determine ticker rate based on desired RPS (`interval_ms = 1000 / target_rps`)

| | Closed Model (VUs) | Open Model (Rate) |
|---|---|---|
| **You configure** | Number of virtual users | Requests per second (rate) |
| **Answers the question** | "If I have N concurrent users behaving like this, what performance do they experience?" | "If traffic arrives at rate R no matter what, does my server keep up, and how does it fail if not?" |
| **Best for** | Simulating realistic user sessions (browsing, waiting, clicking) where behavior genuinely depends on response time (e.g., "user won't click 'next' until page loads") | Capacity testing, finding breaking points, and not lying to yourself about degradation |
| **Throughput (RPS)** | Emergent output — depends on response time | Direct input — you set it |
| **Concurrency (in-flight requests)** | Direct input — capped at N | Emergent output — can pile up |
| **Behavior under server slowdown** | Offered load silently drops (coordinated omission) | Offered load stays constant; pileup/errors surface the real degradation |

# Ramping Behaviour
- [ ] Rate-based (increase load by X users/requests-per-second every T seconds)
- [ ] Support ramp-up/hold/ramp-down pattern (T_up, T_hold, T_down), sum to get total wall clock duration
    - [ ] For open model, recalculate with stepped ramp (`step_duration = T_up / N; R_i = target_rps * (i / N)`)
      - [ ] User configures an increment Δ (req/s to add per step) and a step_interval (seconds per step) `N = target_rps / Δ; step_duration = step_interval` (TODO: Handle remainder if not clean division)
    - [ ] Add phase-tagging (ramp-up, hold, ramp-down)

# API Assertions/Failure Handling
- [ ] HTTP-level failures (non-2xx status)
- [ ] Transport-level failures (connection refused, timeout, DNS failure)
- [ ] Assertion failures (wrong body content, or over latency threshold, even if status was 200)

```bash
assert:
  status: 200
  body_contains: "\"status\":\"ok\""   # optional
  max_latency_ms: 500                   # optional
```

# Export
- [ ] JSON/CSV results

Sources:

1. https://docs.rs/tokio-task-pool/latest/tokio_task_pool/
2. https://medium.com/@huseynzade.dadas/the-rust-compilation-process-with-rustc-3a622dd9e7ce
3. https://github.com/grafana/k6
4. https://medium.com/@kc_clintone/the-ultimate-guide-to-writing-a-great-readme-md-for-your-project-3d49c2023357
5. https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
6. https://tokio.rs/tokio/tutorial/
7. https://stackoverflow.com/questions/75836002/what-is-the-benefit-of-using-tokio-instead-of-os-threads-in-rust
8. https://stackoverflow.com/questions/8137391/percentile-calculation
9. https://www.geeksforgeeks.org/software-testing/software-testing-load-testing/