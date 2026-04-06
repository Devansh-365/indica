use crate::indicators::momentum::macd::Crossover;
use crate::indicators::trend::supertrend::SupertrendDirection;

use super::engine::{IndicatorValues, SignalRule};
use super::types::{SignalStrength, SignalVote};

/// RSI-based signal: oversold = Buy, overbought = Sell.
pub struct RsiRule;

impl SignalRule for RsiRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let rsi = snapshot.rsi?;
        let (strength, reason) = if rsi < 20.0 {
            (SignalStrength::StrongBuy, format!("RSI {rsi:.1} — deeply oversold"))
        } else if rsi < 30.0 {
            (SignalStrength::Buy, format!("RSI {rsi:.1} — oversold"))
        } else if rsi > 80.0 {
            (SignalStrength::StrongSell, format!("RSI {rsi:.1} — deeply overbought"))
        } else if rsi > 70.0 {
            (SignalStrength::Sell, format!("RSI {rsi:.1} — overbought"))
        } else {
            (SignalStrength::Neutral, format!("RSI {rsi:.1} — neutral zone"))
        };
        Some(SignalVote {
            strength,
            weight: 0.8,
            reason,
        })
    }
}

/// MACD crossover signal.
pub struct MacdCrossoverRule;

impl SignalRule for MacdCrossoverRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let macd = snapshot.macd.as_ref()?;
        let (strength, reason) = match macd.crossover {
            Crossover::Bullish => (
                SignalStrength::Buy,
                format!("MACD bullish crossover (histogram: {:.2})", macd.histogram),
            ),
            Crossover::Bearish => (
                SignalStrength::Sell,
                format!("MACD bearish crossover (histogram: {:.2})", macd.histogram),
            ),
            Crossover::Neutral => {
                if macd.histogram > 0.0 {
                    (
                        SignalStrength::Buy,
                        format!("MACD positive histogram ({:.2})", macd.histogram),
                    )
                } else if macd.histogram < 0.0 {
                    (
                        SignalStrength::Sell,
                        format!("MACD negative histogram ({:.2})", macd.histogram),
                    )
                } else {
                    (SignalStrength::Neutral, "MACD flat".to_string())
                }
            }
        };
        Some(SignalVote {
            strength,
            weight: 0.9,
            reason,
        })
    }
}

/// Supertrend direction signal.
pub struct SupertrendRule;

impl SignalRule for SupertrendRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let st = snapshot.supertrend.as_ref()?;
        let (strength, reason) = match st.direction {
            SupertrendDirection::Up => (
                SignalStrength::Buy,
                format!("Supertrend UP — support at {:.2}", st.value),
            ),
            SupertrendDirection::Down => (
                SignalStrength::Sell,
                format!("Supertrend DOWN — resistance at {:.2}", st.value),
            ),
        };
        Some(SignalVote {
            strength,
            weight: 1.0,
            reason,
        })
    }
}

/// Volume trend confirmation / warning.
pub struct VolumeTrendRule;

impl SignalRule for VolumeTrendRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let trend = snapshot.volume_trend.as_str();
        let (strength, reason) = match trend {
            "surging" => (
                SignalStrength::Buy,
                "Volume surging — confirms momentum".to_string(),
            ),
            "increasing" => (
                SignalStrength::Buy,
                "Volume increasing — supports trend".to_string(),
            ),
            "declining" => (
                SignalStrength::Sell,
                "Volume declining — trend weakening".to_string(),
            ),
            "drying up" => (
                SignalStrength::StrongSell,
                "Volume drying up — caution".to_string(),
            ),
            "stable" => (
                SignalStrength::Neutral,
                "Volume stable".to_string(),
            ),
            _ => return None,
        };
        Some(SignalVote {
            strength,
            weight: 0.6,
            reason,
        })
    }
}

/// ADX trend strength — confirms or weakens signals.
pub struct AdxTrendRule;

impl SignalRule for AdxTrendRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let adx_val = snapshot.adx?;
        let (strength, reason) = if adx_val > 40.0 {
            (
                SignalStrength::StrongBuy,
                format!("ADX {adx_val:.1} — very strong trend"),
            )
        } else if adx_val > 25.0 {
            (
                SignalStrength::Neutral,
                format!("ADX {adx_val:.1} — trending (confirms direction)"),
            )
        } else if adx_val > 20.0 {
            (
                SignalStrength::Neutral,
                format!("ADX {adx_val:.1} — weak trend"),
            )
        } else {
            (
                SignalStrength::Neutral,
                format!("ADX {adx_val:.1} — no trend, range-bound"),
            )
        };
        Some(SignalVote {
            strength,
            weight: 0.5,
            reason,
        })
    }
}

