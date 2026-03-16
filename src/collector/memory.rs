pub struct Memory {
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub ram_used_percent: f64,
}

impl Memory {
    pub fn from_bytes(used: u64, total: u64) -> Self {
        let ram_used_gb = used as f64 / 1_073_741_824.0;
        let ram_total_gb = total as f64 / 1_073_741_824.0;
        let ram_used_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Self {
            ram_used_gb,
            ram_total_gb,
            ram_used_percent,
        }
    }
}
