use crate::indicators::india::delivery::DeliveryTrend;
use crate::indicators::momentum::macd::MacdResult;
use crate::indicators::momentum::stochastic::StochasticResult;
use crate::indicators::trend::supertrend::SupertrendResult;
use crate::indicators::volatility::bollinger::BollingerBandsResult;

use super::types::{Signal, SignalStrength, SignalVote};

/// Snapshot of all computed indicator values for a single stock.
#[derive(Debug, Clone, Default)]
pub struct IndicatorValues {
    pub rsi: Option<f64>,
    pub macd: Option<MacdResult>,
    pub supertrend: Option<SupertrendResult>,
    pub bollinger: Option<BollingerBandsResult>,
    pub volume_trend: String,
    pub adx: Option<f64>,
    pub stochastic: Option<StochasticResult>,
    pub delivery_trend: Option<DeliveryTrend>,
}

/// Trait for individual signal rules.
/// Each rule inspects indicator values and optionally casts a vote.
pub trait SignalRule: Send + Sync {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote>;
}

/// Engine that collects votes from multiple rules and produces a composite signal.
pub struct SignalEngine {
    rules: Vec<Box<dyn SignalRule>>,
}

impl SignalEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the engine.
    pub fn add_rule(&mut self, rule: Box<dyn SignalRule>) {
        self.rules.push(rule);
    }

    /// Evaluate all rules against the given indicator values.
    pub fn evaluate(&self, values: &IndicatorValues) -> Signal {
        let votes: Vec<SignalVote> = self
            .rules
            .iter()
            .filter_map(|rule| rule.evaluate(values))
            .collect();

        if votes.is_empty() {
            return Signal {
                strength: SignalStrength::Neutral,
                confidence: 0.0,
                reasons: vec!["No indicator data available".to_string()],
            };
        }

        let total_weight: f64 = votes.iter().map(|v| v.weight).sum();
        if total_weight < f64::EPSILON {
            return Signal {
                strength: SignalStrength::Neutral,
                confidence: 0.0,
                reasons: vec!["All weights are zero".to_string()],
            };
        }

        // Weighted average score
        let weighted_score: f64 = votes
            .iter()
            .map(|v| v.strength.score() * v.weight)
            .sum::<f64>()
            / total_weight;

        let strength = if weighted_score >= 1.5 {
            SignalStrength::StrongBuy
        } else if weighted_score >= 0.5 {
            SignalStrength::Buy
        } else if weighted_score > -0.5 {
            SignalStrength::Neutral
        } else if weighted_score > -1.5 {
            SignalStrength::Sell
        } else {
            SignalStrength::StrongSell
        };

        // Confidence = how much the votes agree (0..1)
        // Perfect agreement = 1.0, evenly split = 0.0
        let max_possible = 2.0; // max absolute score
        let confidence = (weighted_score.abs() / max_possible).min(1.0);

        let reasons: Vec<String> = votes.iter().map(|v| v.reason.clone()).collect();

        Signal {
            strength,
            confidence,
            reasons,
        }
    }
}

impl Default for SignalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysBuy;
    impl SignalRule for AlwaysBuy {
        fn evaluate(&self, _snapshot: &IndicatorValues) -> Option<SignalVote> {
            Some(SignalVote {
                strength: SignalStrength::Buy,
                weight: 1.0,
                reason: "Always buy".to_string(),
            })
        }
    }

    #[test]
    fn engine_single_rule() {
        let mut engine = SignalEngine::new();
        engine.add_rule(Box::new(AlwaysBuy));
        let signal = engine.evaluate(&IndicatorValues::default());
        assert_eq!(signal.strength, SignalStrength::Buy);
        assert!(!signal.reasons.is_empty());
    }

    #[test]
    fn engine_no_rules() {
        let engine = SignalEngine::new();
        let signal = engine.evaluate(&IndicatorValues::default());
        assert_eq!(signal.strength, SignalStrength::Neutral);
        assert_eq!(signal.confidence, 0.0);
    }
}
