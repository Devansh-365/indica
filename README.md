# indica

Fast technical analysis indicators for stock markets. Built in Rust.

[![Crates.io](https://img.shields.io/crates/v/indica)](https://crates.io/crates/indica)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Why

JavaScript/Python TA libraries are slow when screening thousands of stocks. indica computes all indicators for **2,000 stocks in 6ms** using Rust + Rayon parallelism.

| Stocks | Sequential | Parallel (Rayon) |
|--------|-----------|-----------------|
| 100 | 0.5ms | 0.3ms |
| 2,000 | 11ms | 6ms |

## Indicators

- **SMA** — Simple Moving Average
- **EMA** — Exponential Moving Average
- **RSI** — Relative Strength Index (Wilder's smoothing)
- **MACD** — Moving Average Convergence Divergence with crossover detection
- **Bollinger Bands** — Upper, middle, lower bands + %B
- **ATR** — Average True Range (Wilder's smoothing)
- **Pivot Points** — Classic (R3/R2/R1/Pivot/S1/S2/S3)
- **Volume Trend** — Surging / increasing / stable / declining / drying up
- **Relative Strength** — Stock vs benchmark comparison

## Installation

### Rust

```toml
[dependencies]
indica = "0.1"
```

### Node.js (via NAPI-RS)

```bash
npm install indica
```

## Usage (Rust)

```rust
use indica::{sma, rsi, macd, bollinger_bands, atr, pivot_points};

fn main() {
    let closes = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10,
                      45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
                      46.28, 46.00, 46.03, 46.41, 46.22, 46.21];

    // Simple Moving Average
    let sma_20 = sma(&closes, 20);
    println!("SMA(20): {:?}", sma_20); // Some(45.52)

    // RSI
    let rsi_14 = rsi(&closes, 14);
    println!("RSI(14): {:?}", rsi_14); // Some(55.37)

    // MACD
    if let Some(result) = macd(&closes, 12, 26, 9) {
        println!("MACD: {}, Signal: {}, Crossover: {:?}",
            result.value, result.signal, result.crossover);
    }

    // Bollinger Bands
    if let Some(bb) = bollinger_bands(&closes, 20, 2.0) {
        println!("BB Upper: {}, Lower: {}, %B: {}", bb.upper, bb.lower, bb.percent_b);
    }
}
```

## Batch Processing (screen thousands of stocks)

```rust
use indica::batch::{StockData, batch_compute_parallel};

let stocks = vec![
    StockData {
        symbol: "RELIANCE".to_string(),
        closes: vec![/* 250 daily closes */],
        highs: vec![/* ... */],
        lows: vec![/* ... */],
        volumes: vec![/* ... */],
    },
    // ... 2000 more stocks
];

// Computes ALL indicators for ALL stocks using all CPU cores
let results = batch_compute_parallel(&stocks);

for snap in &results {
    println!("{}: RSI={:?}, SMA20={:?}", snap.symbol, snap.rsi_14, snap.sma_20);
}
```

## Usage (Node.js)

```javascript
const { calcRsi, calcMacd, batchComputeIndicators } = require('indica');

// Single indicator
const rsi = calcRsi([44.34, 44.09, /* ... */], 14);
console.log('RSI:', rsi); // 55.37

// MACD with crossover detection
const macd = calcMacd(closes, 12, 26, 9);
console.log(macd); // { value: 0.34, signal: 0.28, histogram: 0.06, crossover: 'bullish' }

// Batch: screen 2000 stocks at once
const results = batchComputeIndicators([
  { symbol: 'RELIANCE', closes: [...], highs: [...], lows: [...], volumes: [...] },
  { symbol: 'TCS', closes: [...], highs: [...], lows: [...], volumes: [...] },
  // ... thousands more
]);
// Returns in ~6ms using all CPU cores
```

## API Reference

### Single Indicators

| Function | Input | Output |
|----------|-------|--------|
| `sma(values, period)` | `&[f64], usize` | `Option<f64>` |
| `ema(values, period)` | `&[f64], usize` | `Option<f64>` |
| `rsi(closes, period)` | `&[f64], usize` | `Option<f64>` |
| `macd(closes, fast, slow, signal)` | `&[f64], usize, usize, usize` | `Option<MacdResult>` |
| `bollinger_bands(closes, period, std_dev)` | `&[f64], usize, f64` | `Option<BollingerBandsResult>` |
| `atr(highs, lows, closes, period)` | `&[f64], &[f64], &[f64], usize` | `Option<f64>` |
| `pivot_points(high, low, close)` | `f64, f64, f64` | `PivotPointsResult` |
| `volume_trend(volumes)` | `&[f64]` | `&str` |
| `relative_strength(stock, bench, period)` | `&[f64], &[f64], usize` | `Option<f64>` |

### Batch

| Function | Input | Output |
|----------|-------|--------|
| `batch_compute(stocks)` | `&[StockData]` | `Vec<IndicatorSnapshot>` |
| `batch_compute_parallel(stocks)` | `&[StockData]` | `Vec<IndicatorSnapshot>` |

All functions return `Option` (Rust's null) when there isn't enough data for the calculation. No panics, no NaN.

## Building from Source

```bash
# Rust library
cargo build --release
cargo test

# Node.js native addon
npm install
npm run build
```

## License

MIT
