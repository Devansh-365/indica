use crate::utils::round;

/// Bollinger Bands result.
#[derive(Debug, Clone)]
pub struct BollingerBandsResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub percent_b: f64,
}

/// Bollinger Bands (default: period=20, std_dev_multiplier=2.0).
/// Returns `None` if insufficient data.
pub fn bollinger_bands(
    closes: &[f64],
    period: usize,
    std_dev_multiplier: f64,
) -> Option<BollingerBandsResult> {
    if closes.len() < period || period == 0 {
        return None;
    }

    let slice = &closes[closes.len() - period..];
    let middle: f64 = slice.iter().sum::<f64>() / period as f64;

    let variance: f64 = slice.iter().map(|&v| (v - middle).powi(2)).sum::<f64>() / period as f64;
    let std_dev = variance.sqrt();

    let upper = middle + std_dev_multiplier * std_dev;
    let lower = middle - std_dev_multiplier * std_dev;

    let current_price = *closes.last().unwrap();
    let percent_b = if (upper - lower).abs() < f64::EPSILON {
        0.5
    } else {
        (current_price - lower) / (upper - lower)
    };

    Some(BollingerBandsResult {
        upper: round(upper, 2),
        middle: round(middle, 2),
        lower: round(lower, 2),
        percent_b: round(percent_b, 2),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bb_basic() {
        let closes: Vec<f64> = (1..=20).map(|i| i as f64).collect();
        let result = bollinger_bands(&closes, 20, 2.0).unwrap();
        assert!(result.upper > result.middle);
        assert!(result.middle > result.lower);
        assert_eq!(result.middle, 10.5); // avg of 1..=20
    }

    #[test]
    fn bb_percent_b() {
        // Price at middle should give ~0.5
        let closes = vec![10.0; 20];
        let result = bollinger_bands(&closes, 20, 2.0).unwrap();
        assert_eq!(result.percent_b, 0.5); // all same = flat bands
    }

    #[test]
    fn bb_insufficient_data() {
        assert!(bollinger_bands(&[1.0; 5], 20, 2.0).is_none());
    }
}
