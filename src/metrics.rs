#[derive(Default)]
pub struct Metrics {
    pub requests_per_second: u32,
    pub time_per_request: u32,
    pub total_transferred: u32,
}
