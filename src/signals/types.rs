/// Signal strength classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalStrength {
    StrongBuy,
    Buy,
    Neutral,
    Sell,
    StrongSell,
}

impl SignalStrength {
    /// Numeric score: StrongBuy = 2, Buy = 1, Neutral = 0, Sell = -1, StrongSell = -2.
    pub fn score(self) -> f64 {
        match self {
            Self::StrongBuy => 2.0,
            Self::Buy => 1.0,
            Self::Neutral => 0.0,
            Self::Sell => -1.0,
            Self::StrongSell => -2.0,
        }
    }
}

impl std::fmt::Display for SignalStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrongBuy => write!(f, "strong_buy"),
            Self::Buy => write!(f, "buy"),
            Self::Neutral => write!(f, "neutral"),
            Self::Sell => write!(f, "sell"),
            Self::StrongSell => write!(f, "strong_sell"),
        }
    }
}

/// Composite signal after aggregating all rule votes.
#[derive(Debug, Clone)]
pub struct Signal {
    pub strength: SignalStrength,
    pub confidence: f64,
    pub reasons: Vec<String>,
}

/// A single vote from one signal rule.
#[derive(Debug, Clone)]
pub struct SignalVote {
    pub strength: SignalStrength,
    pub weight: f64,
    pub reason: String,
}
