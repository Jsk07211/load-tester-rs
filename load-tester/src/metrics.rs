use std::{
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

#[derive(Default)]
pub struct RunStatistics {
    pub test_duration: Duration,
    pub success_count: AtomicU64,
    pub error_count: AtomicU64,
    // std::sync::Mutex blocks the OS thread while waiting for the lock;
    // tokio::sync::Mutex yields the async task while waiting,
    // letting other tasks run on that thread in the meantime.
    pub latencies: Mutex<Vec<Duration>>,
}

/// Calculates a percentile from a sorted slice of durations, using linear
/// interpolation (as opposed to nearest-rank), matching k6's approach:
/// https://github.com/grafana/k6/blob/1636475f14d71fd0f6671b21a98376eed2adc566/metrics/sink.go#L145-L166
pub fn get_percentile(sorted: &[Duration], percentile: f32) -> Duration {
    match sorted.len() {
        0 => Duration::ZERO,
        1 => sorted[0],
        len if percentile <= 100.0 => {
            let i = (percentile / 100.0) * (len as f32 - 1.0); // avoid OOB panic if p100
            let lower = sorted[i.floor() as usize];
            let upper = sorted[i.ceil() as usize];
            let range = upper - lower;
            let fractional = i - i.floor();

            // weights lower & upper according to i's distance from its lower bound
            lower + range.mul_f32(fractional)
        }
        _ => Duration::ZERO,
    }
}

pub fn print_summary(stats: &RunStatistics, test_duration: Duration) {
    let success = stats.success_count.load(Ordering::Relaxed);
    let errors = stats.error_count.load(Ordering::Relaxed);
    let total = success + errors;
    let success_rate = (success / total) * 100;
    let avg_rps = total as f64 / test_duration.as_secs_f64();

    let mut latencies = stats.latencies.lock().unwrap().clone();
    latencies.sort_unstable();

    let avg_latency = if latencies.is_empty() {
        Duration::ZERO
    } else {
        latencies.iter().sum::<Duration>() / total as u32
    };

    let p90 = get_percentile(&latencies, 90.0);
    let p95 = get_percentile(&latencies, 95.0);
    let p99 = get_percentile(&latencies, 99.0);

    println!("\n===== Load Test Summary =====");
    println!("Duration:            {:.2}s", test_duration.as_secs_f64());
    println!("Total requests:      {total}");
    println!("Successful:          {success}");
    println!("Failed:              {errors}");
    println!("Success rate:        {success_rate:.2}%");
    println!("Avg requests/sec:    {avg_rps:.2}");
    println!("------------------------------");
    println!("Avg latency:         {:.2?}", avg_latency);
    println!("p90 latency:         {:.2?}", p90);
    println!("p95 latency:         {:.2?}", p95);
    println!("p99 latency:         {:.2?}", p99);
    println!("==============================\n");
}
