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

pub fn get_percentile(sorted: Vec<Duration>, percentile: f32) -> Duration {
    if percentile > 100f32 || sorted.is_empty() {
        Duration::ZERO
    } else {
        // Linear interpolation for calculating percentile (opposed to nearest rank)
        // https://github.com/grafana/k6/blob/1636475f14d71fd0f6671b21a98376eed2adc566/metrics/sink.go#L145-L166
        if sorted.len() == 1 {
            sorted[0]
        } else {
            let i = (percentile / 100.0) * sorted.len() as f32;
            let lower = sorted[i.floor() as usize];
            let range = sorted[i.ceil() as usize] - lower;
            let fractional = i - i.floor(); // find distance of i away from its floor; blend upper and lower proportionally

            lower + range.mul_f32(fractional)
        }
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

    println!("\n===== Load Test Summary =====");
    println!("Duration:            {:.2}s", test_duration.as_secs_f64());
    println!("Total requests:      {total}");
    println!("Successful:          {success}");
    println!("Failed:              {errors}");
    println!("Success rate:        {success_rate:.2}%");
    println!("Avg requests/sec:    {avg_rps:.2}");
    // println!("------------------------------");
    println!("Avg latency:         {:.2?}", avg_latency);
    // println!("p90 latency:         {:.2?}", p90);
    // println!("p95 latency:         {:.2?}", p95);
    // println!("p99 latency:         {:.2?}", p99);
    // println!("==============================\n");
}
