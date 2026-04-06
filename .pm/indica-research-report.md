# indica -- Research Report: What Would Make This Library Stand Out

**Date**: 2026-04-06
**Current State**: SMA, EMA, RSI, MACD, Bollinger Bands, ATR, Pivot Points, Volume Trend, Relative Strength + Rayon batch + NAPI-RS bindings

---

## 1. Competitive Landscape: Rust TA Libraries

### ta-rs (crate: `ta`)
- **GitHub**: 842 stars | **Downloads**: 213,806 total, ~33K recent
- **Last updated**: June 2021 (effectively abandoned)
- **Indicators**: EMA, SMA, RSI, Fast/Slow Stochastic, MACD, PPO, CCI, MFI, True Range, ATR, SD, MAD, Efficiency Ratio, Bollinger Bands, Chandelier Exit, Keltner Channel, ROC, OBV, Min, Max
- **Architecture**: Streaming/incremental via `Next<T>` trait -- feed one value, get one value back. This is the gold standard pattern.
- **Key strength**: Clean trait-based API, streaming by design
- **Key weakness**: Abandoned for 5 years, limited indicator count (~20), no WASM, no batch

### yata (crate: `yata`)
- **GitHub**: 390 stars | **Downloads**: 179,046 total, ~10K recent
- **Last updated**: March 2024 (slow)
- **Indicators**: 30+ including Ichimoku Cloud, Parabolic SAR, KAMA, Fisher Transform, Awesome Oscillator, Chaikin Money Flow, Elders Force Index, Know Sure Thing + 15 moving average types
- **Architecture**: Configurable numeric types (f32/f64), Heikin Ashi + Renko conversions, serde support
- **Key strength**: Broadest indicator coverage in Rust, timeframe collapsing
- **Key weakness**: Benchmarks show 3-125 ns/iter (fast), but no streaming API, no WASM, no language bindings

### kand (crate: `kand`)
- **GitHub**: 539 stars | **Downloads**: 13,119 total, ~2K recent
- **Last updated**: March 2025 (active)
- **Indicators**: 60+ including ADX, Supertrend, VWAP, Vegas, Aroon, SAR, Stochastic, Williams %R, DEMA, TEMA, T3, candlestick patterns (Doji, Hammer, Marubozu), plus planned statistical indicators (Sharpe, Sortino, Calmar, Kelly criterion)
- **Architecture**: O(1) incremental updates (`ema_inc()` pattern), PyO3 Python bindings, WASM bindings, both f32 and f64, Result-based error handling, mutable buffer outputs for zero-alloc
- **Key strength**: Most modern, broadest feature set, multi-language, incremental design
- **Key weakness**: Very new, unstable API, heavy scope creep risk

### Summary: Where indica Sits

| Feature | ta-rs | yata | kand | indica (current) |
|---------|-------|------|------|-------------------|
| Indicators | ~20 | 30+ | 60+ | 9 |
| Streaming/incremental | Yes | No | Partial | No |
| WASM | No | No | Yes | No |
| Python bindings | No | No | Yes | No |
| Node.js bindings | No | No | Yes (WASM) | Yes (NAPI-RS) |
| Batch parallel | No | No | No | Yes (Rayon) |
| f32/f64 configurable | No | Yes | Yes | No |
| Signal generation | No | No | No | Partial (MACD crossover) |
| India-specific | No | No | No | No |
| Candlestick patterns | No | No | Yes | No |
| Serde support | Optional | Yes | No | No |

**The gap**: Nobody does India-specific indicators. Nobody combines batch parallel processing with streaming updates. Nobody generates actionable trading signals (just raw values). These are indica's opportunities.

---

## 2. Most Requested Missing Indicators (Ranked by Demand)

Based on TradingView popularity, Reddit algotrading discussions, Zerodha/Indian trader usage, and what competing libraries offer:

### Tier 1: High demand, missing from indica, relatively easy to implement

1. **VWAP (Volume-Weighted Average Price)** -- The #1 institutional indicator. Resets daily. Formula: cumulative(price * volume) / cumulative(volume). Essential for intraday. Only kand has it in Rust.

