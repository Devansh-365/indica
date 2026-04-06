/// Relative Strength Index using Wilder's smoothing.
/// Default period is typically 14.
/// Returns `None` if insufficient data (need at least `period + 1` values).
pub fn rsi(closes: &[f64], period: usize) -> Option<f64> {
    if closes.len() < period + 1 || period == 0 {
        return None;
    }

    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;

    // Initial average gain/loss from first `period` changes
    for i in 1..=period {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss += change.abs();
        }
    }
    avg_gain /= period as f64;
    avg_loss /= period as f64;

    // Wilder's smoothing for remaining values
    for i in (period + 1)..closes.len() {
        let change = closes[i] - closes[i - 1];
        let gain = if change > 0.0 { change } else { 0.0 };
        let loss = if change < 0.0 { change.abs() } else { 0.0 };
        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;
    }

    if avg_loss == 0.0 {
        return Some(100.0);
    }

    let rs = avg_gain / avg_loss;
    Some(100.0 - 100.0 / (1.0 + rs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsi_all_gains() {
        let closes: Vec<f64> = (0..20).map(|i| 100.0 + i as f64).collect();
        assert_eq!(rsi(&closes, 14), Some(100.0));
    }

    #[test]
    fn rsi_all_losses() {
        let closes: Vec<f64> = (0..20).map(|i| 100.0 - i as f64).collect();
        let result = rsi(&closes, 14).unwrap();
        assert!(result < 1.0); // Near 0
    }

    #[test]
    fn rsi_midrange() {
        // Alternating up/down should be near 50
        let closes = vec![
            100.0, 102.0, 100.0, 102.0, 100.0, 102.0, 100.0, 102.0,
            100.0, 102.0, 100.0, 102.0, 100.0, 102.0, 100.0, 102.0,
        ];
        let result = rsi(&closes, 14).unwrap();
        assert!((result - 50.0).abs() < 5.0);
    }

    #[test]
    fn rsi_insufficient_data() {
        assert_eq!(rsi(&[100.0, 101.0], 14), None);
        assert_eq!(rsi(&[], 14), None);
    }
}
