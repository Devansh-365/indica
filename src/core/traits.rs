use super::types::Candle;

/// Core trait for all streaming indicators.
///
/// Supports two modes:
/// - **Streaming**: feed candles one at a time via `update()` — O(1) per update
/// - **Batch**: pass a full array via `compute()` or `compute_last()`
///
/// All indicators implement this trait, enabling polymorphic batch processing
/// and consistent API across the library.
pub trait Indicator: Send + Sync {
    /// The output type (f64, MacdResult, BollingerBandsResult, etc.)
    type Output: Clone;

    /// Feed one new candle. Returns `None` while building up, `Some` once ready.
    fn update(&mut self, candle: &Candle) -> Option<Self::Output>;

    /// Reset internal state to start fresh.
    fn reset(&mut self);

    /// Compute the indicator for every candle in the array.
    /// Returns a Vec of the same length as input.
    fn compute(&mut self, candles: &[Candle]) -> Vec<Option<Self::Output>> {
        self.reset();
        candles.iter().map(|c| self.update(c)).collect()
    }

    /// Compute only the final value (most common use case).
    fn compute_last(&mut self, candles: &[Candle]) -> Option<Self::Output> {
        self.reset();
        let mut last = None;
        for c in candles {
            last = self.update(c);
        }
        last
    }
}