/// Stochastic Oscillator rule.
pub struct StochasticRule;

impl SignalRule for StochasticRule {
    fn evaluate(&self, snapshot: &IndicatorValues) -> Option<SignalVote> {
        let stoch = snapshot.stochastic.as_ref()?;
        let (strength, reason) = if stoch.k < 20.0 && stoch.d < 20.0 {
            (
                SignalStrength::StrongBuy,
                format!("Stochastic K={:.1} D={:.1} — deeply oversold", stoch.k, stoch.d),
            )
        } else if stoch.k < 30.0 {
            (
                SignalStrength::Buy,
                format!("Stochastic K={:.1} — oversold zone", stoch.k),
            )
        } else if stoch.k > 80.0 && stoch.d > 80.0 {
            (
                SignalStrength::StrongSell,
                format!("Stochastic K={:.1} D={:.1} — deeply overbought", stoch.k, stoch.d),
            )
        } else if stoch.k > 70.0 {
            (
                SignalStrength::Sell,
                format!("Stochastic K={:.1} — overbought zone", stoch.k),
            )
        } else {
            (
                SignalStrength::Neutral,
                format!("Stochastic K={:.1} — neutral zone", stoch.k),
            )
        };
        Some(SignalVote {
            strength,
            weight: 0.7,
            reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::momentum::macd::MacdResult;
    use crate::indicators::momentum::stochastic::StochasticResult;
    use crate::indicators::trend::supertrend::SupertrendResult;

    #[test]
    fn rsi_oversold() {
        let vals = IndicatorValues {
            rsi: Some(25.0),
            ..Default::default()
        };
        let vote = RsiRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::Buy);
    }

    #[test]
    fn rsi_overbought() {
        let vals = IndicatorValues {
            rsi: Some(75.0),
            ..Default::default()
        };
        let vote = RsiRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::Sell);
    }

    #[test]
    fn rsi_deeply_oversold() {
        let vals = IndicatorValues {
            rsi: Some(15.0),
            ..Default::default()
        };
        let vote = RsiRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::StrongBuy);
    }

    #[test]
    fn macd_bullish_crossover() {
        let vals = IndicatorValues {
            macd: Some(MacdResult {
                value: 1.0,
                signal: 0.5,
                histogram: 0.5,
                crossover: Crossover::Bullish,
            }),
            ..Default::default()
        };
        let vote = MacdCrossoverRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::Buy);
    }

    #[test]
    fn supertrend_up() {
        let vals = IndicatorValues {
            supertrend: Some(SupertrendResult {
                value: 100.0,
                direction: SupertrendDirection::Up,
            }),
            ..Default::default()
        };
        let vote = SupertrendRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::Buy);
    }

    #[test]
    fn volume_surging() {
        let vals = IndicatorValues {
            volume_trend: "surging".to_string(),
            ..Default::default()
        };
        let vote = VolumeTrendRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::Buy);
    }

    #[test]
    fn adx_strong() {
        let vals = IndicatorValues {
            adx: Some(45.0),
            ..Default::default()
        };
        let vote = AdxTrendRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::StrongBuy);
    }

    #[test]
    fn stochastic_oversold() {
        let vals = IndicatorValues {
            stochastic: Some(StochasticResult { k: 15.0, d: 15.0 }),
            ..Default::default()
        };
        let vote = StochasticRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::StrongBuy);
    }

    #[test]
    fn stochastic_overbought() {
        let vals = IndicatorValues {
            stochastic: Some(StochasticResult { k: 85.0, d: 85.0 }),
            ..Default::default()
        };
        let vote = StochasticRule.evaluate(&vals).unwrap();
        assert_eq!(vote.strength, SignalStrength::StrongSell);
    }

    #[test]
    fn missing_data_returns_none() {
        let vals = IndicatorValues::default();
        assert!(RsiRule.evaluate(&vals).is_none());
        assert!(MacdCrossoverRule.evaluate(&vals).is_none());
        assert!(SupertrendRule.evaluate(&vals).is_none());
        assert!(AdxTrendRule.evaluate(&vals).is_none());
        assert!(StochasticRule.evaluate(&vals).is_none());
    }
}
