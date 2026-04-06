/// Round to N decimal places.
pub fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// Wilder's smoothing: (prev * (period-1) + val) / period.
/// Used by RSI (for avg gain/loss) and ATR (for true range smoothing).
pub fn wilders_smooth(seed: f64, values: &[f64], period: usize) -> f64 {
    values.iter().fold(seed, |prev, &val| {
        (prev * (period as f64 - 1.0) + val) / period as f64
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(1.23456, 2), 1.23);
        assert_eq!(round(1.235, 2), 1.24);
        assert_eq!(round(100.0, 2), 100.0);
    }

    #[test]
    fn test_wilders_smooth() {
        // Starting from 10.0, smoothing [12.0] with period 2:
        // (10.0 * 1.0 + 12.0) / 2.0 = 11.0
        assert_eq!(wilders_smooth(10.0, &[12.0], 2), 11.0);
    }
}
