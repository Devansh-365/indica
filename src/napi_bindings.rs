use crate::batch::{StockData, batch_compute_parallel};
use napi_derive::napi;

// ── Simple indicator functions (for single stock use) ──

#[napi]
pub fn calc_sma(values: Vec<f64>, period: u32) -> Option<f64> {
    crate::sma(&values, period as usize)
}

#[napi]
pub fn calc_ema(values: Vec<f64>, period: u32) -> Option<f64> {
    crate::ema(&values, period as usize)
}

#[napi]
pub fn calc_rsi(closes: Vec<f64>, period: u32) -> Option<f64> {
    crate::rsi(&closes, period as usize)
}

#[napi]
pub fn calc_atr(highs: Vec<f64>, lows: Vec<f64>, closes: Vec<f64>, period: u32) -> Option<f64> {
    crate::atr(&highs, &lows, &closes, period as usize)
}

#[napi]
pub fn calc_volume_trend(volumes: Vec<f64>) -> String {
    crate::volume_trend(&volumes).to_string()
}

#[napi]
pub fn calc_relative_strength(stock: Vec<f64>, benchmark: Vec<f64>, period: u32) -> Option<f64> {
    crate::relative_strength(&stock, &benchmark, period as usize)
}

// ── Struct results (returned as JS objects) ──

#[napi(object)]
pub struct JsMacdResult {
    pub value: f64,
    pub signal: f64,
    pub histogram: f64,
    pub crossover: String,
}

impl From<crate::MacdResult> for JsMacdResult {
    fn from(r: crate::MacdResult) -> Self {
        Self {
            value: r.value,
            signal: r.signal,
            histogram: r.histogram,
            crossover: r.crossover.to_string(),
        }
    }
}

#[napi]
pub fn calc_macd(closes: Vec<f64>, fast: u32, slow: u32, signal: u32) -> Option<JsMacdResult> {
    crate::macd(&closes, fast as usize, slow as usize, signal as usize).map(Into::into)
}

#[napi(object)]
pub struct JsBollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub percent_b: f64,
}

impl From<crate::BollingerBandsResult> for JsBollingerBands {
    fn from(r: crate::BollingerBandsResult) -> Self {
        Self {
            upper: r.upper,
            middle: r.middle,
            lower: r.lower,
            percent_b: r.percent_b,
        }
    }
}

#[napi]
pub fn calc_bollinger_bands(
    closes: Vec<f64>,
    period: u32,
    std_dev: f64,
) -> Option<JsBollingerBands> {
    crate::bollinger_bands(&closes, period as usize, std_dev).map(Into::into)
}

#[napi(object)]
pub struct JsPivotPoints {
    pub r3: f64,
    pub r2: f64,
    pub r1: f64,
    pub pivot: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
}

impl From<crate::PivotPointsResult> for JsPivotPoints {
    fn from(r: crate::PivotPointsResult) -> Self {
        Self {
            r3: r.r3,
            r2: r.r2,
            r1: r.r1,
            pivot: r.pivot,
            s1: r.s1,
            s2: r.s2,
            s3: r.s3,
        }
    }
}

#[napi]
pub fn calc_pivot_points(high: f64, low: f64, close: f64) -> JsPivotPoints {
    crate::pivot_points(high, low, close).into()
}

// ── Batch processing (the main payoff) ──

#[napi(object)]
pub struct JsStockData {
    pub symbol: String,
    pub closes: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub volumes: Vec<f64>,
}

#[napi(object)]
pub struct JsIndicatorSnapshot {
    pub symbol: String,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub sma_200: Option<f64>,
    pub ema_20: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd: Option<JsMacdResult>,
    pub bollinger: Option<JsBollingerBands>,
    pub atr_14: Option<f64>,
    pub volume_trend: String,
}

#[napi]
pub fn batch_compute_indicators(stocks: Vec<JsStockData>) -> Vec<JsIndicatorSnapshot> {
    let stock_data: Vec<StockData> = stocks
        .into_iter()
        .map(|s| StockData {
            symbol: s.symbol,
            closes: s.closes,
            highs: s.highs,
            lows: s.lows,
            volumes: s.volumes,
        })
        .collect();

    batch_compute_parallel(&stock_data)
        .into_iter()
        .map(|snap| JsIndicatorSnapshot {
            symbol: snap.symbol,
            sma_20: snap.sma_20,
            sma_50: snap.sma_50,
            sma_200: snap.sma_200,
            ema_20: snap.ema_20,
            rsi_14: snap.rsi_14,
            macd: snap.macd_result.map(Into::into),
            bollinger: snap.bollinger.map(Into::into),
            atr_14: snap.atr_14,
            volume_trend: snap.volume_trend,
        })
        .collect()
}
