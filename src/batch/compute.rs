use rayon::prelude::*;

use crate::indicators::india::delivery::{DeliveryTrend, delivery_pct, delivery_trend};
use crate::indicators::momentum::macd::{MacdResult, macd};
use crate::indicators::momentum::rsi::rsi;
use crate::indicators::momentum::stochastic::{StochasticResult, stochastic};
use crate::indicators::trend::adx::adx;
use crate::indicators::trend::ema::ema;
use crate::indicators::trend::sma::sma;
use crate::indicators::trend::supertrend::{SupertrendResult, supertrend};
use crate::indicators::volatility::atr::atr;
use crate::indicators::volatility::bollinger::{BollingerBandsResult, bollinger_bands};
use crate::indicators::volume::obv::obv;
use crate::indicators::volume::volume_trend::volume_trend;
use crate::indicators::volume::vwap::vwap;

/// Input data for one stock.
#[derive(Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub opens: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<f64>,
    /// Delivery volumes (NSE/BSE specific). Optional.
    pub delivery_volumes: Option<Vec<f64>>,
}

/// Complete indicator snapshot for a single stock.
#[derive(Debug, Clone)]
pub struct IndicatorSnapshot {
    pub symbol: String,
    pub sma_20: Option<f64>,
    pub ema_20: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd_result: Option<MacdResult>,
    pub bollinger: Option<BollingerBandsResult>,
    pub atr_14: Option<f64>,
    pub supertrend: Option<SupertrendResult>,
    pub stochastic: Option<StochasticResult>,
    pub adx_14: Option<f64>,
    pub obv: Option<f64>,
    pub vwap: Option<f64>,
    pub volume_trend: String,
    pub delivery_trend: Option<DeliveryTrend>,
}

/// Compute all indicators for a single stock.
pub fn compute_snapshot(data: &StockData) -> IndicatorSnapshot {
    let delivery_trend_val = data.delivery_volumes.as_ref().and_then(|dv| {
        // Compute delivery percentages from delivery_volume / total_volume
        let delivery_pcts: Vec<f64> = dv
            .iter()
            .zip(data.volumes.iter())
            .map(|(&d, &t)| delivery_pct(d, t))
            .collect();
        delivery_trend(&delivery_pcts, &data.closes, 5, 20)
    });

    IndicatorSnapshot {
        symbol: data.symbol.clone(),
        sma_20: sma(&data.closes, 20),
        ema_20: ema(&data.closes, 20),
        rsi_14: rsi(&data.closes, 14),
        macd_result: macd(&data.closes, 12, 26, 9),
        bollinger: bollinger_bands(&data.closes, 20, 2.0),
        atr_14: atr(&data.highs, &data.lows, &data.closes, 14),
        supertrend: supertrend(&data.highs, &data.lows, &data.closes, 10, 3.0),
        stochastic: stochastic(&data.highs, &data.lows, &data.closes, 14, 3),
        adx_14: adx(&data.highs, &data.lows, &data.closes, 14),
        obv: obv(&data.closes, &data.volumes),
        vwap: vwap(&data.highs, &data.lows, &data.closes, &data.volumes),
        volume_trend: volume_trend(&data.volumes).to_string(),
        delivery_trend: delivery_trend_val,
    }
}

/// Batch compute indicators for multiple stocks (sequential).
pub fn batch_compute(stocks: &[StockData]) -> Vec<IndicatorSnapshot> {
    stocks.iter().map(compute_snapshot).collect()
}

/// Batch compute indicators for multiple stocks (parallel via rayon).
pub fn batch_compute_parallel(stocks: &[StockData]) -> Vec<IndicatorSnapshot> {
    stocks.par_iter().map(compute_snapshot).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stock(symbol: &str, len: usize) -> StockData {
        let closes: Vec<f64> = (0..len).map(|i| 100.0 + (i as f64 * 0.5)).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 2.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 2.0).collect();
        let opens: Vec<f64> = closes.iter().map(|c| c - 0.5).collect();
        let volumes: Vec<f64> = vec![1_000_000.0; len];
        StockData {
            symbol: symbol.to_string(),
            opens,
            highs,
            lows,
            closes,
            volumes,
            delivery_volumes: None,
        }
    }

    #[test]
    fn single_stock_snapshot() {
        let stock = make_stock("RELIANCE", 60);
        let snap = compute_snapshot(&stock);
        assert_eq!(snap.symbol, "RELIANCE");
        assert!(snap.sma_20.is_some());
        assert!(snap.ema_20.is_some());
        assert!(snap.rsi_14.is_some());
        assert!(snap.macd_result.is_some());
        assert!(snap.bollinger.is_some());
        assert!(snap.atr_14.is_some());
        assert!(snap.supertrend.is_some());
        assert!(snap.stochastic.is_some());
        assert!(snap.obv.is_some());
        assert!(snap.vwap.is_some());
        assert_ne!(snap.volume_trend, "insufficient data");
    }

    #[test]
    fn batch_compute_100_stocks() {
        let stocks: Vec<StockData> = (0..100)
            .map(|i| make_stock(&format!("STOCK{i}"), 60))
            .collect();
        let results = batch_compute(&stocks);
        assert_eq!(results.len(), 100);
        for snap in &results {
            assert!(snap.rsi_14.is_some());
        }
    }

    #[test]
    fn batch_compute_parallel_100_stocks() {
        let stocks: Vec<StockData> = (0..100)
            .map(|i| make_stock(&format!("STOCK{i}"), 60))
            .collect();
        let results = batch_compute_parallel(&stocks);
        assert_eq!(results.len(), 100);
        for snap in &results {
            assert!(snap.rsi_14.is_some());
        }
    }

    #[test]
    fn batch_with_delivery_data() {
        let mut stock = make_stock("HDFC", 60);
        stock.delivery_volumes = Some(vec![600_000.0; 60]);
        let snap = compute_snapshot(&stock);
        assert!(snap.delivery_trend.is_some());
    }

    #[test]
    fn batch_empty() {
        let results = batch_compute(&[]);
        assert!(results.is_empty());
    }
}