2. **Supertrend** -- Massively popular in India specifically (Zerodha made it famous). Uses ATR + multiplier. You already have ATR, so this is straightforward. Default: period=7, multiplier=3. Generates direct buy/sell signals.

3. **Stochastic Oscillator (Fast + Slow)** -- Classic overbought/oversold. ta-rs has it, kand has it, yata has it. You don't. Formula needs high/low/close.

4. **ADX (Average Directional Index)** -- Measures trend strength (not direction). Values >25 = strong trend, <20 = weak. Pairs perfectly with Supertrend. Requires +DI/-DI computation.

5. **OBV (On-Balance Volume)** -- Simplest volume indicator: cumulative volume where up days add and down days subtract. Shows money flow before price moves.

### Tier 2: High demand, moderate complexity

6. **Ichimoku Cloud** -- Complete trading system in one indicator: Tenkan-sen, Kijun-sen, Senkou Span A/B, Chikou Span. Very popular with position traders. Complex but well-documented math.

7. **Parabolic SAR** -- Trailing stop indicator. Generates stop-loss levels automatically. Wilder's original.

8. **Williams %R** -- Similar to Stochastic but inverted scale. Quick to implement.

9. **CCI (Commodity Channel Index)** -- Mean-reversion indicator. Identifies cyclical turns.

10. **MFI (Money Flow Index)** -- "Volume-weighted RSI". Needs typical price + volume.

### Tier 3: Differentiating, less common in open-source

11. **Fibonacci Retracement Levels** -- Auto-detect swing high/low, compute 23.6%, 38.2%, 50%, 61.8%, 78.6% levels. Most libraries skip this because it requires swing detection, not just math on a series.

12. **Keltner Channels** -- Like Bollinger but using ATR instead of standard deviation. You have ATR and EMA already.

13. **Donchian Channels** -- High/low over N periods. Simple but useful for breakout systems.

14. **DEMA/TEMA** -- Double/Triple EMA. Reduces lag.

15. **Heikin Ashi candle transformation** -- Modified OHLC for smoother trends.

---

## 3. India-Specific Features (Unique Differentiator -- Nobody Else Does This)

This is where indica can genuinely stand apart from every other TA library in any language.

### 3a. Delivery Volume Analysis
Indian exchanges (NSE/BSE) uniquely report delivery volume separately from total traded volume. This metric does not exist in US/EU markets.

- **Delivery Percentage Indicator**: `delivery_volume / total_volume * 100`. High delivery % (>60%) on an up day = genuine buying (not speculative). Low delivery % on a big move = likely reversal.
- **Delivery Volume Trend**: Compare 5-day avg delivery% vs 20-day avg. Rising delivery% in uptrend = strong conviction.
- **Delivery-Adjusted OBV**: Weight OBV by delivery percentage instead of raw volume. More meaningful for Indian stocks.

### 3b. FII/DII Flow Indicators
Foreign Institutional Investors and Domestic Institutional Investors publish daily net buy/sell data. No TA library computes indicators on this.

- **FII Flow Momentum**: Running sum of FII net purchases over N days. Positive = bullish institutional flow.
- **FII-DII Divergence**: When FII sells but DII buys aggressively, historically marks bottoms.
- **Market Strength Score**: Aggregate FII Cash + DII Absorption + Net Liquidity into 0-100 score. <30 = bear regime, >60 = bull regime.

### 3c. Circuit Limit Detection
Indian stocks have daily circuit limits (2%, 5%, 10%, 20%) that halt trading. No other TA library handles this.

- **Circuit Proximity Indicator**: How close is current price to upper/lower circuit? Expressed as percentage.
- **Circuit Hit Counter**: Count of circuit hits in last N days. Stocks hitting upper circuits repeatedly = strong momentum or illiquidity risk.
- **Circuit-Adjusted ATR**: ATR calculation that accounts for truncated trading days when circuits are hit.

### 3d. Market Session Awareness
- **Pre-open auction detection**: NSE has a 9:00-9:15 pre-open session with different dynamics.
- **Muhurat Trading flag**: Special Diwali trading session (1 hour). Historically bullish bias.

