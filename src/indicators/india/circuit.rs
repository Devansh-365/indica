use crate::core::utils::round;

/// Circuit limit bands for Indian stocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitLimit {
    Percent2,
    Percent5,
    Percent10,
    Percent20,
}

impl CircuitLimit {
    fn multiplier(&self) -> f64 {
        match self {
            Self::Percent2 => 0.02,
            Self::Percent5 => 0.05,
            Self::Percent10 => 0.10,
            Self::Percent20 => 0.20,
        }
    }
}

/// Circuit proximity status.
#[derive(Debug, Clone)]
pub struct CircuitStatus {
    pub upper_limit: f64,
    pub lower_limit: f64,
    pub upper_distance_pct: f64, // How far from upper circuit (0 = at limit)
    pub lower_distance_pct: f64, // How far from lower circuit
    pub near_upper: bool,        // Within 1% of upper circuit
    pub near_lower: bool,        // Within 1% of lower circuit
}

/// Check how close a stock is to its circuit limits.
/// Indian stocks have daily price movement limits (2/5/10/20%).
#[must_use]
pub fn circuit_proximity(
    current_price: f64,
    prev_close: f64,
    limit: CircuitLimit,
) -> CircuitStatus {
    let mult = limit.multiplier();
    let upper_limit = round(prev_close * (1.0 + mult), 2);
    let lower_limit = round(prev_close * (1.0 - mult), 2);

    let upper_distance_pct = if current_price < upper_limit {
        round((upper_limit - current_price) / upper_limit * 100.0, 2)
    } else {
        0.0
    };

    let lower_distance_pct = if current_price > lower_limit {
        round((current_price - lower_limit) / lower_limit * 100.0, 2)
    } else {
        0.0
    };

    CircuitStatus {
        upper_limit,
        lower_limit,
        upper_distance_pct,
        lower_distance_pct,
        near_upper: upper_distance_pct < 1.0,
        near_lower: lower_distance_pct < 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_basic() {
        let status = circuit_proximity(108.0, 100.0, CircuitLimit::Percent10);
        assert_eq!(status.upper_limit, 110.0);
        assert_eq!(status.lower_limit, 90.0);
        assert!(!status.near_upper);
        assert!(!status.near_lower);
    }

    #[test]
    fn circuit_near_upper() {
        let status = circuit_proximity(109.5, 100.0, CircuitLimit::Percent10);
        assert!(status.near_upper);
        assert!(!status.near_lower);
    }

    #[test]
    fn circuit_at_lower() {
        let status = circuit_proximity(90.0, 100.0, CircuitLimit::Percent10);
        assert_eq!(status.lower_distance_pct, 0.0);
        assert!(status.near_lower);
    }

    #[test]
    fn circuit_2_pct() {
        let status = circuit_proximity(100.0, 100.0, CircuitLimit::Percent2);
        assert_eq!(status.upper_limit, 102.0);
        assert_eq!(status.lower_limit, 98.0);
    }
}
