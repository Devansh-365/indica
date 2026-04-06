use crate::moving_avg::ema_series;
use crate::utils::round;

/// MACD crossover direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Crossover {
    Bullish,
    Bearish,
    Neutral,
}

impl std::fmt::Display for Crossover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Crossover::Bullish => write!(f, "bullish"),
            Crossover::Bearish => write!(f, "bearish"),
            Crossover::Neutral => write!(f, "none"),
        }
    }
}

/// MACD computation result.
#[derive(Debug, Clone)]
pub struct MacdResult {
    pub value: f64,
    pub signal: f64,
    pub histogram: f64,
    pub crossover: Crossover,
}

/// MACD (Moving Average Convergence Divergence).
/// Default parameters: fast=12, slow=26, signal=9.
/// Returns `None` if insufficient data or invalid parameters.
#[must_use]
pub fn macd(
    closes: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Option<MacdResult> {
    if closes.len() < slow_period + signal_period
        || fast_period == 0
        || slow_period == 0
        || signal_period == 0
        || fast_period >= slow_period
    {
        return None;
    }

    // Get full EMA series for fast and slow
    let ema_fast = ema_series(closes, fast_period)?;
    let ema_slow = ema_series(closes, slow_period)?;

    // MACD line = fast EMA - slow EMA (aligned from slow_period onward)
    // ema_fast starts at index fast_period, ema_slow starts at index slow_period
    // We need to align them: skip the first (slow_period - fast_period) entries of ema_fast
    let offset = slow_period - fast_period;
    if ema_fast.len() <= offset {
        return None;
    }

    let macd_line: Vec<f64> = ema_fast[offset..]
        .iter()
        .zip(ema_slow.iter())
        .map(|(f, s)| f - s)
        .collect();

    if macd_line.len() < signal_period {
        return None;
    }

    // Signal line = EMA of MACD line
    let signal_series = ema_series(&macd_line, signal_period)?;
    let current_signal = *signal_series.last()?;
    let current_macd = *macd_line.last()?;
    let histogram = current_macd - current_signal;

    // Crossover detection: compare current and previous histogram
    let prev_signal = if signal_series.len() >= 2 {
        signal_series[signal_series.len() - 2]
    } else {
        current_signal
    };
    let prev_macd = if macd_line.len() >= 2 {
        macd_line[macd_line.len() - 2]
    } else {
        current_macd
    };
    let prev_histogram = prev_macd - prev_signal;

    let crossover = if prev_histogram <= 0.0 && histogram > 0.0 {
        Crossover::Bullish
    } else if prev_histogram >= 0.0 && histogram < 0.0 {
        Crossover::Bearish
    } else {
        Crossover::Neutral
    };

    Some(MacdResult {
        value: round(current_macd, 2),
        signal: round(current_signal, 2),
        histogram: round(histogram, 2),
        crossover,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trending_up() -> Vec<f64> {
        (0..50).map(|i| 100.0 + i as f64 * 0.5).collect()
    }

    fn trending_down() -> Vec<f64> {
        (0..50).map(|i| 150.0 - i as f64 * 0.5).collect()
    }

    #[test]
    fn macd_trending_up() {
        let result = macd(&trending_up(), 12, 26, 9).unwrap();
        assert!(result.value > 0.0, "MACD should be positive in uptrend");
    }

    #[test]
    fn macd_trending_down() {
        let result = macd(&trending_down(), 12, 26, 9).unwrap();
        assert!(result.value < 0.0, "MACD should be negative in downtrend");
    }

    #[test]
    fn macd_insufficient_data() {
        assert!(macd(&[1.0; 20], 12, 26, 9).is_none());
    }

    #[test]
    fn macd_invalid_params() {
        let data = vec![1.0; 50];
        assert!(macd(&data, 0, 26, 9).is_none());
        assert!(macd(&data, 12, 0, 9).is_none());
        assert!(macd(&data, 12, 26, 0).is_none());
        assert!(macd(&data, 26, 12, 9).is_none()); // fast >= slow
    }

    #[test]
    fn macd_crossover_detection() {
        let mut data: Vec<f64> = vec![100.0; 35];
        for i in 0..20 {
            data.push(100.0 + i as f64 * 2.0);
        }
        let result = macd(&data, 12, 26, 9);
        assert!(result.is_some());
    }
}
