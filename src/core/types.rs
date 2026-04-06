/// A single OHLCV candle.
#[derive(Debug, Clone, Copy)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Candle {
    /// Create a candle from just a close price (for indicators that only need close).
    pub fn from_close(close: f64) -> Self {
        Self {
            open: close,
            high: close,
            low: close,
            close,
            volume: 0.0,
        }
    }
}
