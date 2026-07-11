use std::time::Duration;

#[derive(Default, Debug)]
pub struct RunMetrics {
    pub test_duration: Duration,
    pub success_count: u64,
    pub error_count: u64,
    pub latencies: Vec<Duration>,
}

#[derive(PartialEq, Debug)]
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

// TODO: This could be an impl of SummaryStatistics
pub fn get_summary(metrics: &RunMetrics) -> SummaryStatistics {
    let success = metrics.success_count;
    let errors = metrics.error_count;
    let total_requests = success + errors;
    let success_rate = match total_requests {
        0 => 0.0,
        _ => (success as f64 / total_requests as f64) * 100.0,
    };
    let avg_rps = total_requests as f64 / metrics.test_duration.as_secs_f64();

    let mut latencies = metrics.latencies.clone();
    latencies.sort_unstable();

    let avg_latency = if latencies.is_empty() {
        Duration::ZERO
    } else {
        latencies.iter().sum::<Duration>() / total_requests as u32
    };

    let p90 = get_percentile(&latencies, 90.0);
    let p95 = get_percentile(&latencies, 95.0);
    let p99 = get_percentile(&latencies, 99.0);

    SummaryStatistics {
        test_duration: metrics.test_duration.as_secs_f64(),
        total_requests,
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

#[cfg(test)]
mod tests {
    use super::*;

    impl RunMetrics {
        fn new(
            test_duration_s: f64,
            success_count: u64,
            error_count: u64,
            latencies: Vec<Duration>,
        ) -> Result<RunMetrics, Self> {
            Ok(RunMetrics {
                test_duration: Duration::from_secs_f64(test_duration_s),
                success_count,
                error_count,
                latencies,
            })
        }
    }

    mod get_percentile_tests {
        use super::*;

        #[test]
        fn get_percentile_max() {
            let durations = [
                Duration::from_secs_f64(10.0),
                Duration::from_secs_f64(20.0),
                Duration::from_secs_f64(30.0),
                Duration::from_secs_f64(40.0),
            ];

            let expected = Duration::from_secs_f64(40.0);
            let actual = get_percentile(&durations, 100.0);

            assert_eq!(expected, actual)
        }

        #[test]
        fn get_percentile_precision() {
            let durations = [
                Duration::from_secs_f64(10.0),
                Duration::from_secs_f64(20.0),
                Duration::from_secs_f64(30.0),
                Duration::from_secs_f64(40.12),
            ];

            let expected = Duration::from_secs_f64(40.12);
            let actual = get_percentile(&durations, 100.0);

            assert_eq!(expected, actual)
        }
    }

    mod get_summary_tests {
        use super::*;

        #[test]
        fn get_summary_precision() {
            let mut durations = vec![
                Duration::from_secs_f64(3.39),
                Duration::from_secs_f64(5.51),
                Duration::from_secs_f64(4.25),
                Duration::from_secs_f64(1.62),
                Duration::from_secs_f64(8.03),
                Duration::from_secs_f64(7.91),
                Duration::from_secs_f64(3.15),
                Duration::from_secs_f64(6.12),
                Duration::from_secs_f64(7.98),
                Duration::from_secs_f64(7.42),
                Duration::from_secs_f64(7.27),
                Duration::from_secs_f64(3.72),
                Duration::from_secs_f64(7.34),
                Duration::from_secs_f64(2.85),
                Duration::from_secs_f64(7.15),
                Duration::from_secs_f64(4.53),
                Duration::from_secs_f64(7.53),
                Duration::from_secs_f64(1.81),
                Duration::from_secs_f64(5.61),
                Duration::from_secs_f64(7.67),
                Duration::from_secs_f64(6.84),
                Duration::from_secs_f64(1.84),
                Duration::from_secs_f64(2.68),
                Duration::from_secs_f64(3.02),
                Duration::from_secs_f64(9.86),
                Duration::from_secs_f64(8.58),
                Duration::from_secs_f64(0.43),
                Duration::from_secs_f64(0.88),
                Duration::from_secs_f64(2.40),
                Duration::from_secs_f64(0.13),
                Duration::from_secs_f64(6.99),
                Duration::from_secs_f64(5.26),
                Duration::from_secs_f64(9.17),
                Duration::from_secs_f64(1.88),
                Duration::from_secs_f64(5.11),
                Duration::from_secs_f64(4.13),
                Duration::from_secs_f64(1.89),
                Duration::from_secs_f64(1.65),
                Duration::from_secs_f64(9.91),
                Duration::from_secs_f64(7.63),
            ];
            durations.sort();
            let metrics = RunMetrics::new(5.0, 20, 20, durations.clone()).unwrap();
            let expected = SummaryStatistics {
                test_duration: 5.0,
                total_requests: 40,
                success: 20,
                errors: 20,
                success_rate: 0.5,
                avg_rps: 8.0,
                avg_latency: Duration::from_secs_f64(5.0285),
                p90: get_percentile(&durations, 90.0),
                p95: get_percentile(&durations, 95.0),
                p99: get_percentile(&durations, 99.0),
            };
            let actual = get_summary(&metrics);
            assert_eq!(expected, actual)
        }
    }
}
