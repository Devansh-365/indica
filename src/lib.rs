//! # indica
//!
//! Fast technical analysis indicators for stock markets.
//! SMA, EMA, RSI, MACD, Bollinger Bands, ATR, Pivot Points, and more.

mod atr;
pub mod batch;
mod bollinger;
mod macd;
mod moving_avg;
mod napi_bindings;
mod pivot;
mod relative_strength;
mod rsi;
mod utils;
mod volume;

pub use atr::atr;
pub use bollinger::{BollingerBandsResult, bollinger_bands};
pub use macd::{Crossover, MacdResult, macd};
pub use moving_avg::{ema, sma};
pub use pivot::{PivotPointsResult, pivot_points};
pub use relative_strength::relative_strength;
pub use rsi::rsi;
pub use volume::volume_trend;
