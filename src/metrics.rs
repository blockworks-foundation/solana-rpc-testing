use derive_more::{Add, Sum};
use std::time::Duration;

use serde::Serialize;

#[derive(Default, Serialize, Add, Sum)]
pub struct Metric {
    pub requests_per_second: f64,
    pub time_per_request: f64,
    pub total_transferred: u64,
    #[serde(flatten)]
    pub partial_metric: PartialMetric,
}

#[derive(Default, Serialize, Add, Sum)]
pub struct PartialMetric {
    pub total_time: Duration,
    pub passed: u64,
    pub failed: u64,
}

impl std::ops::Div<u64> for Metric {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self {
            requests_per_second: self.requests_per_second / rhs as f64,
            time_per_request: self.time_per_request / rhs as f64,
            total_transferred: self.total_transferred / rhs,
            partial_metric: self.partial_metric / rhs,
        }
    }
}

impl std::ops::Div<u64> for PartialMetric {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self {
            total_time: Duration::from_secs_f64(self.total_time.as_secs_f64() / rhs as f64),
            passed: self.passed / rhs,
            failed: self.failed / rhs,
        }
    }
}

impl From<PartialMetric> for Metric {
    fn from(partial_metric: PartialMetric) -> Self {
        let total_transferred = partial_metric.passed + partial_metric.failed;
        let total_time_secs = partial_metric.total_time.as_secs_f64();

        Metric {
            requests_per_second: total_transferred as f64 / total_time_secs,
            time_per_request: total_time_secs / total_transferred as f64,
            total_transferred,
            partial_metric,
        }
    }
}
