use crate::utils::round;

/// Classic Pivot Points result.
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

/// Classic Pivot Points from a single candle (typically previous day).
pub fn pivot_points(high: f64, low: f64, close: f64) -> PivotPointsResult {
    let pivot = (high + low + close) / 3.0;
    let r1 = 2.0 * pivot - low;
    let s1 = 2.0 * pivot - high;
    let r2 = pivot + (high - low);
    let s2 = pivot - (high - low);
    let r3 = high + 2.0 * (pivot - low);
    let s3 = low - 2.0 * (high - pivot);

    PivotPointsResult {
        r3: round(r3, 2),
        r2: round(r2, 2),
        r1: round(r1, 2),
        pivot: round(pivot, 2),
        s1: round(s1, 2),
        s2: round(s2, 2),
        s3: round(s3, 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_basic() {
        let result = pivot_points(110.0, 90.0, 100.0);
        assert_eq!(result.pivot, 100.0);
        assert!(result.r1 > result.pivot);
        assert!(result.r2 > result.r1);
        assert!(result.r3 > result.r2);
        assert!(result.s1 < result.pivot);
        assert!(result.s2 < result.s1);
        assert!(result.s3 < result.s2);
    }

    #[test]
    fn pivot_symmetry() {
        // When close == midpoint of range, pivot = close
        let result = pivot_points(110.0, 90.0, 100.0);
        assert_eq!(result.pivot, 100.0);
    }
}
