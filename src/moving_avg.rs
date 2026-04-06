/// Simple Moving Average of the last `period` values.
/// Returns `None` if there are fewer values than `period`.
pub fn sma(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period || period == 0 {
        return None;
    }
    let slice = &values[values.len() - period..];
    let sum: f64 = slice.iter().sum();
    Some(sum / period as f64)
}

/// Exponential Moving Average.
/// Seeded with SMA of the first `period` values, then smoothed forward.
/// Returns `None` if there are fewer values than `period`.
pub fn ema(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period || period == 0 {
        return None;
    }
    let k = 2.0 / (period as f64 + 1.0);
    let seed: f64 = values[..period].iter().sum::<f64>() / period as f64;
    let result = values[period..]
        .iter()
        .fold(seed, |prev, &val| val * k + prev * (1.0 - k));
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_basic() {
        assert_eq!(sma(&[1.0, 2.0, 3.0, 4.0, 5.0], 3), Some(4.0));
        assert_eq!(sma(&[10.0, 20.0, 30.0], 3), Some(20.0));
    }

    #[test]
    fn sma_insufficient_data() {
        assert_eq!(sma(&[1.0, 2.0], 5), None);
        assert_eq!(sma(&[], 1), None);
    }

    #[test]
    fn sma_period_one() {
        assert_eq!(sma(&[42.0, 99.0], 1), Some(99.0));
    }

    #[test]
    fn ema_basic() {
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let result = ema(&data, 3).unwrap();
        // EMA(3) of [10,11,12,13,14,15]:
        // Seed = avg(10,11,12) = 11.0, k = 0.5
        // 13*0.5 + 11*0.5 = 12.0
        // 14*0.5 + 12*0.5 = 13.0
        // 15*0.5 + 13*0.5 = 14.0
        assert!((result - 14.0).abs() < 0.01);
    }

    #[test]
    fn ema_insufficient_data() {
        assert_eq!(ema(&[1.0], 5), None);
    }

    #[test]
    fn ema_equals_sma_at_period_length() {
        let data = vec![10.0, 20.0, 30.0];
        // With exactly `period` values, EMA seed = SMA and no further smoothing
        assert_eq!(ema(&data, 3), sma(&data, 3));
    }
}
