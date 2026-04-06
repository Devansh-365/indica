use crate::core::traits::Indicator;
use crate::core::types::Candle;
use crate::core::utils::{round, wilders_step};

/// Supertrend result.
#[derive(Debug, Clone, Copy)]
pub struct SupertrendResult {
    pub value: f64,
    pub direction: SupertrendDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupertrendDirection {
    Up,
    Down,
}

impl std::fmt::Display for SupertrendDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}

/// Streaming Supertrend indicator.
pub struct Supertrend {
    atr_period: usize,
    multiplier: f64,
    count: usize,
    // ATR state
    prev_close: f64,
    tr_sum: f64,
    atr: f64,
    // Supertrend state
    prev_upper: f64,
    prev_lower: f64,
    prev_st: f64,
    prev_direction: SupertrendDirection,
}

impl Supertrend {
    pub fn new(atr_period: usize, multiplier: f64) -> Self {
        Self {
            atr_period,
            multiplier,
            count: 0,
            prev_close: 0.0,
            tr_sum: 0.0,
            atr: 0.0,
            prev_upper: 0.0,
            prev_lower: 0.0,
            prev_st: 0.0,
            prev_direction: SupertrendDirection::Up,
        }
    }
}

impl Indicator for Supertrend {
    type Output = SupertrendResult;

    fn update(&mut self, candle: &Candle) -> Option<SupertrendResult> {
        self.count += 1;

        if self.count == 1 {
            self.prev_close = candle.close;
            return None;
        }

        // True Range
        let tr = (candle.high - candle.low)
            .max((candle.high - self.prev_close).abs())
            .max((candle.low - self.prev_close).abs());

        if self.count <= self.atr_period + 1 {
            self.tr_sum += tr;
            if self.count == self.atr_period + 1 {
                self.atr = self.tr_sum / self.atr_period as f64;
            } else {
                self.prev_close = candle.close;
                return None;
            }
        } else {
            self.atr = wilders_step(self.atr, tr, self.atr_period);
        }

        let hl2 = (candle.high + candle.low) / 2.0;
        let basic_upper = hl2 + self.multiplier * self.atr;
        let basic_lower = hl2 - self.multiplier * self.atr;

        let upper = if basic_upper < self.prev_upper || self.prev_close > self.prev_upper {
            basic_upper
        } else {
            self.prev_upper
        };

        let lower = if basic_lower > self.prev_lower || self.prev_close < self.prev_lower {
            basic_lower
        } else {
            self.prev_lower
        };

        let (st, direction) = if self.prev_st == self.prev_upper {
            if candle.close <= upper {
                (upper, SupertrendDirection::Down)
            } else {
                (lower, SupertrendDirection::Up)
            }
        } else if candle.close >= lower {
            (lower, SupertrendDirection::Up)
        } else {
            (upper, SupertrendDirection::Down)
        };

        self.prev_upper = upper;
        self.prev_lower = lower;
        self.prev_st = st;
        self.prev_direction = direction;
        self.prev_close = candle.close;

        Some(SupertrendResult {
            value: round(st, 2),
            direction,
        })
    }

    fn reset(&mut self) {
        self.count = 0;
        self.prev_close = 0.0;
        self.tr_sum = 0.0;
        self.atr = 0.0;
        self.prev_upper = 0.0;
        self.prev_lower = 0.0;
        self.prev_st = 0.0;
        self.prev_direction = SupertrendDirection::Up;
    }
}

/// Convenience function: Supertrend from slices.
#[must_use]
pub fn supertrend(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    atr_period: usize,
    multiplier: f64,
) -> Option<SupertrendResult> {
    let len = closes.len();
    if len < atr_period + 1 || highs.len() < len || lows.len() < len || atr_period == 0 {
        return None;
    }
    let candles: Vec<Candle> = (0..len)
        .map(|i| Candle {
            open: closes[i],
            high: highs[i],
            low: lows[i],
            close: closes[i],
            volume: 0.0,
        })
        .collect();
    let mut ind = Supertrend::new(atr_period, multiplier);
    ind.compute_last(&candles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supertrend_uptrend() {
        let closes: Vec<f64> = (0..30).map(|i| 100.0 + i as f64).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 1.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 1.0).collect();
        let result = supertrend(&highs, &lows, &closes, 10, 3.0).unwrap();
        assert_eq!(result.direction, SupertrendDirection::Up);
        assert!(result.value < *closes.last().unwrap());
    }

    #[test]
    fn supertrend_downtrend() {
        let closes: Vec<f64> = (0..30).map(|i| 200.0 - i as f64).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 1.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 1.0).collect();
        let result = supertrend(&highs, &lows, &closes, 10, 3.0).unwrap();
        assert_eq!(result.direction, SupertrendDirection::Down);
    }

    #[test]
    fn supertrend_insufficient() {
        assert!(supertrend(&[1.0; 5], &[1.0; 5], &[1.0; 5], 10, 3.0).is_none());
    }
}
