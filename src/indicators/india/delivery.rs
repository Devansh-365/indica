use crate::core::utils::round;

/// Delivery trend classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryTrend {
    StrongAccumulation, // High delivery % + price up
    Accumulation,       // Above avg delivery % + price up
    Distribution,       // Above avg delivery % + price down
    StrongDistribution, // High delivery % + price down
    Neutral,
}

impl std::fmt::Display for DeliveryTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrongAccumulation => write!(f, "strong_accumulation"),
            Self::Accumulation => write!(f, "accumulation"),
            Self::Distribution => write!(f, "distribution"),
            Self::StrongDistribution => write!(f, "strong_distribution"),
            Self::Neutral => write!(f, "neutral"),
        }
    }
}

/// Delivery percentage: delivery_volume / total_volume * 100.
/// Unique to NSE/BSE — not available in US markets.
#[must_use]
pub fn delivery_pct(delivery_volume: f64, total_volume: f64) -> f64 {
    if total_volume < f64::EPSILON {
        return 0.0;
    }
    round(delivery_volume / total_volume * 100.0, 2)
}

/// Delivery trend analysis: compares recent delivery % with price movement.
/// High delivery + price up = genuine buying (accumulation).
/// High delivery + price down = genuine selling (distribution).
#[must_use]
pub fn delivery_trend(
    delivery_pcts: &[f64],
    closes: &[f64],
    short_period: usize,
    long_period: usize,
) -> Option<DeliveryTrend> {
    if delivery_pcts.len() < long_period
        || closes.len() < long_period
        || short_period == 0
        || long_period == 0
        || short_period >= long_period
    {
        return None;
    }

    let len = delivery_pcts.len();
    let short_avg: f64 =
        delivery_pcts[len - short_period..].iter().sum::<f64>() / short_period as f64;
    let long_avg: f64 = delivery_pcts[len - long_period..].iter().sum::<f64>() / long_period as f64;

    let delivery_ratio = if long_avg < f64::EPSILON {
        1.0
    } else {
        short_avg / long_avg
    };

    let clen = closes.len();
    let price_change = if closes[clen - short_period] > f64::EPSILON {
        (closes[clen - 1] - closes[clen - short_period]) / closes[clen - short_period]
    } else {
        0.0
    };

    Some(match (delivery_ratio, price_change) {
        (r, p) if r > 1.3 && p > 0.02 => DeliveryTrend::StrongAccumulation,
        (r, p) if r > 1.1 && p > 0.0 => DeliveryTrend::Accumulation,
        (r, p) if r > 1.3 && p < -0.02 => DeliveryTrend::StrongDistribution,
        (r, p) if r > 1.1 && p < 0.0 => DeliveryTrend::Distribution,
        _ => DeliveryTrend::Neutral,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivery_pct_basic() {
        assert_eq!(delivery_pct(500_000.0, 1_000_000.0), 50.0);
        assert_eq!(delivery_pct(0.0, 0.0), 0.0);
    }

    #[test]
    fn accumulation() {
        // High delivery + price up
        let delivery_pcts = vec![40.0, 42.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 72.0, 75.0];
        let closes = vec![
            100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 110.0,
        ];
        let result = delivery_trend(&delivery_pcts, &closes, 3, 10).unwrap();
        assert!(
            result == DeliveryTrend::Accumulation || result == DeliveryTrend::StrongAccumulation
        );
    }

    #[test]
    fn distribution() {
        // High delivery + price down
        let delivery_pcts = vec![40.0, 42.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 72.0, 75.0];
        let closes = vec![
            110.0, 109.0, 108.0, 107.0, 106.0, 105.0, 104.0, 103.0, 102.0, 100.0,
        ];
        let result = delivery_trend(&delivery_pcts, &closes, 3, 10).unwrap();
        assert!(
            result == DeliveryTrend::Distribution || result == DeliveryTrend::StrongDistribution
        );
    }
}
