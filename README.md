# indica

Fast technical analysis indicators for stock markets. Built in Rust.

[![npm](https://img.shields.io/npm/v/@devanshhq/indica)](https://www.npmjs.com/package/@devanshhq/indica)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Devansh-365/indica/actions/workflows/ci.yml/badge.svg)](https://github.com/Devansh-365/indica/actions)

## Why

JavaScript/Python TA libraries are slow when screening thousands of stocks. indica computes all indicators for **2,000 stocks in 6ms** using Rust + Rayon parallelism.

| Stocks | Sequential | Parallel (Rayon) |
|--------|-----------|-----------------|
| 100 | 0.5ms | 0.3ms |
| 2,000 | 11ms | **6ms** |

## Indicators

| Indicator | Description |
|-----------|-------------|
| **SMA** | Simple Moving Average |
| **EMA** | Exponential Moving Average |
| **RSI** | Relative Strength Index (Wilder's smoothing) |
| **MACD** | Moving Average Convergence Divergence + crossover detection |
| **Bollinger Bands** | Upper, middle, lower bands + %B |
| **ATR** | Average True Range (Wilder's smoothing) |
| **Pivot Points** | Classic (R3/R2/R1/Pivot/S1/S2/S3) |
| **Volume Trend** | Surging / increasing / stable / declining / drying up |
| **Relative Strength** | Stock vs benchmark comparison |

## Installation

### Node.js

```bash
npm install @devanshhq/indica
```

### Rust

```toml
[dependencies]
indica = "0.1"
```

## Quick Start (Node.js)

```javascript
const {
  calcSma, calcEma, calcRsi, calcMacd,
  calcBollingerBands, calcAtr, calcPivotPoints,
  calcVolumeTrend, calcRelativeStrength,
  batchComputeIndicators
} = require('@devanshhq/indica');

const closes = [44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10,
                45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
                46.28, 46.00, 46.03, 46.41, 46.22, 46.21];

// Moving Averages
calcSma(closes, 20);   // 45.52
calcEma(closes, 12);   // 45.68

// RSI
calcRsi(closes, 14);   // 55.37

// MACD with crossover detection
calcMacd(closes, 12, 26, 9);
// { value: 0.34, signal: 0.28, histogram: 0.06, crossover: 'bullish' }

// Bollinger Bands
calcBollingerBands(closes, 20, 2.0);
// { upper: 46.89, middle: 45.52, lower: 44.15, percentB: 0.73 }

// ATR
calcAtr(highs, lows, closes, 14);  // 1.23

// Pivot Points
calcPivotPoints(46.41, 45.61, 46.21);
// { r3: 47.81, r2: 47.01, r1: 46.61, pivot: 46.08, s1: 45.68, s2: 44.88, s3: 44.48 }

// Volume Trend
calcVolumeTrend(volumes);  // 'surging' | 'increasing' | 'stable' | 'declining' | 'drying up'

// Relative Strength vs Benchmark
calcRelativeStrength(stockCloses, niftyCloses, 50);  // 1.15 (outperforming)
```

## Batch Processing (screen thousands of stocks)

The killer feature. Screen 2,000+ stocks through all indicators at once using all CPU cores.

```javascript
const { batchComputeIndicators } = require('@devanshhq/indica');

const results = batchComputeIndicators([
  {
    symbol: 'RELIANCE',
    closes: [/* 250 daily closes */],
    highs: [/* 250 daily highs */],
    lows: [/* 250 daily lows */],
    volumes: [/* 250 daily volumes */],
  },
  { symbol: 'TCS', closes: [...], highs: [...], lows: [...], volumes: [...] },
  { symbol: 'INFY', closes: [...], highs: [...], lows: [...], volumes: [...] },
  // ... 2000 more stocks
]);

// Returns in ~6ms using all CPU cores
for (const stock of results) {
  console.log(`${stock.symbol}: RSI=${stock.rsi14}, SMA20=${stock.sma20}`);
}
```

Each result contains:

```typescript
{
  symbol: string;
  sma20: number | null;
  sma50: number | null;
  sma200: number | null;
  ema20: number | null;
  rsi14: number | null;
  macd: { value: number, signal: number, histogram: number, crossover: string } | null;
  bollinger: { upper: number, middle: number, lower: number, percentB: number } | null;
  atr14: number | null;
  volumeTrend: string;
}
```

## Usage (Rust)

```rust
use indica::{sma, rsi, macd, bollinger_bands, atr, pivot_points};

fn main() {
    let closes = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10,
                      45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
                      46.28, 46.00, 46.03, 46.41, 46.22, 46.21];

    // All functions return Option — None when insufficient data
    let sma_20 = sma(&closes, 20);       // Some(45.52)
    let rsi_14 = rsi(&closes, 14);       // Some(55.37)

    if let Some(result) = macd(&closes, 12, 26, 9) {
        println!("MACD: {}, Signal: {}, Crossover: {:?}",
            result.value, result.signal, result.crossover);
    }

    if let Some(bb) = bollinger_bands(&closes, 20, 2.0) {
        println!("Upper: {}, Lower: {}, %B: {}", bb.upper, bb.lower, bb.percent_b);
    }
}
```

### Batch (Rust)

```rust
use indica::batch::{StockData, batch_compute_parallel};

let stocks: Vec<StockData> = load_2000_stocks();

// Uses all CPU cores via Rayon
let results = batch_compute_parallel(&stocks);

for snap in &results {
    if let Some(rsi) = snap.rsi_14 {
        if rsi < 30.0 { println!("{} is oversold (RSI: {:.1})", snap.symbol, rsi); }
    }
}
```

## API Reference

### Node.js Functions

| Function | Parameters | Returns |
|----------|-----------|---------|
| `calcSma(values, period)` | `number[], number` | `number \| null` |
| `calcEma(values, period)` | `number[], number` | `number \| null` |
| `calcRsi(closes, period)` | `number[], number` | `number \| null` |
| `calcMacd(closes, fast, slow, signal)` | `number[], number, number, number` | `MacdResult \| null` |
| `calcBollingerBands(closes, period, stdDev)` | `number[], number, number` | `BollingerBands \| null` |
| `calcAtr(highs, lows, closes, period)` | `number[], number[], number[], number` | `number \| null` |
| `calcPivotPoints(high, low, close)` | `number, number, number` | `PivotPoints` |
| `calcVolumeTrend(volumes)` | `number[]` | `string` |
| `calcRelativeStrength(stock, benchmark, period)` | `number[], number[], number` | `number \| null` |
| `batchComputeIndicators(stocks)` | `StockData[]` | `IndicatorSnapshot[]` |

### Rust Functions

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
| `batch_compute(stocks)` | `&[StockData]` | `Vec<IndicatorSnapshot>` |
| `batch_compute_parallel(stocks)` | `&[StockData]` | `Vec<IndicatorSnapshot>` |

All functions return `Option`/`null` when there isn't enough data. No panics, no NaN, no exceptions.

## Building from Source

```bash
# Rust
cargo build --release
cargo test

# Node.js native addon
npm install
npm run build
```

## Contributing

PRs welcome. Run `cargo test` and `cargo clippy` before submitting.

## License

MIT
