# indica — Architecture Design Document

## 1. Vision

indica is not "TA-Lib in Rust." It's a **trading intelligence library** built specifically for Indian stock markets. Three layers:

```
┌──────────────────────────────────────────────────────────┐
│                    SIGNALS LAYER                         │
│  Composite buy/sell signals with confidence + reasons    │
├──────────────────────────────────────────────────────────┤
│                  INDICATORS LAYER                        │
│  Standard TA (RSI, MACD, BB...) + India-specific         │
│  (delivery %, circuit proximity, FII/DII momentum)       │
├──────────────────────────────────────────────────────────┤
│                    CORE LAYER                            │
│  Streaming trait, batch processing, math utils           │
└──────────────────────────────────────────────────────────┘
     │              │              │
   Rust           NAPI          WASM
  (crate)       (Node.js)     (Browser)
```

## 2. Current State (v0.1)

```
src/
├── lib.rs              ← flat re-exports
├── moving_avg.rs       ← SMA, EMA, ema_series
├── rsi.rs              ← RSI
├── macd.rs             ← MACD
├── bollinger.rs        ← Bollinger Bands
├── atr.rs              ← ATR
├── pivot.rs            ← Pivot Points
├── volume.rs           ← Volume Trend
├── relative_strength.rs
├── batch.rs            ← Rayon parallel batch
├── napi_bindings.rs    ← Node.js FFI
└── utils.rs            ← round, wilders_smooth
```

Problems with current architecture:
- **Flat module structure** — everything in `src/`, will get messy at 20+ indicators
- **No shared trait** — each indicator is an independent function, no polymorphism
- **Batch is hardcoded** — `batch.rs` hardcodes which indicators to compute
- **NAPI bindings are manual** — every new indicator needs 20+ lines of boilerplate
- **No streaming** — must recompute from full array every time

## 3. Target Architecture (v1.0)

```
src/
├── lib.rs                      ← public API surface
│
├── core/                       ← CORE LAYER
│   ├── mod.rs
│   ├── traits.rs               ← Indicator trait (streaming + batch)
│   ├── types.rs                ← Candle, Signal, shared types
│   └── utils.rs                ← round, wilders_smooth, ema_step
│
├── indicators/                 ← INDICATORS LAYER
│   ├── mod.rs                  ← re-exports all indicators
│   ├── trend/                  ← Trend indicators
│   │   ├── mod.rs
│   │   ├── sma.rs
│   │   ├── ema.rs
│   │   ├── supertrend.rs
│   │   ├── adx.rs
│   │   └── ichimoku.rs         (future)
│   ├── momentum/               ← Momentum indicators
│   │   ├── mod.rs
│   │   ├── rsi.rs
│   │   ├── macd.rs
│   │   ├── stochastic.rs
│   │   └── cci.rs              (future)
│   ├── volatility/             ← Volatility indicators
│   │   ├── mod.rs
│   │   ├── bollinger.rs
│   │   └── atr.rs
│   ├── volume/                 ← Volume indicators
│   │   ├── mod.rs
│   │   ├── obv.rs
│   │   ├── vwap.rs
│   │   └── volume_trend.rs
│   ├── support_resistance/     ← Levels
│   │   ├── mod.rs
│   │   ├── pivot.rs
│   │   └── fibonacci.rs        (future)
│   └── india/                  ← INDIA-SPECIFIC (the moat)
│       ├── mod.rs
│       ├── delivery.rs         ← delivery %, delivery trend
│       ├── circuit.rs          ← circuit proximity, hit count
│       └── fii_dii.rs          ← institutional flow momentum
│
├── signals/                    ← SIGNALS LAYER
│   ├── mod.rs
│   ├── types.rs                ← Signal enum, Confidence, Reason
│   ├── engine.rs               ← Composite signal generator
│   └── presets.rs              ← Built-in strategies (swing, momentum, etc.)
│
├── batch/                      ← Batch processing
│   ├── mod.rs
│   ├── compute.rs              ← configurable batch compute
│   └── screen.rs               ← stock screening with filters
│
└── bindings/                   ← Platform bindings
    ├── mod.rs
    ├── napi.rs                 ← Node.js (feature-gated)
    └── wasm.rs                 ← Browser (feature-gated)
```