### Impact Assessment
These India-specific features would make indica the **only** technical analysis library in any language that understands Indian market microstructure. This is a genuine moat. A Rust library with "built for Indian markets" positioning and Zerodha/Kite integration examples would attract significant attention from the Indian algotrading community.

---

## 4. Architecture Improvements (Ranked by Impact)

### 4a. Streaming/Incremental API (HIGH IMPACT)

This is the single most important architectural upgrade. Every serious competitor (ta-rs, kand, trading-signals) has this or is building it.

**Current problem**: Every indica function recomputes from scratch. To update RSI with one new candle, you re-process all 250 candles. In a real-time system processing ticks for 2,000 stocks, this is O(n) per tick per stock.

**Proposed design** -- trait-based streaming:

```rust
pub trait Indicator {
    type Output;
    /// Feed one new value, get updated result
    fn update(&mut self, value: f64) -> Self::Output;
    /// Reset state
    fn reset(&mut self);
}

// Usage:
let mut rsi = Rsi::new(14);
for candle in historical_data {
    let value = rsi.update(candle.close);
}
// New tick arrives -- O(1) update
let current_rsi = rsi.update(new_close);
```

**Keep the existing batch functions** as convenience wrappers that create an indicator, feed all data, return the final value. This preserves backward compatibility while adding streaming.

**Priority**: Do this BEFORE adding more indicators. It changes the internal architecture of every indicator.

### 4b. WASM Compilation (MEDIUM-HIGH IMPACT)

kand already has this. The value proposition is massive:
- Run TA calculations in the browser (TradingView-like charting apps)
- No server round-trip for indicator computation
- Works with any JS framework (React, Vue, etc.)

**Implementation path**:
1. Feature-gate NAPI-RS behind `#[cfg(feature = "napi")]`
2. Add `wasm-bindgen` behind `#[cfg(feature = "wasm")]`
3. Compile with `wasm-pack build --target web`
4. Publish to npm as `@devanshhq/indica-wasm`

**Blocker**: Rayon does not work in WASM. Batch parallel must be feature-gated separately. Single-stock indicators work fine in WASM.

### 4c. Signal Generation Layer (HIGH IMPACT, Unique Differentiator)

No Rust TA library generates actionable signals. They all return raw numbers (RSI = 67.3) and leave interpretation to the user. This is the biggest gap in the entire ecosystem.

**Proposed design**:

```rust
pub enum Signal {
    StrongBuy,
    Buy,
    Neutral,
    Sell,
    StrongSell,
}

pub struct SignalResult {
    pub signal: Signal,
    pub confidence: f64,     // 0.0 to 1.0
    pub reason: &'static str, // "RSI oversold + MACD bullish crossover"
    pub stop_loss: Option<f64>,
    pub target: Option<f64>,
}

// Composite signal from multiple indicators
pub fn generate_signal(snapshot: &IndicatorSnapshot) -> SignalResult;
```

This bridges the gap between "library for quants" and "library for traders." Most Indian retail traders on Zerodha want signals, not raw indicator values.

### 4d. Configurable Precision (LOW-MEDIUM IMPACT)

Both yata and kand support f32/f64. For batch processing 2,000+ stocks, f32 halves memory and can be faster with SIMD.

**Implementation**: Use a generic parameter `T: Float` trait bound, or simpler: a feature flag that switches the internal type.

### 4e. Serde Serialization (LOW IMPACT, Easy Win)

Add `#[derive(Serialize, Deserialize)]` to all result structs behind a `serde` feature flag. Lets users cache/persist indicator state. Both ta-rs and yata have this.

---

## 5. What Makes TA Libraries Succeed

### Download/star comparison across ecosystems:

| Library | Language | Stars | Downloads | Key Success Factor |
|---------|----------|-------|-----------|-------------------|
| TA-Lib | C/Python | 10K+ | Millions | First mover, 150+ indicators, C performance |
| pandas-ta | Python | 5K+ | 30K/week | Pandas integration, 130+ indicators, easy API |
| ta-rs | Rust | 842 | 214K | Clean trait API, first Rust TA lib |
| trading-signals | JS/TS | 800+ | High | Streaming design, TypeScript types, active maintenance |
| tulip-indicators | C | 800+ | Moderate | Pure C, zero dependencies, embeddable |
| yata | Rust | 390 | 179K | Broad indicator coverage |
| kand | Rust | 539 | 13K | Modern design, multi-language |

