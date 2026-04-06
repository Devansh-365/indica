use crate::utils::{round, wilders_smooth};

/// Average True Range using Wilder's smoothing.
/// Requires `period + 1` data points minimum.
/// Returns `None` if insufficient data.
#[must_use]
pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64> {
    let len = closes.len();
    if len < period + 1 || highs.len() < len || lows.len() < len || period == 0 {
        return None;
    }

    // Compute true ranges
    let true_ranges: Vec<f64> = (1..len)
        .map(|i| {
            let hl = highs[i] - lows[i];
            let hc = (highs[i] - closes[i - 1]).abs();
            let lc = (lows[i] - closes[i - 1]).abs();
            hl.max(hc).max(lc)
        })
        .collect();

    if true_ranges.len() < period {
        return None;
    }

    // Initial ATR = simple average of first `period` true ranges
    let seed: f64 = true_ranges[..period].iter().sum::<f64>() / period as f64;

    // Wilder's smoothing for remaining true ranges
    let smoothed = wilders_smooth(seed, &true_ranges[period..], period);

    Some(round(smoothed, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atr_basic() {
        let highs = vec![
            48.7, 48.72, 48.9, 48.87, 48.82, 49.05, 49.2, 49.35, 49.92, 50.19, 50.12, 49.66, 49.88,
            50.19, 50.36, 50.57,
        ];
        let lows = vec![
            47.79, 48.14, 48.39, 48.37, 48.24, 48.64, 48.94, 48.86, 49.50, 49.87, 49.20, 48.90,
            49.43, 49.73, 49.26, 50.09,
        ];
        let closes = vec![
            48.16, 48.61, 48.75, 48.63, 48.74, 49.03, 49.07, 49.32, 49.91, 50.13, 49.53, 49.50,
            49.75, 50.03, 49.99, 50.23,
        ];
        let result = atr(&highs, &lows, &closes, 14).unwrap();
        assert!(result > 0.0);
        assert!(result < 2.0);
    }

    #[test]
    fn atr_insufficient_data() {
        assert!(atr(&[1.0; 5], &[1.0; 5], &[1.0; 5], 14).is_none());
    }

    #[test]
    fn atr_flat_market() {
        let data = vec![100.0; 20];
        let result = atr(&data, &data, &data, 14).unwrap();
        assert_eq!(result, 0.0);
    }
}