## 4. Key Architectural Decisions

### ADR-001: Streaming Indicator Trait

**Context:** Current indicators are pure functions that recompute from full arrays. Real-time trading needs O(1) updates.

**Decision:** Define an `Indicator` trait with two modes:

```rust
/// A candle of OHLCV data.
#[derive(Debug, Clone, Copy)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Core trait for all indicators.
/// Supports both streaming (one candle at a time) and batch (full array).
pub trait Indicator: Send + Sync {
    /// The output type (f64, MacdResult, BollingerBandsResult, etc.)
    type Output: Clone;

    /// Feed one new candle. Returns None while building up, Some once ready.
    fn update(&mut self, candle: &Candle) -> Option<Self::Output>;

    /// Reset internal state.
    fn reset(&mut self);

    /// Compute from a full array (convenience wrapper around update).
    fn compute(&mut self, candles: &[Candle]) -> Vec<Option<Self::Output>> {
        self.reset();
        candles.iter().map(|c| self.update(c)).collect()
    }

    /// Compute the final value only (most common use case).
    fn compute_last(&mut self, candles: &[Candle]) -> Option<Self::Output> {
        self.reset();
        let mut last = None;
        for c in candles {
            last = self.update(c);
        }
        last
    }
}
```

**Trade-offs:**
- (+) O(1) streaming updates for real-time use
- (+) Batch mode built on top of streaming (DRY)
- (+) All indicators share the same interface → configurable batch
- (-) Slightly more complex than pure functions for simple use cases
- (-) Indicators need internal state (mutable)

**Backward compatibility:** Keep the existing pure functions (`sma()`, `rsi()`, etc.) as convenience wrappers that create an indicator, call `compute_last`, and return. Zero breaking changes.

```rust
// Convenience function (existing API, unchanged)
pub fn rsi(closes: &[f64], period: usize) -> Option<f64> {
    let candles: Vec<Candle> = closes.iter()
        .map(|&c| Candle { open: c, high: c, low: c, close: c, volume: 0.0 })
        .collect();
    let mut ind = Rsi::new(period);
    ind.compute_last(&candles)
}

// Streaming API (new)
let mut rsi = Rsi::new(14);
for candle in live_feed {
    if let Some(value) = rsi.update(&candle) {
        println!("RSI: {}", value);
    }
}
```

---

### ADR-002: Candle as Universal Input

**Context:** Current functions take separate slices (`&[f64]` for closes, highs, lows, volumes). This leads to mismatched lengths and parameter explosion.

**Decision:** Introduce a `Candle` struct as the standard input type. Keep the `&[f64]` convenience functions for backward compatibility.

**Trade-offs:**
- (+) Type-safe: impossible to swap highs and lows
- (+) Simpler API: one `&[Candle]` instead of four `&[f64]` slices
- (+) Volume always available (needed for VWAP, OBV)
- (-) Slight overhead for indicators that only need close (e.g., SMA)
- (-) Users must construct Candle from their data format

---

### ADR-003: Feature-Gated Bindings

**Context:** NAPI-RS and WASM have heavy dependencies. Pure Rust users shouldn't need them.

**Decision:** Feature-gate bindings:

```toml
[features]
default = []
napi = ["dep:napi", "dep:napi-derive"]
wasm = ["dep:wasm-bindgen"]

[dependencies]
napi = { version = "3", features = ["napi4"], optional = true }
napi-derive = { version = "3", optional = true }
wasm-bindgen = { version = "0.2", optional = true }
rayon = "1.11"

[lib]
crate-type = ["lib"]  # default: pure Rust library

# NAPI users build with: cargo build --features napi
# WASM users build with: wasm-pack build --features wasm
```

**Trade-offs:**
- (+) Smaller dependency tree for pure Rust users
- (+) Clean separation of concerns
- (-) CI must test with and without features

---

### ADR-004: India-Specific Module

**Context:** No other TA library has Indian market indicators. This is the differentiation.

**Decision:** Dedicated `indicators/india/` module for indicators that require India-specific data:

