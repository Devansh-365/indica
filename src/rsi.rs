use crate::utils::wilders_smooth;

/// Relative Strength Index using Wilder's smoothing.
/// Default period is typically 14.
/// Returns `None` if insufficient data (need at least `period + 1` values).
#[must_use]
pub fn rsi(closes: &[f64], period: usize) -> Option<f64> {
    if closes.len() < period + 1 || period == 0 {
        return None;
    }

    // Compute gains and losses
    let changes: Vec<(f64, f64)> = closes
        .windows(2)
        .map(|w| {
            let change = w[1] - w[0];
            if change > 0.0 {
                (change, 0.0)
            } else {
                (0.0, change.abs())
            }
        })
        .collect();

    // Initial average gain/loss from first `period` changes
    let avg_gain: f64 = changes[..period].iter().map(|(g, _)| g).sum::<f64>() / period as f64;
    let avg_loss: f64 = changes[..period].iter().map(|(_, l)| l).sum::<f64>() / period as f64;

    // Wilder's smoothing for remaining changes
    let gains: Vec<f64> = changes[period..].iter().map(|(g, _)| *g).collect();
    let losses: Vec<f64> = changes[period..].iter().map(|(_, l)| *l).collect();

    let final_avg_gain = wilders_smooth(avg_gain, &gains, period);
    let final_avg_loss = wilders_smooth(avg_loss, &losses, period);

    if final_avg_loss < f64::EPSILON {
        return Some(100.0);
    }

    let rs = final_avg_gain / final_avg_loss;
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
        assert!(result < 1.0);
    }

    #[test]
    fn rsi_midrange() {
        let closes = vec![
            100.0, 102.0, 100.0, 102.0, 100.0, 102.0, 100.0, 102.0, 100.0, 102.0, 100.0, 102.0,
            100.0, 102.0, 100.0, 102.0,
        ];
        let result = rsi(&closes, 14).unwrap();
        assert!((result - 50.0).abs() < 5.0);
    }

    #[test]
    fn rsi_insufficient_data() {
        assert_eq!(rsi(&[100.0, 101.0], 14), None);
        assert_eq!(rsi(&[], 14), None);
    }

    #[test]
    fn rsi_zero_period() {
        assert_eq!(rsi(&[100.0; 20], 0), None);
    }
}
