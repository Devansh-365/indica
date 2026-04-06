use crate::core::traits::Indicator;
use crate::core::types::Candle;
use crate::core::utils::wilders_step;

/// Streaming RSI (Wilder's smoothing).
pub struct Rsi {
    period: usize,
    count: usize,
    prev_close: f64,
    gain_sum: f64,
    loss_sum: f64,
    avg_gain: f64,
    avg_loss: f64,
}

impl Rsi {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            count: 0,
            prev_close: 0.0,
            gain_sum: 0.0,
            loss_sum: 0.0,
            avg_gain: 0.0,
            avg_loss: 0.0,
        }
    }
}

impl Indicator for Rsi {
    type Output = f64;

    fn update(&mut self, candle: &Candle) -> Option<f64> {
        self.count += 1;

        if self.count == 1 {
            self.prev_close = candle.close;
            return None;
        }

        let change = candle.close - self.prev_close;
        let gain = change.max(0.0);
        let loss = (-change).max(0.0);
        self.prev_close = candle.close;

        let idx = self.count - 1;

        if idx <= self.period {
            self.gain_sum += gain;
            self.loss_sum += loss;
            if idx == self.period {
                self.avg_gain = self.gain_sum / self.period as f64;
                self.avg_loss = self.loss_sum / self.period as f64;
            } else {
                return None;
            }
        } else {
            self.avg_gain = wilders_step(self.avg_gain, gain, self.period);
            self.avg_loss = wilders_step(self.avg_loss, loss, self.period);
        }

        if self.avg_loss < f64::EPSILON {
            return Some(100.0);
        }

        let rs = self.avg_gain / self.avg_loss;
        Some(100.0 - 100.0 / (1.0 + rs))
    }

    fn reset(&mut self) {
        *self = Self::new(self.period);
    }
}

/// Convenience function: RSI from close values.
#[must_use]
pub fn rsi(closes: &[f64], period: usize) -> Option<f64> {
    if closes.len() < period + 1 || period == 0 {
        return None;
    }
    let candles: Vec<Candle> = closes.iter().map(|&c| Candle::from_close(c)).collect();
    let mut ind = Rsi::new(period);
    ind.compute_last(&candles)
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
    fn rsi_insufficient() {
        assert_eq!(rsi(&[100.0, 101.0], 14), None);
    }

    #[test]
    fn rsi_streaming() {
        let closes: Vec<f64> = (0..20).map(|i| 100.0 + i as f64).collect();
        let fn_result = rsi(&closes, 14).unwrap();

        let mut ind = Rsi::new(14);
        let mut last = None;
        for &c in &closes {
            last = ind.update(&Candle::from_close(c));
        }
        assert!((last.unwrap() - fn_result).abs() < 0.01);
    }
}
