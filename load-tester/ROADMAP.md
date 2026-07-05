# Roadmap

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

## POST Support
- [ ] Create POST endpoint to test
- [ ] Test POST endpoint
- [ ] Accept custom payloads

# Robustness
- [ ] Handle request errors without crashing (currently discarded)
- [ ] Configurable per-request timeout (guard against hung requests)
- [ ] Graceful handling of unreachable / invalid endpoint

Sources:

https://docs.rs/tokio-task-pool/latest/tokio_task_pool/
https://medium.com/@huseynzade.dadas/the-rust-compilation-process-with-rustc-3a622dd9e7ce
https://github.com/grafana/k6
https://medium.com/@kc_clintone/the-ultimate-guide-to-writing-a-great-readme-md-for-your-project-3d49c2023357
https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
https://tokio.rs/tokio/tutorial/
https://stackoverflow.com/questions/75836002/what-is-the-benefit-of-using-tokio-instead-of-os-threads-in-rust
https://stackoverflow.com/questions/8137391/percentile-calculation