### Patterns of success:

1. **Comprehensive indicator count matters** -- TA-Lib and pandas-ta dominate because they have 130-150+ indicators. Developers choose the library that has everything they need so they don't have to mix libraries.

2. **Clean, consistent API** -- ta-rs succeeded despite having fewer indicators because its `Next` trait is elegant. Every indicator works the same way.

3. **Ecosystem integration** -- pandas-ta wins in Python because it extends DataFrame. trading-signals wins in JS because it's TypeScript-first with streaming.

4. **Documentation with real examples** -- Successful libraries have examples showing real trading strategies, not just API reference. Benchmarks against alternatives are table stakes.

5. **Active maintenance** -- ta-rs downloads are declining because it's abandoned. Users don't trust unmaintained financial libraries.

6. **Performance claims with benchmarks** -- Every successful library publishes benchmarks. indica's "2,000 stocks in 6ms" claim is excellent -- lean into this harder.

---

## 6. Real-World Usage Patterns

### How quant/algo traders actually use TA libraries:

**Pattern 1: Screening pipeline** (indica already nails this)
```
Load 2,000 stocks -> Compute all indicators -> Filter by criteria -> Return candidates
```
This is indica's strongest use case. Rayon parallelism is the right approach.

**Pattern 2: Real-time monitoring** (indica cannot do this yet)
```
WebSocket tick arrives -> Update indicator state -> Check signal conditions -> Alert
```
Requires streaming/incremental API. Currently impossible without recomputing everything.

**Pattern 3: Backtesting** (indica partially supports this)
```
For each historical day:
    Compute indicators at that point in time
    Apply strategy rules
    Track positions and P&L
```
Requires either: (a) full series output (all 250 RSI values, not just the latest), or (b) streaming API that can replay history. indica currently returns only the final value.

**Pattern 4: Strategy composition** (indica cannot do this)
```
Buy when: RSI < 30 AND MACD bullish crossover AND price > SMA(200)
Sell when: RSI > 70 OR price < SMA(50)
```
Requires a signal/strategy layer that combines multiple indicators.

---

## 7. Actionable Recommendations (Ranked by Impact)

### Phase 1: Architecture Foundation (Do First)

**1. Add streaming/incremental API via trait system**
- Impact: CRITICAL. Unlocks real-time use cases, backtesting, and makes every future indicator automatically streamable.
- Effort: Medium (refactor internals of each existing indicator to maintain state)
- Keep existing batch functions as wrappers.

**2. Return full series, not just final value**
- Impact: HIGH. Unlocks backtesting and charting.
- Change `rsi(&closes, 14) -> Option<f64>` to also offer `rsi_series(&closes, 14) -> Option<Vec<f64>>`.
- You already have `ema_series` internally -- expose this pattern for all indicators.

### Phase 2: High-Value Indicators (Do Second)

**3. Add Supertrend**
- Impact: HIGH for Indian market positioning. Most requested indicator in Indian trading communities. You already have ATR. Generates buy/sell signals inherently.

**4. Add VWAP**
- Impact: HIGH. #1 institutional indicator. Simple math. Needs session-reset awareness.

**5. Add Stochastic Oscillator**
- Impact: HIGH. Expected by every serious trader. Missing from indica is conspicuous.

**6. Add ADX (+DI/-DI)**
- Impact: HIGH. Pairs with Supertrend for trend strength confirmation. Universal indicator.

**7. Add OBV**
- Impact: MEDIUM-HIGH. Simplest volume indicator. 10 lines of code. No excuse not to have it.

### Phase 3: India-Specific Moat (Do Third -- This Is The Differentiator)

**8. Delivery Volume indicators**
- Impact: VERY HIGH for positioning. No other library in any language does this.
- `delivery_percentage()`, `delivery_trend()`, `delivery_adjusted_obv()`

**9. Circuit limit detection**
- Impact: HIGH for Indian traders. Unique feature.
- `circuit_proximity()`, `circuit_hit_count()`

