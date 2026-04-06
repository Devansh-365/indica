use crate::indicators::trend::supertrend::SupertrendDirection;

use super::compute::{IndicatorSnapshot, StockData, compute_snapshot};

/// Filter criteria for stock screening.
#[derive(Debug, Clone)]
pub enum ScreenFilter {
    RsiBelow(f64),
    RsiAbove(f64),
    SupertrendUp,
    SupertrendDown,
    AdxAbove(f64),
    VolumeAbove(f64),
}

/// A stock that passed all screening filters.
#[derive(Debug, Clone)]
pub struct ScreenResult {
    pub symbol: String,
    pub snapshot: IndicatorSnapshot,
}

fn matches_filter(snap: &IndicatorSnapshot, filter: &ScreenFilter, data: &StockData) -> bool {
    match filter {
        ScreenFilter::RsiBelow(threshold) => snap.rsi_14.is_some_and(|rsi| rsi < *threshold),
        ScreenFilter::RsiAbove(threshold) => snap.rsi_14.is_some_and(|rsi| rsi > *threshold),
        ScreenFilter::SupertrendUp => snap
            .supertrend
            .as_ref()
            .is_some_and(|st| st.direction == SupertrendDirection::Up),
        ScreenFilter::SupertrendDown => snap
            .supertrend
            .as_ref()
            .is_some_and(|st| st.direction == SupertrendDirection::Down),
        ScreenFilter::AdxAbove(threshold) => {
            snap.adx_14.is_some_and(|adx_val| adx_val > *threshold)
        }
        ScreenFilter::VolumeAbove(threshold) => {
            data.volumes.last().is_some_and(|&vol| vol > *threshold)
        }
    }
}

/// Screen stocks: compute indicators and return those matching ALL filters.
pub fn screen(stocks: &[StockData], filters: &[ScreenFilter]) -> Vec<ScreenResult> {
    stocks
        .iter()
        .filter_map(|data| {
            let snapshot = compute_snapshot(data);
            let passes = filters.iter().all(|f| matches_filter(&snapshot, f, data));
            if passes {
                Some(ScreenResult {
                    symbol: data.symbol.clone(),
                    snapshot,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Screen with pre-computed snapshots (avoids recomputing).
pub fn screen_precomputed(
    snapshots: &[(StockData, IndicatorSnapshot)],
    filters: &[ScreenFilter],
) -> Vec<ScreenResult> {
    snapshots
        .iter()
        .filter_map(|(data, snapshot)| {
            let passes = filters.iter().all(|f| matches_filter(snapshot, f, data));
            if passes {
                Some(ScreenResult {
                    symbol: data.symbol.clone(),
                    snapshot: snapshot.clone(),
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_uptrend_stock(symbol: &str) -> StockData {
        let len = 60;
        let closes: Vec<f64> = (0..len).map(|i| 100.0 + i as f64 * 2.0).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 2.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 2.0).collect();
        let opens: Vec<f64> = closes.iter().map(|c| c - 0.5).collect();
        let volumes: Vec<f64> = vec![2_000_000.0; len];
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

    fn make_downtrend_stock(symbol: &str) -> StockData {
        let len = 60;
        let closes: Vec<f64> = (0..len).map(|i| 200.0 - i as f64 * 2.0).collect();
        let highs: Vec<f64> = closes.iter().map(|c| c + 2.0).collect();
        let lows: Vec<f64> = closes.iter().map(|c| c - 2.0).collect();
        let opens: Vec<f64> = closes.iter().map(|c| c + 0.5).collect();
        let volumes: Vec<f64> = vec![500_000.0; len];
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
    fn screen_supertrend_up() {
        let stocks = vec![
            make_uptrend_stock("RELIANCE"),
            make_downtrend_stock("YESBANK"),
        ];
        let results = screen(&stocks, &[ScreenFilter::SupertrendUp]);
        assert!(results.iter().any(|r| r.symbol == "RELIANCE"));
        assert!(!results.iter().any(|r| r.symbol == "YESBANK"));
    }

    #[test]
    fn screen_volume_above() {
        let stocks = vec![
            make_uptrend_stock("RELIANCE"),  // 2M volume
            make_downtrend_stock("YESBANK"), // 500K volume
        ];
        let results = screen(&stocks, &[ScreenFilter::VolumeAbove(1_000_000.0)]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "RELIANCE");
    }

    #[test]
    fn screen_multiple_filters() {
        let stocks = vec![
            make_uptrend_stock("RELIANCE"),
            make_downtrend_stock("YESBANK"),
        ];
        let results = screen(
            &stocks,
            &[
                ScreenFilter::SupertrendUp,
                ScreenFilter::VolumeAbove(1_000_000.0),
            ],
        );
        // Only RELIANCE should match both
        assert!(results.iter().all(|r| r.symbol == "RELIANCE"));
    }

    #[test]
    fn screen_empty_filters_returns_all() {
        let stocks = vec![make_uptrend_stock("A"), make_downtrend_stock("B")];
        let results = screen(&stocks, &[]);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn screen_no_matches() {
        let stocks = vec![make_uptrend_stock("RELIANCE")];
        let results = screen(&stocks, &[ScreenFilter::SupertrendDown]);
        assert!(results.is_empty());
    }
}
