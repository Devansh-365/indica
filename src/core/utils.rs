/// Round to N decimal places.
pub fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// Wilder's smoothing step: (prev * (period-1) + val) / period.
/// Used by RSI and ATR.
pub fn wilders_step(prev: f64, val: f64, period: usize) -> f64 {
    (prev * (period as f64 - 1.0) + val) / period as f64
}

/// EMA smoothing step: val * k + prev * (1 - k), where k = 2/(period+1).
pub fn ema_step(prev: f64, val: f64, k: f64) -> f64 {
    val * k + prev * (1.0 - k)
}

/// Compute EMA smoothing factor for a given period.
pub fn ema_k(period: usize) -> f64 {
    2.0 / (period as f64 + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(1.23456, 2), 1.23);
        assert_eq!(round(1.235, 2), 1.24);
    }

    #[test]
    fn test_wilders_step() {
        assert_eq!(wilders_step(10.0, 12.0, 2), 11.0);
    }

    #[test]
    fn test_ema_step() {
        let k = ema_k(3); // 0.5
        assert_eq!(ema_step(10.0, 12.0, k), 11.0);
    }
}
