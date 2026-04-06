use crate::core::traits::Indicator;
use crate::core::types::Candle;
use crate::core::utils::{round, wilders_step};

/// Streaming ADX (Average Directional Index).
/// Measures trend strength regardless of direction.
pub struct Adx {
    period: usize,
    count: usize,
    prev_high: f64,
    prev_low: f64,
    prev_close: f64,
    plus_dm_sum: f64,
    minus_dm_sum: f64,
    tr_sum: f64,
    smooth_plus_dm: f64,
    smooth_minus_dm: f64,
    smooth_tr: f64,
    dx_sum: f64,
    dx_count: usize,
    adx: f64,
}

impl Adx {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            count: 0,
            prev_high: 0.0,
            prev_low: 0.0,
            prev_close: 0.0,
            plus_dm_sum: 0.0,
            minus_dm_sum: 0.0,
            tr_sum: 0.0,
            smooth_plus_dm: 0.0,
            smooth_minus_dm: 0.0,
            smooth_tr: 0.0,
            dx_sum: 0.0,
            dx_count: 0,
            adx: 0.0,
        }
    }
}

impl Indicator for Adx {
    type Output = f64;

    fn update(&mut self, candle: &Candle) -> Option<f64> {
        self.count += 1;

        if self.count == 1 {
            self.prev_high = candle.high;
            self.prev_low = candle.low;
            self.prev_close = candle.close;
            return None;
        }

        let plus_dm = (candle.high - self.prev_high).max(0.0);
        let minus_dm = (self.prev_low - candle.low).max(0.0);
        let (plus_dm, minus_dm) = if plus_dm > minus_dm {
            (plus_dm, 0.0)
        } else {
            (0.0, minus_dm)
        };

        let tr = (candle.high - candle.low)
            .max((candle.high - self.prev_close).abs())
            .max((candle.low - self.prev_close).abs());

        self.prev_high = candle.high;
        self.prev_low = candle.low;
        self.prev_close = candle.close;

        let idx = self.count - 1; // 1-based data index

        if idx <= self.period {
            self.plus_dm_sum += plus_dm;
            self.minus_dm_sum += minus_dm;
            self.tr_sum += tr;

            if idx == self.period {
                self.smooth_plus_dm = self.plus_dm_sum;
                self.smooth_minus_dm = self.minus_dm_sum;
                self.smooth_tr = self.tr_sum;
            } else {
                return None;
            }
        } else {
            self.smooth_plus_dm = wilders_step(self.smooth_plus_dm, plus_dm, self.period);
            self.smooth_minus_dm = wilders_step(self.smooth_minus_dm, minus_dm, self.period);
            self.smooth_tr = wilders_step(self.smooth_tr, tr, self.period);
        }

        if self.smooth_tr < f64::EPSILON {
            return None;
        }

        let plus_di = 100.0 * self.smooth_plus_dm / self.smooth_tr;
        let minus_di = 100.0 * self.smooth_minus_dm / self.smooth_tr;
        let di_sum = plus_di + minus_di;

        if di_sum < f64::EPSILON {
            return None;
        }

        let dx = 100.0 * (plus_di - minus_di).abs() / di_sum;

        self.dx_count += 1;
        if self.dx_count < self.period {
            self.dx_sum += dx;
            None
        } else if self.dx_count == self.period {
            self.dx_sum += dx;
            self.adx = self.dx_sum / self.period as f64;
            Some(round(self.adx, 2))
        } else {
            self.adx = wilders_step(self.adx, dx, self.period);
            Some(round(self.adx, 2))
        }
    }

    fn reset(&mut self) {
        *self = Self::new(self.period);
    }
}

/// Convenience function: ADX from slices.
#[must_use]
pub fn adx(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64> {
    let len = closes.len();
    if len < 2 * period + 1 || highs.len() < len || lows.len() < len || period == 0 {
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
    let mut ind = Adx::new(period);
    ind.compute_last(&candles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adx_strong_trend() {
        let closes: Vec<f64> = (0..50).map(|i| 100.0 + i as f64 * 2.0).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 1.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 1.0).collect();
        let result = adx(&highs, &lows, &closes, 14).unwrap();
        assert!(
            result > 20.0,
            "ADX should be high in strong trend: {}",
            result
        );
    }

    #[test]
    fn adx_insufficient() {
        assert!(adx(&[1.0; 10], &[1.0; 10], &[1.0; 10], 14).is_none());
    }
}
