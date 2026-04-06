use crate::core::utils::{round, wilders_step};

/// Average True Range using Wilder's smoothing.
#[must_use]
pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64> {
    let len = closes.len();
    if len < period + 1 || highs.len() < len || lows.len() < len || period == 0 {
        return None;
    }
    let true_ranges: Vec<f64> = (1..len)
        .map(|i| {
            (highs[i] - lows[i])
                .max((highs[i] - closes[i - 1]).abs())
                .max((lows[i] - closes[i - 1]).abs())
        })
        .collect();
    if true_ranges.len() < period {
        return None;
    }
    let seed: f64 = true_ranges[..period].iter().sum::<f64>() / period as f64;
    let mut value = seed;
    for &tr in &true_ranges[period..] {
        value = wilders_step(value, tr, period);
    }
    Some(round(value, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atr_basic() {
        let closes: Vec<f64> = (0..20).map(|i| 100.0 + (i as f64 * 0.5)).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 2.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 2.0).collect();
        let result = atr(&highs, &lows, &closes, 14).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn atr_flat() {
        let data = vec![100.0; 20];
        assert_eq!(atr(&data, &data, &data, 14), Some(0.0));
    }

    #[test]
    fn atr_insufficient() {
        assert!(atr(&[1.0; 5], &[1.0; 5], &[1.0; 5], 14).is_none());
    }
}