```rust
/// Delivery volume analysis (unique to NSE/BSE).
pub struct DeliveryAnalysis {
    period: usize,
}

impl DeliveryAnalysis {
    /// Compute delivery percentage.
    /// delivery_pct = delivery_volume / total_volume * 100
    pub fn delivery_pct(delivery_vol: f64, total_vol: f64) -> f64;

    /// Delivery trend: compare recent avg delivery % to longer-term avg.
    /// High delivery % on up days = genuine buying.
    pub fn delivery_trend(
        delivery_pcts: &[f64],
        closes: &[f64],
        short_period: usize,
        long_period: usize,
    ) -> Option<DeliveryTrend>;
}

/// Circuit limit proximity detector.
pub fn circuit_proximity(
    current_price: f64,
    prev_close: f64,
    circuit_limit: CircuitLimit, // Percent2, Percent5, Percent10, Percent20
) -> CircuitStatus; // { upper_distance_pct, lower_distance_pct, near_upper, near_lower }

/// FII/DII flow momentum.
pub fn fii_dii_momentum(
    fii_flows: &[f64],   // Daily net FII values
    dii_flows: &[f64],   // Daily net DII values
    period: usize,
) -> Option<FlowMomentum>; // { fii_trend, dii_trend, divergence }
```

**These indicators cannot exist in any US/global TA library** because the underlying data (delivery volume, circuit limits, FII/DII flows) is unique to Indian exchanges.

---

### ADR-005: Signal Generation Engine

**Context:** Raw indicator values require interpretation. Traders need actionable signals.

**Decision:** A composable signal engine:

```rust
#[derive(Debug, Clone)]
pub enum SignalStrength {
    StrongBuy,
    Buy,
    Neutral,
    Sell,
    StrongSell,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub strength: SignalStrength,
    pub confidence: f64,       // 0.0 to 1.0
    pub reasons: Vec<String>,  // ["RSI oversold (28.5)", "MACD bullish crossover"]
}

/// Signal engine that combines multiple indicators.
pub struct SignalEngine {
    rules: Vec<Box<dyn SignalRule>>,
}

pub trait SignalRule: Send + Sync {
    fn evaluate(&self, snapshot: &IndicatorSnapshot) -> Option<SignalVote>;
}

// Built-in rules:
// - RsiRule: oversold < 30 = Buy, overbought > 70 = Sell
// - MacdCrossoverRule: bullish crossover = Buy, bearish = Sell
// - SupertrendRule: price above = Buy, below = Sell
// - VolumeConfirmationRule: surging volume confirms signal
// - DeliveryRule (India-specific): high delivery % confirms Buy
```

The engine collects votes from all rules, weights them, and produces a composite signal. Users can add custom rules.

**Trade-offs:**
- (+) Actionable output (Buy/Sell, not 67.3)
- (+) Extensible via trait
- (+) Composable — users mix and match rules
- (-) Opinions baked in (what counts as "oversold"?)
- (-) More complex API surface

**Mitigation:** Built-in presets (conservative, aggressive, swing, momentum) plus full customization.

---

## 5. Implementation Phases

### Phase 1: Core Refactor (Foundation)

Restructure into the module layout. Introduce `Candle` type and `Indicator` trait. Migrate existing indicators to implement the trait while keeping convenience functions.

**Files:** `core/traits.rs`, `core/types.rs`, `core/utils.rs`
**Breaking changes:** None (existing API preserved)

### Phase 2: New Indicators

Add the 5 most-requested indicators:

| Indicator | Category | Depends on | Effort |
|-----------|----------|------------|--------|
| Supertrend | trend | ATR | 2 hrs |
| VWAP | volume | Volume data | 2 hrs |
| Stochastic | momentum | High/Low/Close | 2 hrs |
| ADX | trend | ATR | 3 hrs |
| OBV | volume | Close + Volume | 1 hr |

### Phase 3: India-Specific Indicators

| Indicator | What it does | Data needed |
|-----------|-------------|-------------|
| `delivery_pct` | Delivery volume / total volume | Delivery + total vol |
| `delivery_trend` | Delivery % trend with price context | Delivery pcts + closes |
| `circuit_proximity` | Distance from circuit limits | Price + prev close |
| `fii_dii_momentum` | Institutional flow trend | Daily FII/DII values |

### Phase 4: Signal Engine

Build the signal generation layer. Start with 5 built-in rules + 2 presets (swing trader, momentum trader).

