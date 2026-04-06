use super::engine::SignalEngine;
use super::rules::{
    AdxTrendRule, MacdCrossoverRule, RsiRule, StochasticRule, SupertrendRule, VolumeTrendRule,
};

/// Swing trader strategy: RSI + MACD + Supertrend + Volume.
/// Best for multi-day holds on NSE/BSE.
pub fn swing_trader() -> SignalEngine {
    let mut engine = SignalEngine::new();
    engine.add_rule(Box::new(RsiRule));
    engine.add_rule(Box::new(MacdCrossoverRule));
    engine.add_rule(Box::new(SupertrendRule));
    engine.add_rule(Box::new(VolumeTrendRule));
    engine
}

/// Momentum trader strategy: RSI + Stochastic + ADX + Volume.
/// Best for short-term momentum plays.
pub fn momentum_trader() -> SignalEngine {
    let mut engine = SignalEngine::new();
    engine.add_rule(Box::new(RsiRule));
    engine.add_rule(Box::new(StochasticRule));
    engine.add_rule(Box::new(AdxTrendRule));
    engine.add_rule(Box::new(VolumeTrendRule));
    engine
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::momentum::macd::{Crossover, MacdResult};
    use crate::indicators::momentum::stochastic::StochasticResult;
    use crate::indicators::trend::supertrend::{SupertrendDirection, SupertrendResult};
    use crate::signals::engine::IndicatorValues;
    use crate::signals::types::SignalStrength;

    #[test]
    fn swing_trader_bullish() {
        let engine = swing_trader();
        let vals = IndicatorValues {
            rsi: Some(28.0),
            macd: Some(MacdResult {
                value: 1.0,
                signal: 0.5,
                histogram: 0.5,
                crossover: Crossover::Bullish,
            }),
            supertrend: Some(SupertrendResult {
                value: 95.0,
                direction: SupertrendDirection::Up,
            }),
            volume_trend: "surging".to_string(),
            ..Default::default()
        };
        let signal = engine.evaluate(&vals);
        assert_eq!(signal.strength, SignalStrength::Buy);
        assert!(signal.confidence > 0.0);
        assert_eq!(signal.reasons.len(), 4);
    }

    #[test]
    fn swing_trader_bearish() {
        let engine = swing_trader();
        let vals = IndicatorValues {
            rsi: Some(75.0),
            macd: Some(MacdResult {
                value: -1.0,
                signal: -0.5,
                histogram: -0.5,
                crossover: Crossover::Bearish,
            }),
            supertrend: Some(SupertrendResult {
                value: 120.0,
                direction: SupertrendDirection::Down,
            }),
            volume_trend: "declining".to_string(),
            ..Default::default()
        };
        let signal = engine.evaluate(&vals);
        assert_eq!(signal.strength, SignalStrength::Sell);
        assert!(signal.confidence > 0.0);
    }

    #[test]
    fn momentum_trader_bullish() {
        let engine = momentum_trader();
        let vals = IndicatorValues {
            rsi: Some(25.0),
            stochastic: Some(StochasticResult { k: 18.0, d: 15.0 }),
            adx: Some(45.0),
            volume_trend: "surging".to_string(),
            ..Default::default()
        };
        let signal = engine.evaluate(&vals);
        assert!(
            signal.strength == SignalStrength::Buy || signal.strength == SignalStrength::StrongBuy
        );
        assert!(signal.confidence > 0.0);
    }

    #[test]
    fn swing_trader_no_data() {
        let engine = swing_trader();
        let vals = IndicatorValues::default();
        let signal = engine.evaluate(&vals);
        // volume_trend defaults to "" which returns None from VolumeTrendRule
        assert_eq!(signal.strength, SignalStrength::Neutral);
    }
}
