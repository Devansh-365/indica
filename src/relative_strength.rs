use crate::utils::round;

/// Relative Strength vs a benchmark (e.g., Nifty 50).
/// RS = 1 + (stock_return - benchmark_return).
/// Returns >1 if outperforming, <1 if underperforming.
/// Returns `None` if insufficient data.
pub fn relative_strength(
    stock_closes: &[f64],
    benchmark_closes: &[f64],
    period: usize,
) -> Option<f64> {
    if stock_closes.len() < period || benchmark_closes.len() < period || period == 0 {
        return None;
    }

    let stock_start = stock_closes[stock_closes.len() - period];
    let stock_end = *stock_closes.last().unwrap();
    let bench_start = benchmark_closes[benchmark_closes.len() - period];
    let bench_end = *benchmark_closes.last().unwrap();

    if stock_start == 0.0 || bench_start == 0.0 {
        return None;
    }

    let stock_return = (stock_end - stock_start) / stock_start;
    let bench_return = (bench_end - bench_start) / bench_start;

    if bench_return == 0.0 {
        return Some(if stock_return > 0.0 { 2.0 } else { 0.0 });
    }

    Some(round(1.0 + (stock_return - bench_return), 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_outperforming() {
        let stock = vec![100.0, 110.0, 120.0]; // +20%
        let bench = vec![100.0, 105.0, 110.0]; // +10%
        let result = relative_strength(&stock, &bench, 3).unwrap();
        assert!(result > 1.0); // Stock beat benchmark
    }

    #[test]
    fn rs_underperforming() {
        let stock = vec![100.0, 95.0, 90.0];  // -10%
        let bench = vec![100.0, 105.0, 110.0]; // +10%
        let result = relative_strength(&stock, &bench, 3).unwrap();
        assert!(result < 1.0); // Stock lagged benchmark
    }

    #[test]
    fn rs_equal_performance() {
        let stock = vec![100.0, 110.0]; // +10%
        let bench = vec![100.0, 110.0]; // +10%
        let result = relative_strength(&stock, &bench, 2).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn rs_insufficient_data() {
        assert!(relative_strength(&[100.0], &[100.0], 5).is_none());
    }
}