### Phase 5: Bindings

- Feature-gate NAPI behind `features = ["napi"]`
- Add WASM bindings behind `features = ["wasm"]`
- Auto-generate NAPI bindings for new indicators using a macro

### Phase 6: Series Output

Add `_series()` variants to all indicators that return `Vec<f64>` (full history). Needed for charting and backtesting.

---

## 6. Cargo.toml (Target State)

```toml
[package]
name = "indica"
version = "1.0.0"
edition = "2024"
description = "Fast technical analysis indicators for stock markets. Built for Indian markets."
license = "MIT"
repository = "https://github.com/Devansh-365/indica"
keywords = ["trading", "technical-analysis", "indicators", "nse", "stocks"]
categories = ["finance", "mathematics"]

[features]
default = []
napi = ["dep:napi", "dep:napi-derive"]
wasm = ["dep:wasm-bindgen"]
india = []       # India-specific indicators (delivery, circuit, FII/DII)
signals = []     # Signal generation engine

[dependencies]
rayon = "1.11"
napi = { version = "3", features = ["napi4"], optional = true }
napi-derive = { version = "3", optional = true }
wasm-bindgen = { version = "0.2", optional = true }

[lib]
crate-type = ["lib"]
```

## 7. Public API Surface (Target)

```rust
// ── Convenience functions (existing, backward-compatible) ──
indica::sma(closes, period) -> Option<f64>
indica::ema(closes, period) -> Option<f64>
indica::rsi(closes, period) -> Option<f64>
indica::macd(closes, fast, slow, signal) -> Option<MacdResult>
indica::bollinger_bands(closes, period, mult) -> Option<BollingerBandsResult>
indica::atr(highs, lows, closes, period) -> Option<f64>
indica::pivot_points(high, low, close) -> PivotPointsResult
indica::volume_trend(volumes) -> &str
indica::relative_strength(stock, bench, period) -> Option<f64>
indica::supertrend(highs, lows, closes, period, mult) -> Option<SupertrendResult>
indica::vwap(candles) -> Option<f64>
indica::stochastic(highs, lows, closes, k_period, d_period) -> Option<StochasticResult>
indica::adx(highs, lows, closes, period) -> Option<f64>
indica::obv(closes, volumes) -> f64

// ── Streaming API (new) ──
let mut rsi = indica::Rsi::new(14);
rsi.update(&candle) -> Option<f64>

// ── Batch (existing, enhanced) ──
indica::batch::compute_parallel(stocks) -> Vec<IndicatorSnapshot>
indica::batch::screen(stocks, filter) -> Vec<ScreenResult>

// ── India-specific (new) ──
indica::india::delivery_pct(delivery_vol, total_vol) -> f64
indica::india::delivery_trend(pcts, closes, short, long) -> Option<DeliveryTrend>
indica::india::circuit_proximity(price, prev_close, limit) -> CircuitStatus
indica::india::fii_dii_momentum(fii, dii, period) -> Option<FlowMomentum>

// ── Signals (new) ──
let engine = indica::signals::Engine::preset_swing();
engine.evaluate(&snapshot) -> Signal { strength, confidence, reasons }
```

## 8. Non-Functional Requirements

| Requirement | Target | How |
|-------------|--------|-----|
| Performance | 2,000 stocks in <10ms | Rayon parallelism, zero allocations in hot path |
| Correctness | Bit-exact with TA-Lib for standard indicators | Cross-reference tests against known values |
| Safety | Zero panics on any input | All unwrap() replaced with ?, fuzzing tests |
| Binary size | <500KB for WASM | Feature-gate heavy deps, LTO |
| Compatibility | Rust 1.85+ (edition 2024) | CI tests on MSRV |
| Platforms | macOS, Linux, Windows, WASM | CI matrix |

## 9. Risks

| Risk | Mitigation |
|------|------------|
| Streaming trait is too complex | Keep convenience functions as primary API |
| India indicators have no adoption | Ship them in Metis first, prove value |
| WASM + Rayon conflict | Rayon disabled in WASM, single-threaded batch |
| Signal engine is opinionated | Make rules configurable, presets optional |
| Breaking changes on refactor | Version bump to 0.2, deprecation warnings |
