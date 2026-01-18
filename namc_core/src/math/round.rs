pub const fn round_at(v: f64, p: usize) -> f64 {
    (v * 10_i64.pow(p as u32) as f64).round() / 10_i64.pow(p as u32) as f64
}