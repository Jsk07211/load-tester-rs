use std::{
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

#[derive(Default)]
pub struct RunMetrics {
    pub test_duration: Duration,
    pub success_count: AtomicU64,
    pub error_count: AtomicU64,
    // std::sync::Mutex blocks the OS thread while waiting for the lock;
    // tokio::sync::Mutex yields the async task while waiting,
    // letting other tasks run on that thread in the meantime.
    pub latencies: Mutex<Vec<Duration>>,
}

pub struct SummaryStatistics {
    pub test_duration: f64,
    pub total_requests: u64,
    pub success: u64,
    pub errors: u64,
    pub success_rate: f64,
    pub avg_rps: f64,
    pub avg_latency: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
}

impl SummaryStatistics {
    pub fn report(&self) -> String {
        let mut out = String::new();

        out.push_str("\n===== Load Test Summary =====\n");
        out.push_str(&format!("Duration:           {:.2}s\n", self.test_duration));
        out.push_str(&format!("Total requests:     {}\n", self.total_requests));
        out.push_str(&format!("Successful:         {}\n", self.success));
        out.push_str(&format!("Failed:             {}\n", self.errors));
        out.push_str(&format!("Success rate:       {:.2}%\n", self.success_rate));
        out.push_str(&format!("Avg requests/sec:   {:.2}\n", self.avg_rps));
        out.push_str("-----------------------------\n");
        out.push_str(&format!("Avg latency:        {:.2?}\n", self.avg_latency));
        out.push_str(&format!("p90 latency:        {:.2?}\n", self.p90));
        out.push_str(&format!("p95 latency:        {:.2?}\n", self.p95));
        out.push_str(&format!("p99 latency:        {:.2?}\n", self.p99));
        out.push_str("=============================\n");

        out
    }
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

pub fn get_summary(metrics: &RunMetrics, test_duration: Duration) -> SummaryStatistics {
    let success = metrics.success_count.load(Ordering::Relaxed);
    let errors = metrics.error_count.load(Ordering::Relaxed);
    let total = success + errors;
    let success_rate = (success / total) as f64 * 100.0;
    let avg_rps = total as f64 / test_duration.as_secs_f64();

    let mut latencies = metrics.latencies.lock().unwrap().clone();
    latencies.sort_unstable();

    let avg_latency = if latencies.is_empty() {
        Duration::ZERO
    } else {
        latencies.iter().sum::<Duration>() / total as u32
    };

    let p90 = get_percentile(&latencies, 90.0);
    let p95 = get_percentile(&latencies, 95.0);
    let p99 = get_percentile(&latencies, 99.0);

    SummaryStatistics {
        test_duration: test_duration.as_secs_f64(),
        total_requests: success + errors,
        success,
        errors,
        success_rate,
        avg_rps,
        avg_latency,
        p90,
        p95,
        p99,
    }
}
