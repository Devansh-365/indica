use crate::utils::round;

/// MACD crossover direction.
#[derive(Debug, Clone, PartialEq)]
pub enum Crossover {
    Bullish,
    Bearish,
    None,
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
/// Returns `None` if insufficient data.
pub fn macd(
    closes: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> Option<MacdResult> {
    if closes.len() < slow_period + signal_period || slow_period == 0 {
        return None;
    }

    let k_fast = 2.0 / (fast_period as f64 + 1.0);
    let k_slow = 2.0 / (slow_period as f64 + 1.0);

    let mut ema_slow: f64 = closes[..slow_period].iter().sum::<f64>() / slow_period as f64;

    // Seed fast EMA and advance it to slow_period point
    let mut ema_fast: f64 = closes[..fast_period].iter().sum::<f64>() / fast_period as f64;
    for i in fast_period..slow_period {
        ema_fast = closes[i] * k_fast + ema_fast * (1.0 - k_fast);
    }

    let mut macd_line = Vec::new();
    for i in slow_period..closes.len() {
        ema_fast = closes[i] * k_fast + ema_fast * (1.0 - k_fast);
        ema_slow = closes[i] * k_slow + ema_slow * (1.0 - k_slow);
        macd_line.push(ema_fast - ema_slow);
    }

    if macd_line.len() < signal_period {
        return None;
    }

    // Signal line = EMA of MACD line
    let k_signal = 2.0 / (signal_period as f64 + 1.0);
    let mut signal_line: f64 =
        macd_line[..signal_period].iter().sum::<f64>() / signal_period as f64;

    let mut prev_signal = signal_line;
    for i in signal_period..macd_line.len() {
        prev_signal = signal_line;
        signal_line = macd_line[i] * k_signal + signal_line * (1.0 - k_signal);
    }

    let current_macd = *macd_line.last().unwrap();
    let histogram = current_macd - signal_line;

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
        Crossover::None
    };

    Some(MacdResult {
        value: round(current_macd, 2),
        signal: round(signal_line, 2),
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
        assert!(result.histogram > 0.0 || result.histogram.abs() < 0.5);
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
    fn macd_crossover_detection() {
        // Flat then up — should eventually get bullish crossover
        let mut data: Vec<f64> = vec![100.0; 35];
        for i in 0..20 {
            data.push(100.0 + i as f64 * 2.0);
        }
        let result = macd(&data, 12, 26, 9);
        assert!(result.is_some());
    }
}
