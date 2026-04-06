use crate::core::traits::Indicator;
use crate::core::types::Candle;
use crate::core::utils::{ema_k, ema_step};

/// Streaming Exponential Moving Average.
pub struct Ema {
    period: usize,
    k: f64,
    count: usize,
    sum: f64,
    value: f64,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            k: ema_k(period),
            count: 0,
            sum: 0.0,
            value: 0.0,
        }
    }
}

impl Indicator for Ema {
    type Output = f64;

    fn update(&mut self, candle: &Candle) -> Option<f64> {
        self.count += 1;
        if self.count < self.period {
            self.sum += candle.close;
            None
        } else if self.count == self.period {
            self.sum += candle.close;
            self.value = self.sum / self.period as f64; // Seed with SMA
            Some(self.value)
        } else {
            self.value = ema_step(self.value, candle.close, self.k);
            Some(self.value)
        }
    }

    fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
        self.value = 0.0;
    }
}

/// Convenience function: EMA of close values.
#[must_use]
pub fn ema(values: &[f64], period: usize) -> Option<f64> {
    if values.len() < period || period == 0 {
        return None;
    }
    let candles: Vec<Candle> = values.iter().map(|&c| Candle::from_close(c)).collect();
    let mut ind = Ema::new(period);
    ind.compute_last(&candles)
}

/// Full EMA series (used internally by MACD).
pub(crate) fn ema_series(values: &[f64], period: usize) -> Option<Vec<f64>> {
    if values.len() < period || period == 0 {
        return None;
    }
    let candles: Vec<Candle> = values.iter().map(|&c| Candle::from_close(c)).collect();
    let mut ind = Ema::new(period);
    let results: Vec<f64> = ind.compute(&candles).into_iter().flatten().collect();
    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ema_fn_basic() {
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let result = ema(&data, 3).unwrap();
        assert!((result - 14.0).abs() < 0.01);
    }

    #[test]
    fn ema_fn_insufficient() {
        assert_eq!(ema(&[1.0], 5), None);
    }

    #[test]
    fn ema_streaming() {
        let mut ind = Ema::new(3);
        assert_eq!(ind.update(&Candle::from_close(10.0)), None);
        assert_eq!(ind.update(&Candle::from_close(11.0)), None);
        let seed = ind.update(&Candle::from_close(12.0)).unwrap();
        assert!((seed - 11.0).abs() < 0.01); // SMA seed
        let next = ind.update(&Candle::from_close(13.0)).unwrap();
        assert!((next - 12.0).abs() < 0.01); // 13*0.5 + 11*0.5
    }

    #[test]
    fn ema_series_works() {
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let series = ema_series(&data, 3).unwrap();
        assert_eq!(series.len(), 4);
    }
}
