use crate::*;
use rayon::prelude::*;

/// Input data for a single stock.
#[derive(Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub closes: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub volumes: Vec<f64>,
}

/// All indicators computed for a single stock.
#[derive(Debug, Clone)]
pub struct IndicatorSnapshot {
    pub symbol: String,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub sma_200: Option<f64>,
    pub ema_20: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd_result: Option<MacdResult>,
    pub bollinger: Option<BollingerBandsResult>,
    pub atr_14: Option<f64>,
    pub volume_trend: String,
}

/// Compute all indicators for a single stock.
pub fn compute_indicators(stock: &StockData) -> IndicatorSnapshot {
    IndicatorSnapshot {
        symbol: stock.symbol.clone(),
        sma_20: sma(&stock.closes, 20),
        sma_50: sma(&stock.closes, 50),
        sma_200: sma(&stock.closes, 200),
        ema_20: ema(&stock.closes, 20),
        rsi_14: rsi(&stock.closes, 14),
        macd_result: macd(&stock.closes, 12, 26, 9),
        bollinger: bollinger_bands(&stock.closes, 20, 2.0),
        atr_14: atr(&stock.highs, &stock.lows, &stock.closes, 14),
        volume_trend: volume_trend(&stock.volumes).to_string(),
    }
}

/// Compute indicators for multiple stocks sequentially.
pub fn batch_compute(stocks: &[StockData]) -> Vec<IndicatorSnapshot> {
    stocks.iter().map(compute_indicators).collect()
}

/// Compute indicators for multiple stocks in parallel using Rayon.
/// Automatically distributes work across all CPU cores.
pub fn batch_compute_parallel(stocks: &[StockData]) -> Vec<IndicatorSnapshot> {
    stocks.par_iter().map(compute_indicators).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stock(symbol: &str, days: usize) -> StockData {
        let closes: Vec<f64> = (0..days).map(|i| 100.0 + (i as f64 * 0.5)).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 2.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 2.0).collect();
        let volumes: Vec<f64> = vec![1_000_000.0; days];
        StockData {
            symbol: symbol.to_string(),
            closes,
            highs,
            lows,
            volumes,
        }
    }

    #[test]
    fn single_stock() {
        let stock = make_stock("RELIANCE", 250);
        let result = compute_indicators(&stock);
        assert_eq!(result.symbol, "RELIANCE");
        assert!(result.sma_20.is_some());
        assert!(result.sma_50.is_some());
        assert!(result.sma_200.is_some());
        assert!(result.rsi_14.is_some());
        assert!(result.macd_result.is_some());
        assert!(result.bollinger.is_some());
        assert!(result.atr_14.is_some());
        assert_eq!(result.volume_trend, "stable");
    }

    #[test]
    fn batch_multiple() {
        let stocks: Vec<StockData> = ["RELIANCE", "TCS", "INFY", "SBIN", "HDFCBANK"]
            .iter()
            .map(|s| make_stock(s, 250))
            .collect();

        let results = batch_compute(&stocks);
        assert_eq!(results.len(), 5);
        assert_eq!(results[0].symbol, "RELIANCE");
        assert_eq!(results[4].symbol, "HDFCBANK");
    }

    #[test]
    fn batch_2000_stocks() {
        let stocks: Vec<StockData> = (0..2000)
            .map(|i| make_stock(&format!("STOCK{}", i), 250))
            .collect();

        let start = std::time::Instant::now();
        let results = batch_compute(&stocks);
        let elapsed = start.elapsed();

        assert_eq!(results.len(), 2000);
        println!("2000 stocks sequential: {:?}", elapsed);
    }

    #[test]
    fn batch_2000_parallel() {
        let stocks: Vec<StockData> = (0..2000)
            .map(|i| make_stock(&format!("STOCK{}", i), 250))
            .collect();

        let start = std::time::Instant::now();
        let results = batch_compute_parallel(&stocks);
        let elapsed = start.elapsed();

        assert_eq!(results.len(), 2000);
        println!("2000 stocks parallel:   {:?}", elapsed);
    }

    #[test]
    fn insufficient_data_stock() {
        let stock = make_stock("NEWIPO", 10);
        let result = compute_indicators(&stock);
        assert_eq!(result.symbol, "NEWIPO");
        assert!(result.sma_20.is_none());
        assert!(result.rsi_14.is_none());
        assert!(result.macd_result.is_none());
    }
}
