<div align="center">

# indica

**Technical analysis. Rust speed. Indian market intelligence.**

[npm](https://www.npmjs.com/package/@devanshhq/indica) · [Docs](https://docs.rs/indica) · [GitHub](https://github.com/Devansh-365/indica)

[![npm](https://img.shields.io/npm/v/@devanshhq/indica?label=npm&color=cb3837)](https://www.npmjs.com/package/@devanshhq/indica)
[![license](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-74_passing-brightgreen)](https://github.com/Devansh-365/indica/actions)

</div>

---

## The problem

You want to screen 2,000 stocks through RSI, MACD, Supertrend, and Bollinger Bands.

In JavaScript, that takes **8 seconds**.
In Python, about **3 seconds**.
In indica, **6 milliseconds**.

And no other library — in any language — computes delivery volume trends or circuit limit proximity. Because those are India-only concepts, and indica was built for India.

---

## Install

```bash
cargo add indica        # Rust
npm i @devanshhq/indica # Node.js
```

---

## 30-second demo

```rust
use indica::*;

let closes = vec![/* 250 daily closes */];

// Indicators — one function, one answer
let rsi_val   = rsi(&closes, 14);                      // Some(62.4)
let macd_val  = macd(&closes, 12, 26, 9);               // Some(MacdResult { crossover: Bullish })
let bb        = bollinger_bands(&closes, 20, 2.0);       // Some(BollingerBandsResult { %B: 0.73 })
let st        = supertrend(&highs, &lows, &closes, 10, 3.0); // Some({ direction: Up })

// Signal engine — from numbers to decisions
let engine = signals::presets::swing_trader();
let signal = engine.evaluate(&values);
// → Signal { strength: StrongBuy, confidence: 0.82,
//            reasons: ["RSI oversold", "MACD bullish crossover"] }

// Batch — 2,000 stocks, all indicators, all cores
let results = batch::batch_compute_parallel(&stocks); // 6ms

// Screen — find what matters
let picks = batch::screen::screen(&stocks, &[
    ScreenFilter::RsiBelow(30.0),
    ScreenFilter::SupertrendUp,
]);
```

---

## What's inside

```
 SIGNALS          Signal engine with composable rules
                  Buy/Sell/Neutral with confidence scores
                  Presets: swing trader, momentum trader
                  ─────────────────────────────────────
 INDICATORS       16 indicators across 6 categories
                  Streaming mode: O(1) per tick update
                  ─────────────────────────────────────
 CORE             Candle type, Indicator trait
                  Rayon parallel batch processing
                  Stock screening with filters
```

### Indicators

| | | |
|---|---|---|
| **Trend** | SMA · EMA · Supertrend · ADX | |
| **Momentum** | RSI · MACD · Stochastic | |
| **Volatility** | Bollinger Bands · ATR | |
| **Volume** | OBV · VWAP · Volume Trend | |
| **Levels** | Pivot Points (R3 → S3) | |
| **India** | Delivery % · Circuit Limits | ← *no other library has this* |

---

## Streaming — real-time, zero waste

Every indicator implements the `Indicator` trait. Feed one candle, get one update. O(1). No recomputation.

```rust
use indica::{Rsi, Supertrend, Candle, Indicator};

let mut rsi = Rsi::new(14);
let mut st  = Supertrend::new(10, 3.0);

for candle in live_feed {
    if let Some(r) = rsi.update(&candle) {
        if r < 30.0 { println!("oversold: {:.1}", r); }
    }
    if let Some(s) = st.update(&candle) {
        println!("{}: {:.2}", s.direction, s.value);
    }
}
```

---

## Signals — stop reading numbers, start making decisions

Every TA library gives you `RSI = 28.5`. None tells you what to do with it.

```rust
use indica::signals::{presets::swing_trader, engine::IndicatorValues};

let engine = swing_trader();

let signal = engine.evaluate(&IndicatorValues {
    rsi: Some(28.5),
    macd: Some(macd_result),     // bullish crossover
    supertrend: Some(st_result), // direction: Up
    volume_trend: "surging".into(),
    ..Default::default()
});

// Signal {
//   strength: StrongBuy,
//   confidence: 0.85,
//   reasons: [
//     "RSI 28.5 — oversold",
//     "MACD bullish crossover",
//     "Supertrend — uptrend",
//     "Volume surging — confirms"
//   ]
// }
```

Built-in rules: RSI · MACD Crossover · Supertrend · Volume · ADX · Stochastic

Presets: `swing_trader()` · `momentum_trader()`

Or build your own — implement `SignalRule` and plug it in.

---

## India-only indicators 🇮🇳

These exist because NSE/BSE publish data that no other exchange does.

**Delivery Volume Analysis** — NSE reports how much volume was actually delivered vs speculated. High delivery on up-days = real buying.

```rust
let pct = delivery_pct(500_000.0, 1_000_000.0); // 50%

let trend = delivery_trend(&pcts, &closes, 3, 10);
// → StrongAccumulation (high delivery + price rising)
```

**Circuit Limit Proximity** — Indian stocks have 2/5/10/20% daily price caps. Know when you're near one.

```rust
let status = circuit_proximity(108.0, 100.0, CircuitLimit::Percent10);
// upper: 110.0, distance: 1.82%, near_upper: false
```

---

## Batch + Screening

Screen thousands of stocks. Find the ones that matter.

```rust
use indica::batch::{batch_compute_parallel, screen::{screen, ScreenFilter}};

// All indicators, all stocks, all cores
let snapshots = batch_compute_parallel(&stocks); // 2,000 stocks → 6ms

// Filter: oversold + uptrend + strong trend
let picks = screen(&stocks, &[
    ScreenFilter::RsiBelow(30.0),
    ScreenFilter::SupertrendUp,
    ScreenFilter::AdxAbove(25.0),
]);
```

---

## Performance

Benchmarked on Apple M1, release mode, 250 daily candles per stock:

| Stocks | Sequential | Parallel |
|--------|-----------|----------|
| 100 | 0.5ms | 0.3ms |
| 500 | 2.5ms | 1.2ms |
| 2,000 | 11ms | **6ms** |

---

## API at a glance

<details>
<summary>All 16 convenience functions</summary>

```
sma(closes, period)                           → Option<f64>
ema(closes, period)                           → Option<f64>
rsi(closes, period)                           → Option<f64>
macd(closes, fast, slow, signal)              → Option<MacdResult>
stochastic(highs, lows, closes, k, d)         → Option<StochasticResult>
bollinger_bands(closes, period, mult)         → Option<BollingerBandsResult>
atr(highs, lows, closes, period)              → Option<f64>
supertrend(highs, lows, closes, period, mult) → Option<SupertrendResult>
adx(highs, lows, closes, period)              → Option<f64>
obv(closes, volumes)                          → Option<f64>
vwap(highs, lows, closes, volumes)            → Option<f64>
volume_trend(volumes)                         → &str
pivot_points(high, low, close)                → PivotPointsResult
delivery_pct(delivery_vol, total_vol)         → f64
delivery_trend(pcts, closes, short, long)     → Option<DeliveryTrend>
circuit_proximity(price, prev, limit)         → CircuitStatus
```

Returns `Option` when data is insufficient. No panics. No NaN.

</details>

---

## Contributing

```bash
cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

---

<div align="center">

MIT · Built by [Devansh Tiwari](https://github.com/Devansh-365)

</div>
