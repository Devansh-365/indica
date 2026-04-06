use crate::core::utils::round;

#[derive(Debug, Clone)]
pub struct PivotPointsResult {
    pub r3: f64,
    pub r2: f64,
    pub r1: f64,
    pub pivot: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
}

/// Classic Pivot Points from a single candle.
#[must_use]
pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
    let pivot = (high + low + close) / 3.0;
    PivotPointsResult {
        r3: round(high + 2.0 * (pivot - low), 2),
        r2: round(pivot + (high - low), 2),
        r1: round(2.0 * pivot - low, 2),
        pivot: round(pivot, 2),
        s1: round(2.0 * pivot - high, 2),
        s2: round(pivot - (high - low), 2),
        s3: round(low - 2.0 * (high - pivot), 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_basic() {
        let r = pivot_points(110.0, 90.0, 100.0);
        assert_eq!(r.pivot, 100.0);
        assert!(r.r1 > r.pivot);
        assert!(r.s1 < r.pivot);
    }
}
