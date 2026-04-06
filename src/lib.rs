//! # indica
//!
//! Fast technical analysis indicators for stock markets.
//! Built for Indian markets. Screening to signals.
//!
//! ## Indicators
//!
//! **Trend:** SMA, EMA, Supertrend, ADX
//! **Momentum:** RSI, MACD, Stochastic
//! **Volatility:** Bollinger Bands, ATR
//! **Volume:** OBV, VWAP, Volume Trend
//! **Support/Resistance:** Pivot Points
//! **India-Specific:** Delivery Analysis, Circuit Limits
//!
//! ## Usage
//!
//! ```rust
//! use indica::{sma, rsi, supertrend};
//! ```

pub mod core;
pub mod indicators;

// ── Backward-compatible convenience re-exports ──

// Trend
pub use indicators::trend::adx::adx;
pub use indicators::trend::ema::ema;
pub use indicators::trend::sma::sma;
pub use indicators::trend::supertrend::{SupertrendDirection, SupertrendResult, supertrend};

// Momentum
pub use indicators::momentum::macd::{Crossover, MacdResult, macd};
pub use indicators::momentum::rsi::rsi;
pub use indicators::momentum::stochastic::{StochasticResult, stochastic};

// Volatility
pub use indicators::volatility::atr::atr;
pub use indicators::volatility::bollinger::{BollingerBandsResult, bollinger_bands};

// Volume
pub use indicators::volume::obv::obv;
pub use indicators::volume::volume_trend::volume_trend;
pub use indicators::volume::vwap::vwap;

// Support/Resistance
pub use indicators::support_resistance::pivot::{PivotPointsResult, pivot_points};

// India-specific
pub use indicators::india::circuit::{CircuitLimit, CircuitStatus, circuit_proximity};
pub use indicators::india::delivery::{DeliveryTrend, delivery_pct, delivery_trend};

// Core types
pub use core::traits::Indicator;
pub use core::types::Candle;

// Streaming indicator structs
pub use indicators::momentum::rsi::Rsi;
pub use indicators::trend::adx::Adx;
pub use indicators::trend::ema::Ema;
pub use indicators::trend::sma::Sma;
pub use indicators::trend::supertrend::Supertrend;
