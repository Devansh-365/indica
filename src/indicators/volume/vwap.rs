use crate::core::utils::round;

/// Volume Weighted Average Price.
/// VWAP = sum(typical_price * volume) / sum(volume)
/// where typical_price = (high + low + close) / 3
#[must_use]
pub fn vwap(highs: &[f64], lows: &[f64], closes: &[f64], volumes: &[f64]) -> Option<f64> {
    let len = closes.len();
    if len == 0 || highs.len() < len || lows.len() < len || volumes.len() < len {
        return None;
    }

    let mut cum_tp_vol = 0.0;
    let mut cum_vol = 0.0;

    for i in 0..len {
        let typical_price = (highs[i] + lows[i] + closes[i]) / 3.0;
        cum_tp_vol += typical_price * volumes[i];
        cum_vol += volumes[i];
    }

    if cum_vol < f64::EPSILON {
        return None;
    }

    Some(round(cum_tp_vol / cum_vol, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vwap_basic() {
        let highs = vec![12.0, 13.0, 14.0];
        let lows = vec![10.0, 11.0, 12.0];
        let closes = vec![11.0, 12.0, 13.0];
        let volumes = vec![1000.0, 2000.0, 3000.0];
        let result = vwap(&highs, &lows, &closes, &volumes).unwrap();
        assert!(result > 11.0 && result < 14.0);
    }

    #[test]
    fn vwap_zero_volume() {
        assert!(vwap(&[10.0], &[10.0], &[10.0], &[0.0]).is_none());
    }

    #[test]
    fn vwap_empty() {
        let empty: Vec<f64> = vec![];
        assert!(vwap(&empty, &empty, &empty, &empty).is_none());
    }
}
