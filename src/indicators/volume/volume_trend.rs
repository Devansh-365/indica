/// Classify volume trend by comparing recent 5-day avg to 20-day avg.
#[must_use]
pub fn volume_trend(volumes: &[f64]) -> &'static str {
    if volumes.len() < 20 {
        return "insufficient data";
    }
    let avg5: f64 = volumes[volumes.len() - 5..].iter().sum::<f64>() / 5.0;
    let avg20: f64 = volumes[volumes.len() - 20..].iter().sum::<f64>() / 20.0;
    if avg20 < f64::EPSILON {
        return "insufficient data";
    }
    let ratio = avg5 / avg20;
    if ratio > 1.5 {
        "surging"
    } else if ratio > 1.1 {
        "increasing"
    } else if ratio > 0.9 {
        "stable"
    } else if ratio > 0.5 {
        "declining"
    } else {
        "drying up"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable() {
        assert_eq!(volume_trend(&vec![1000.0; 25]), "stable");
    }

    #[test]
    fn surging() {
        let mut v = vec![100.0; 20];
        v.extend_from_slice(&[500.0; 5]);
        assert_eq!(volume_trend(&v), "surging");
    }

    #[test]
    fn insufficient() {
        assert_eq!(volume_trend(&[100.0; 10]), "insufficient data");
    }
}
