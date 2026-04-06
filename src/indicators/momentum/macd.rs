use crate::core::utils::round;
use crate::indicators::trend::ema::ema_series;

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
            Self::Bullish => write!(f, "bullish"),
            Self::Bearish => write!(f, "bearish"),
            Self::Neutral => write!(f, "none"),
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

    let ema_fast = ema_series(closes, fast_period)?;
    let ema_slow = ema_series(closes, slow_period)?;

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

    let signal_series = ema_series(&macd_line, signal_period)?;
    let current_signal = *signal_series.last()?;
    let current_macd = *macd_line.last()?;
    let histogram = current_macd - current_signal;

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

    #[test]
    fn macd_uptrend() {
        let closes: Vec<f64> = (0..50).map(|i| 100.0 + i as f64 * 0.5).collect();
        let result = macd(&closes, 12, 26, 9).unwrap();
        assert!(result.value > 0.0);
    }

    #[test]
    fn macd_downtrend() {
        let closes: Vec<f64> = (0..50).map(|i| 150.0 - i as f64 * 0.5).collect();
        let result = macd(&closes, 12, 26, 9).unwrap();
        assert!(result.value < 0.0);
    }

    #[test]
    fn macd_invalid_params() {
        let data = vec![1.0; 50];
        assert!(macd(&data, 0, 26, 9).is_none());
        assert!(macd(&data, 26, 12, 9).is_none());
    }
}
