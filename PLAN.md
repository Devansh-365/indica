# indica — Implementation Plan

> Fast technical analysis indicators for stock markets. Built in Rust.
> Learning Rust by building something real.

## What This Is

A Rust library that computes stock market technical indicators (SMA, RSI, MACD, Bollinger Bands, etc.) — the same math currently done in JavaScript in the Metis codebase (`lib/stock/indicators.ts`). The goal is to eventually use this from Node.js via NAPI-RS for 10-50x faster batch screening.

## Why Rust

- Learn Rust by building, not reading books
- Real performance gains for batch stock screening (2,000+ stocks)
- Publishable to crates.io (Rust) and npm (Node.js via NAPI)
- Portfolio project — first Rust TA library for Indian markets

## Current JS Reference

The TypeScript file we're reimplementing (`metis-2/lib/stock/indicators.ts`) has these functions:

```
sma(values, period)           → number        Simple Moving Average
ema(values, period)           → number        Exponential Moving Average
rsi(closes, period)           → number        RSI (Wilder's smoothing)
macd(closes, fast, slow, sig) → MACDResult    MACD with crossover detection
bollingerBands(closes, p, m)  → BBResult      Bollinger Bands
atr(highs, lows, closes, p)  → number        Average True Range
pivotPoints(high, low, close) → PivotResult   Classic Pivot Points
volumeTrend(volumes)          → string        Volume trend classification
relativeStrength(stock, bench, period) → number  RS vs benchmark
```

---

## Phase 0: Rust Fundamentals (You Are Here)

**Goal:** Get comfortable with Rust basics before writing indicators.

### Concepts to learn:
- [x] `fn`, return types, `pub`
- [x] `let` (immutable) vs `let mut`
- [x] `println!` macro
- [x] `#[test]` and `assert_eq!`
- [ ] **Ownership & borrowing** — Rust's core concept (`&`, `&mut`)
- [ ] **Slices** (`&[f64]`) — how Rust handles arrays without copying
- [ ] **Option** (`Some(value)` / `None`) — Rust's null replacement
- [ ] **Vec<f64>** — growable arrays (like JS arrays)
- [ ] **Iterators** (`.iter()`, `.map()`, `.sum()`, `.fold()`)
- [ ] **Structs** — like TypeScript interfaces but with data
- [ ] **Enums** — like TypeScript union types but more powerful
- [ ] **Error handling** (`Result<T, E>`) — no try/catch in Rust

### Mini exercises (do these in src/lib.rs):
1. Write a function that takes `&[f64]` and returns the sum
2. Write a function that returns `Option<f64>` (None if empty slice)
3. Write a function that returns a `Vec<f64>` (new computed array)
4. Create a struct with named fields and a method on it
5. Create an enum with variants and match on it

**When you're done:** You'll understand enough Rust to start Phase 1.

---

## Phase 1: Moving Averages — SMA & EMA

**Goal:** First real indicator. Teaches slices, iterators, Option.

### What to build:
```rust
/// Simple Moving Average of the last `period` values
pub fn sma(values: &[f64], period: usize) -> Option<f64>

/// Exponential Moving Average
pub fn ema(values: &[f64], period: usize) -> Option<f64>
```

### Rust concepts you'll use:
- `&[f64]` — borrowing a slice (read-only view of an array)
- `Option<f64>` — returns `None` instead of `NaN` when data is insufficient
- `.len()`, `.iter()`, `.sum::<f64>()` — iterator methods
- `.last()` — safely get last element

### Tests to write:
- SMA of `[1, 2, 3, 4, 5]` with period 3 → 4.0
- SMA with insufficient data → None
- EMA matches known values
- Edge cases: empty slice, period=1, period=len

### File structure:
```
src/
  lib.rs          ← re-exports everything
  moving_avg.rs   ← sma() and ema()
```

---

## Phase 2: RSI — Wilder's Smoothing

**Goal:** More complex math. Teaches mutable state + loops.

### What to build:
```rust
/// Relative Strength Index (Wilder's smoothing)
pub fn rsi(closes: &[f64], period: usize) -> Option<f64>
```

### Rust concepts you'll use:
- `let mut` — mutable variables (for running averages)
- `for i in 1..closes.len()` — range-based loops
- `.abs()` — method on f64
- Pattern: seed with initial average, then smooth iteratively

### Tests:
- RSI of known data matches expected output
- Oversold (RSI < 30) and overbought (RSI > 70) cases
- All gains → RSI = 100, all losses → RSI = 0
- Edge: insufficient data → None

### File:
```
src/
  rsi.rs
```

---

## Phase 3: MACD — Structs & Enums

**Goal:** Return complex data. Teaches structs, enums, derive macros.

### What to build:
```rust
#[derive(Debug, Clone)]
pub enum Crossover {
    Bullish,
    Bearish,
    None,
}

#[derive(Debug, Clone)]
pub struct MacdResult {
    pub value: f64,
    pub signal: f64,
    pub histogram: f64,
    pub crossover: Crossover,
}

pub fn macd(
    closes: &[f64],
    fast: usize,    // default 12
    slow: usize,    // default 26
    signal: usize,  // default 9
) -> Option<MacdResult>
```