**10. FII/DII flow indicators**
- Impact: MEDIUM-HIGH. Requires external data but the computation layer is valuable.

### Phase 4: Platform Expansion

**11. WASM compilation**
- Impact: HIGH. Opens browser use case. Feature-gate NAPI-RS and Rayon.
- Publish as separate npm package `@devanshhq/indica-wasm`.

**12. Signal generation layer**
- Impact: HIGH. Goes from "indicator library" to "trading intelligence library."
- Composite signals from multiple indicators with confidence scoring.

### Phase 5: Completeness

**13. Add remaining standard indicators**: Ichimoku Cloud, Parabolic SAR, Williams %R, CCI, MFI, Keltner Channels, DEMA/TEMA, Fibonacci levels, Heikin Ashi
- Impact: MEDIUM. Completeness matters for adoption.

**14. Candlestick pattern recognition**: Doji, Hammer, Engulfing, Morning Star, etc.
- Impact: MEDIUM. kand has this. Popular with manual traders.

**15. Serde support + configurable precision (f32/f64)**
- Impact: LOW-MEDIUM. Easy wins for ecosystem integration.

---

## 8. Positioning Strategy

### What indica should NOT try to be:
- Another "TA-Lib but in Rust" (kand is already doing this)
- The library with the most indicators (kand has 60+, you can't catch up)

### What indica SHOULD be:

**"The fast, opinionated TA library built for Indian stock markets. Screening to signals."**

1. **Speed story**: 2,000 stocks in 6ms. Keep this front and center. Benchmark against ta-rs, kand, yata.
2. **India-first**: Delivery volume, circuit limits, FII/DII -- features nobody else has.
3. **Signals, not just numbers**: Go beyond raw indicator values to actionable buy/sell signals.
4. **Dual-mode**: Batch screening (Rayon) + real-time streaming. Cover both use cases.
5. **Multi-platform**: Rust + Node.js (NAPI-RS) + Browser (WASM). Run anywhere.
6. **Metis integration story**: Dog-food it in a real product. Show real-world usage.

### Tagline options:
- "Technical analysis for Indian markets. Fast."
- "From screening to signals. In Rust."
- "2,000 stocks. 6ms. Built for Indian traders."

---

## Sources

- [ta-rs GitHub](https://github.com/greyblake/ta-rs) -- 842 stars, streaming Next trait, last updated 2021
- [kand GitHub](https://github.com/kand-ta/kand) -- 539 stars, 60+ indicators, O(1) incremental, WASM + Python
- [yata GitHub](https://github.com/amv-dev/yata) -- 390 stars, 30+ indicators, configurable types
- [Zerodha Varsity: ADX, Supertrend, VWAP](https://zerodha.com/varsity/chapter/supplementary-notes-1/)
- [Zerodha: Delivery Volume on Kite](https://support.zerodha.com/category/trading-and-markets/general-kite/kite-mw/articles/delivery-volume-percentage-on-kite)
- [NSE: Circuit Breakers](https://www.nseindia.com/products-services/equity-market-circuit-breakers)
- [NSE: FII/DII Data](https://www.nseindia.com/reports/fii-dii)
- [TA-Lib](https://ta-lib.org/) -- 150+ indicators, C performance baseline
- [pandas-ta](https://www.pandas-ta.dev/) -- 130+ indicators, Python ecosystem leader
- [TradingView Community Scripts](https://www.tradingview.com/scripts/) -- 150K+ scripts, popularity signals
- [Research 360: Delivery Screener](https://www.research360.in/screeners/volume-and-delivery)
- [Best Indicators for Indian Swing Traders](https://www.gwcindia.in/blog/best-indicator-combinations-for-swing-traders-in-india/)
- [100 Best Trading Indicators 2026](https://www.quantifiedstrategies.com/trading-indicators/)
- [Supertrend Indicator Formula](https://www.tradingfuel.com/supertrend-indicator-formula-and-calculation/)
- [trading-signals npm](https://github.com/bennycode/trading-signals) -- streaming TA for TypeScript
- [Rust and WebAssembly](https://rustwasm.github.io/book/)
