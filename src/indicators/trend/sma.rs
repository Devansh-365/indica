use crate::core::traits::Indicator;
use crate::core::types::Candle;
use std::collections::VecDeque;

/// Streaming Simple Moving Average.
pub struct Sma {
    period: usize,
    window: VecDeque<f64>,
}

impl Sma {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
        }
    }
}

impl Indicator for Sma {
    type Output = f64;

    fn update(&mut self, candle: &Candle) -> Option<f64> {
        self.window.push_back(candle.close);
        if self.window.len() > self.period {
            self.window.pop_front();
        }
        if self.window.len() == self.period {
            Some(self.window.iter().sum::<f64>() / self.period as f64)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.window.clear();
    }
}

/// Convenience function: SMA of the last `period` close values.
#[must_use]
pub fn sma(closes: &[f64], period: usize) -> Option<f64> {
    if closes.len() < period || period == 0 {
        return None;
    }
    let slice = &closes[closes.len() - period..];
    Some(slice.iter().sum::<f64>() / period as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_fn_basic() {
        assert_eq!(sma(&[1.0, 2.0, 3.0, 4.0, 5.0], 3), Some(4.0));
        assert_eq!(sma(&[10.0, 20.0, 30.0], 3), Some(20.0));
    }

    #[test]
    fn sma_fn_insufficient() {
        assert_eq!(sma(&[1.0, 2.0], 5), None);
        assert_eq!(sma(&[], 1), None);
    }

    #[test]
    fn sma_streaming() {
        let mut ind = Sma::new(3);
        assert_eq!(ind.update(&Candle::from_close(1.0)), None);
        assert_eq!(ind.update(&Candle::from_close(2.0)), None);
        assert_eq!(ind.update(&Candle::from_close(3.0)), Some(2.0));
        assert_eq!(ind.update(&Candle::from_close(4.0)), Some(3.0));
        assert_eq!(ind.update(&Candle::from_close(5.0)), Some(4.0));
    }
}
