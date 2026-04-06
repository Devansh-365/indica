/// Classify volume trend by comparing recent 5-day avg to 20-day avg.
/// Returns a static string: "surging", "increasing", "stable", "declining", "drying up",
/// or "insufficient data".
pub fn volume_trend(volumes: &[f64]) -> &'static str {
    if volumes.len() < 20 {
        return "insufficient data";
    }

    let avg5: f64 = volumes[volumes.len() - 5..].iter().sum::<f64>() / 5.0;
    let avg20: f64 = volumes[volumes.len() - 20..].iter().sum::<f64>() / 20.0;

    if avg20 == 0.0 {
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
    fn volume_stable() {
        let volumes = vec![1000.0; 25];
        assert_eq!(volume_trend(&volumes), "stable");
    }

    #[test]
    fn volume_surging() {
        let mut volumes = vec![100.0; 20];
        // Last 5 are much higher
        for _ in 0..5 {
            volumes.push(500.0);
        }
        assert_eq!(volume_trend(&volumes), "surging");
    }

    #[test]
    fn volume_drying_up() {
        let mut volumes = vec![1000.0; 20];
        for _ in 0..5 {
            volumes.push(10.0);
        }
        assert_eq!(volume_trend(&volumes), "drying up");
    }

    #[test]
    fn volume_insufficient() {
        assert_eq!(volume_trend(&[100.0; 10]), "insufficient data");
    }
}
