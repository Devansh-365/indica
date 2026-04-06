/// On-Balance Volume.
/// Running total: +volume on up days, -volume on down days.
#[must_use]
pub fn obv(closes: &[f64], volumes: &[f64]) -> Option<f64> {
    let len = closes.len();
    if len < 2 || volumes.len() < len {
        return None;
    }
    let mut value = 0.0;
    for i in 1..len {
        if closes[i] > closes[i - 1] {
            value += volumes[i];
        } else if closes[i] < closes[i - 1] {
            value -= volumes[i];
        }
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obv_up_trend() {
        let closes = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let volumes = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        let result = obv(&closes, &volumes).unwrap();
        assert_eq!(result, 1400.0); // 200+300+400+500
    }

    #[test]
    fn obv_mixed() {
        let closes = vec![10.0, 12.0, 11.0, 13.0];
        let volumes = vec![100.0, 200.0, 300.0, 400.0];
        let result = obv(&closes, &volumes).unwrap();
        assert_eq!(result, 300.0); // +200 -300 +400
    }

    #[test]
    fn obv_insufficient() {
        assert!(obv(&[10.0], &[100.0]).is_none());
    }
}
