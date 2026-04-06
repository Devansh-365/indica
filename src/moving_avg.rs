/// Simple Moving Average of the last `period` values.
/// Returns `None` if there are fewer values than `period`.
#[must_use]
pub fn sma(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period || period == 0 {
        return None;
    }
    let slice = &values[values.len() - period..];
    let sum: f64 = slice.iter().sum();
    Some(sum / period as f64)
}

/// Exponential Moving Average (final value only).
/// Seeded with SMA of the first `period` values, then smoothed forward.
/// Returns `None` if there are fewer values than `period`.
#[must_use]
pub fn ema(values: &[f64], period: usize) -> Option<f64> {
    ema_series(values, period).map(|s| *s.last().unwrap())
}

/// Compute the full EMA series starting from the seed point.
/// Returns a Vec where [0] is the seed (SMA of first `period` values)
/// and each subsequent value is smoothed forward.
/// Used internally by MACD to access intermediate EMA values.
pub(crate) fn ema_series(values: &[f64], period: usize) -> Option<Vec<f64>> {
    if values.len() < period || period == 0 {
        return None;
    }
    let k = 2.0 / (period as f64 + 1.0);
    let seed: f64 = values[..period].iter().sum::<f64>() / period as f64;
    let mut result = Vec::with_capacity(values.len() - period + 1);
    result.push(seed);
    let mut prev = seed;
    for &val in &values[period..] {
        prev = val * k + prev * (1.0 - k);
        result.push(prev);
    }
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
        assert!((result - 14.0).abs() < 0.01);
    }

    #[test]
    fn ema_series_length() {
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let series = ema_series(&data, 3).unwrap();
        // 6 values, period 3: seed + 3 smoothed = 4 values
        assert_eq!(series.len(), 4);
        assert!((series[0] - 11.0).abs() < 0.01); // seed = avg(10,11,12)
    }

    #[test]
    fn ema_insufficient_data() {
        assert_eq!(ema(&[1.0], 5), None);
    }

    #[test]
    fn ema_equals_sma_at_period_length() {
        let data = vec![10.0, 20.0, 30.0];
        assert_eq!(ema(&data, 3), sma(&data, 3));
    }
}