### Rust concepts:
- `struct` with `pub` fields
- `enum` with variants (no values)
- `#[derive(Debug, Clone)]` — auto-generate formatting and copying
- `Vec<f64>` — building the MACD line series
- Internal helper: reuse `ema()` from Phase 1 or compute inline

### File:
```
src/
  macd.rs
```

---

## Phase 4: Bollinger Bands & ATR

**Goal:** Statistics (std dev) and multi-input functions.

### What to build:
```rust
#[derive(Debug, Clone)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub percent_b: f64,
}

pub fn bollinger_bands(closes: &[f64], period: usize, std_dev_mult: f64) -> Option<BollingerBands>

/// Average True Range (Wilder's smoothing)
pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64>
```

### Rust concepts:
- `.sqrt()`, `.powi(2)` — math on f64
- Multiple slice parameters (`highs`, `lows`, `closes`)
- `.zip()` — iterating multiple slices together
- `f64::max()` / `.max()` — finding maximum of values

### Files:
```
src/
  bollinger.rs
  atr.rs
```

---

## Phase 5: Pivot Points, Volume Trend, Relative Strength

**Goal:** Complete the indicator set. Teaches &str returns and simple logic.

### What to build:
```rust
#[derive(Debug, Clone)]
pub struct PivotPoints {
    pub r3: f64, pub r2: f64, pub r1: f64,
    pub pivot: f64,
    pub s1: f64, pub s2: f64, pub s3: f64,
}

pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPoints

pub fn volume_trend(volumes: &[f64]) -> &'static str

pub fn relative_strength(stock: &[f64], benchmark: &[f64], period: usize) -> Option<f64>
```

### Rust concepts:
- `&'static str` — string literals that live forever (like "surging", "declining")
- Simple arithmetic (no new concepts, just practice)
- Lifetime annotation basics (the `'static` part)

### Files:
```
src/
  pivot.rs
  volume.rs
  relative_strength.rs
```

---

## Phase 6: Batch Processing

**Goal:** Compute indicators for 2,000+ stocks at once. Where Rust shines.

### What to build:
```rust
#[derive(Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub closes: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub volumes: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct IndicatorSnapshot {
    pub symbol: String,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd: Option<MacdResult>,
    pub bb: Option<BollingerBands>,
    pub atr_14: Option<f64>,
}

/// Compute all indicators for multiple stocks
pub fn batch_compute(stocks: &[StockData]) -> Vec<IndicatorSnapshot>
```

### Rust concepts:
- `String` vs `&str` — owned vs borrowed strings
- `Vec<T>` — vectors of structs
- **Rayon** (parallel iterator crate) — `stocks.par_iter().map(...)` for multi-core
- `cargo add rayon` — adding dependencies

### Why this matters:
This is the function that Metis would call from Node.js. Screening 2,000 stocks through all indicators in parallel — Rust + Rayon can do this in milliseconds vs seconds in JS.

---

## Phase 7: NAPI-RS Bindings (Connect to Node.js)

**Goal:** Make the library callable from Metis's TypeScript code.

### What to do:
1. Add napi-rs dependencies to Cargo.toml
2. Create `src/napi.rs` with `#[napi]` annotated wrapper functions
3. Build with `cargo build --release`
4. Import in Metis: `import { sma, rsi, macd } from 'indica'`

### Rust concepts:
- `#[napi]` macro — generates JS bindings
- Type conversion: JS number[] ↔ Rust Vec<f64>
- Publishing to npm via napi-rs CLI

### This is the payoff:
```typescript
// In Metis — same API, 50x faster
import { batchCompute } from 'indica';

const results = batchCompute(allStocks); // milliseconds, not seconds
```

---

## File Structure (final)

```
indica/
├── Cargo.toml
├── src/
│   ├── lib.rs              ← re-exports all modules
│   ├── moving_avg.rs       ← Phase 1: SMA, EMA
│   ├── rsi.rs              ← Phase 2: RSI
│   ├── macd.rs             ← Phase 3: MACD
│   ├── bollinger.rs        ← Phase 4: Bollinger Bands
│   ├── atr.rs              ← Phase 4: ATR
│   ├── pivot.rs            ← Phase 5: Pivot Points
│   ├── volume.rs           ← Phase 5: Volume Trend
│   ├── relative_strength.rs← Phase 5: RS vs Benchmark
│   ├── batch.rs            ← Phase 6: Batch processing
│   └── napi.rs             ← Phase 7: Node.js bindings
├── tests/
│   └── integration.rs      ← Cross-module integration tests
├── benches/
│   └── indicators.rs       ← Performance benchmarks
├── PLAN.md                 ← This file
└── README.md
```

---

## Learning Resources

- **The Rust Book** (free): https://doc.rust-lang.org/book/
  - Ch 4: Ownership (READ THIS FIRST)
  - Ch 5: Structs
  - Ch 6: Enums and Pattern Matching
  - Ch 8: Vectors
  - Ch 13: Iterators
- **Rust by Example** (free): https://doc.rust-lang.org/rust-by-example/
- **Exercism Rust Track** (free): https://exercism.org/tracks/rust
- **ta crate** (study code): https://github.com/greyblake/ta-rs

---

## How We'll Work

1. Start each phase by reading the relevant Rust Book chapter
2. Write the functions with tests
3. `cargo test -- --nocapture` to verify
4. Commit after each phase
5. Ask questions when stuck — that's the whole point
