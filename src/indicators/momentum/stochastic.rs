use crate::core::utils::round;
use std::collections::VecDeque;

/// Stochastic Oscillator result.
#[derive(Debug, Clone, Copy)]
pub struct StochasticResult {
    pub k: f64, // Fast line (0-100)
    pub d: f64, // Slow line (SMA of K)
}

/// Stochastic Oscillator (%K, %D).
/// %K = (close - lowest_low) / (highest_high - lowest_low) * 100
/// %D = SMA of %K over d_period
#[must_use]
pub fn stochastic(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    k_period: usize,
    d_period: usize,
) -> Option<StochasticResult> {
    let len = closes.len();
    if len < k_period + d_period - 1
        || highs.len() < len
        || lows.len() < len
        || k_period == 0
        || d_period == 0
    {
        return None;
    }

    // Compute %K values
    let mut k_values = VecDeque::with_capacity(d_period);

    for i in (k_period - 1)..len {
        let window_highs = &highs[i + 1 - k_period..=i];
        let window_lows = &lows[i + 1 - k_period..=i];

        let highest = window_highs
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let lowest = window_lows.iter().cloned().fold(f64::INFINITY, f64::min);

        let range = highest - lowest;
        let k = if range < f64::EPSILON {
            50.0
        } else {
            (closes[i] - lowest) / range * 100.0
        };

        k_values.push_back(k);
        if k_values.len() > d_period {
            k_values.pop_front();
        }
    }

    if k_values.len() < d_period {
        return None;
    }

    let k = *k_values.back()?;
    let d = k_values.iter().sum::<f64>() / d_period as f64;

    Some(StochasticResult {
        k: round(k, 2),
        d: round(d, 2),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stochastic_basic() {
        let highs = vec![10.0, 12.0, 14.0, 13.0, 15.0, 14.0, 16.0, 15.0, 17.0, 16.0];
        let lows = vec![8.0, 9.0, 11.0, 10.0, 12.0, 11.0, 13.0, 12.0, 14.0, 13.0];
        let closes = vec![9.0, 11.0, 13.0, 12.0, 14.0, 13.0, 15.0, 14.0, 16.0, 15.0];
        let result = stochastic(&highs, &lows, &closes, 5, 3).unwrap();
        assert!(result.k >= 0.0 && result.k <= 100.0);
        assert!(result.d >= 0.0 && result.d <= 100.0);
    }

    #[test]
    fn stochastic_overbought() {
        // Price at highs = %K near 100
        let highs: Vec<f64> = (0..20).map(|i| 100.0 + i as f64).collect();
        let lows: Vec<f64> = highs.iter().map(|h| h - 5.0).collect();
        let closes = highs.clone(); // Close at high
        let result = stochastic(&highs, &lows, &closes, 14, 3).unwrap();
        assert!(result.k > 80.0);
    }

    #[test]
    fn stochastic_insufficient() {
        assert!(stochastic(&[1.0; 5], &[1.0; 5], &[1.0; 5], 14, 3).is_none());
    }
}
