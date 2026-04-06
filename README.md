# indica

Fast technical analysis indicators for stock markets. Built in Rust. Built for Indian markets.

[![npm](https://img.shields.io/npm/v/@devanshhq/indica)](https://www.npmjs.com/package/@devanshhq/indica)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Devansh-365/indica/actions/workflows/ci.yml/badge.svg)](https://github.com/Devansh-365/indica/actions)

## Why

JavaScript/Python TA libraries are slow when screening thousands of stocks. indica computes all indicators for **2,000 stocks in 6ms** using Rust + Rayon parallelism.

No other TA library has India-specific indicators (delivery volume analysis, circuit limit detection).

## Indicators (16)

| Category | Indicators |
|----------|-----------|
| **Trend** | SMA, EMA, Supertrend, ADX |
| **Momentum** | RSI, MACD (with crossover), Stochastic (%K/%D) |
| **Volatility** | Bollinger Bands (%B), ATR |
| **Volume** | OBV, VWAP, Volume Trend |
| **Support/Resistance** | Classic Pivot Points (R3-S3) |
| **India-Specific** | Delivery % Analysis, Circuit Limit Proximity |

## Installation

```toml
# Rust
[dependencies]
indica = "0.1"
```

## Quick Start

```rust
use indica::{sma, rsi, macd, supertrend, bollinger_bands, stochastic, vwap, obv};

let closes = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10,
                  45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
                  46.28, 46.00, 46.03, 46.41, 46.22, 46.21];

sma(&closes, 20);                           // Some(45.52)
rsi(&closes, 14);                           // Some(55.37)
macd(&closes, 12, 26, 9);                   // Some(MacdResult { crossover: Bullish, ... })
bollinger_bands(&closes, 20, 2.0);          // Some(BollingerBandsResult { upper, lower, %B })
stochastic(&highs, &lows, &closes, 14, 3); // Some(StochasticResult { k, d })
supertrend(&highs, &lows, &closes, 10, 3.0); // Some(SupertrendResult { direction: Up })
vwap(&highs, &lows, &closes, &volumes);    // Some(245.67)
obv(&closes, &volumes);                     // Some(1234567.0)
```

## Streaming API (real-time)

Feed candles one at a time — O(1) updates, no recomputation:

```rust
use indica::{Rsi, Sma, Candle, Indicator};

let mut rsi = Rsi::new(14);
let mut sma = Sma::new(20);

for candle in live_feed {
    if let Some(value) = rsi.update(&candle) {
        println!("RSI: {:.1}", value);
    }
    if let Some(value) = sma.update(&candle) {
        println!("SMA: {:.2}", value);
    }
}
```

Available streaming indicators: `Sma`, `Ema`, `Rsi`, `Supertrend`, `Adx`

## India-Specific Indicators

Indicators that only work with Indian market data (NSE/BSE):

```rust
use indica::{delivery_pct, delivery_trend, circuit_proximity, CircuitLimit};

// Delivery Volume Analysis (unique to NSE/BSE)
let pct = delivery_pct(500_000.0, 1_000_000.0); // 50.0%

// Delivery trend: high delivery + price up = genuine buying
let trend = delivery_trend(&delivery_pcts, &closes, 3, 10);
// Some(DeliveryTrend::Accumulation)

// Circuit Limit Proximity
let status = circuit_proximity(108.0, 100.0, CircuitLimit::Percent10);
// CircuitStatus { upper_limit: 110.0, near_upper: false, ... }
```

## Architecture

```
src/
├── core/               ← Candle, Indicator trait, utils
├── indicators/
│   ├── trend/          ← SMA, EMA, Supertrend, ADX
│   ├── momentum/       ← RSI, MACD, Stochastic
│   ├── volatility/     ← Bollinger Bands, ATR
│   ├── volume/         ← OBV, VWAP, Volume Trend
│   ├── support_resistance/ ← Pivot Points
│   └── india/          ← Delivery Analysis, Circuit Limits
├── signals/            ← Signal engine (Buy/Sell with confidence)
└── batch/              ← Parallel batch processing + screening
```

## API Reference

### Convenience Functions

| Function | Returns |
|----------|---------|
| `sma(closes, period)` | `Option<f64>` |
| `ema(closes, period)` | `Option<f64>` |
| `rsi(closes, period)` | `Option<f64>` |
| `macd(closes, fast, slow, signal)` | `Option<MacdResult>` |
| `stochastic(highs, lows, closes, k, d)` | `Option<StochasticResult>` |
| `bollinger_bands(closes, period, mult)` | `Option<BollingerBandsResult>` |
| `atr(highs, lows, closes, period)` | `Option<f64>` |
| `supertrend(highs, lows, closes, period, mult)` | `Option<SupertrendResult>` |
| `adx(highs, lows, closes, period)` | `Option<f64>` |
| `obv(closes, volumes)` | `Option<f64>` |
| `vwap(highs, lows, closes, volumes)` | `Option<f64>` |
| `volume_trend(volumes)` | `&str` |
| `pivot_points(high, low, close)` | `PivotPointsResult` |
| `delivery_pct(delivery_vol, total_vol)` | `f64` |
| `delivery_trend(pcts, closes, short, long)` | `Option<DeliveryTrend>` |
| `circuit_proximity(price, prev_close, limit)` | `CircuitStatus` |

All functions return `Option` when there isn't enough data. No panics, no NaN.

## Building

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
```

## Contributing

PRs welcome. Run `cargo test && cargo clippy -- -D warnings && cargo fmt --check` before submitting.

## License

MIT
