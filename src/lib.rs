//! # indica
//!
//! Fast technical analysis indicators for stock markets.
//! SMA, EMA, RSI, MACD, Bollinger Bands, ATR, Pivot Points, and more.

mod moving_avg;
mod rsi;
mod macd;
mod bollinger;
mod atr;
mod pivot;
mod volume;
mod relative_strength;
mod utils;

pub use moving_avg::{sma, ema};
pub use rsi::rsi;
pub use macd::{macd, MacdResult, Crossover};
pub use bollinger::{bollinger_bands, BollingerBandsResult};
pub use atr::atr;
pub use pivot::{pivot_points, PivotPointsResult};
pub use volume::volume_trend;
pub use relative_strength::relative_strength;
