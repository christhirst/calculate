struct Candle<'a> {
    open: &'a f64,
    high: &'a f64,
    low: &'a f64,
    close: &'a f64,
}

fn is_bearish_candle(current_candle: &Candle) -> f64 {
    if current_candle.close < current_candle.open {
        return -current_candle.open / current_candle.close;
    }
    0.0
}
fn is_bullish_candle(current_candle: &Candle) -> f64 {
    if current_candle.close > current_candle.open {
        return current_candle.close / current_candle.open;
    }
    0.0
}

fn is_bullish_engulfing(previous_candle: Candle, current_candle: Candle) -> Option<(f64, f64)> {
    if is_bearish_candle(&previous_candle) < 0.0
        && current_candle.close > previous_candle.open
        && current_candle.open < previous_candle.close
        && current_candle.high > previous_candle.high
        && current_candle.low < previous_candle.low
    {
        return Some((
            current_candle.close / previous_candle.open,
            previous_candle.close / current_candle.open,
        ));
    }
    None
}

fn is_bearish_engulfing(previous_candle: Candle, current_candle: Candle) -> Option<(f64, f64)> {
    if is_bullish_candle(&previous_candle) > 0.0
        && current_candle.open > previous_candle.close
        && current_candle.close < previous_candle.open
        && current_candle.close < previous_candle.low
    {
        return Some((
            current_candle.open / previous_candle.close,
            previous_candle.open / current_candle.close,
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candles() {
        let bear_candle = Candle {
            open: &14.0,
            high: &2.0,
            low: &1.0,
            close: &2.0,
        };
        let bull_candle = Candle {
            close: &28.0,
            ..bear_candle
        };
        assert_eq!(is_bearish_candle(&bear_candle), -7.0);
        assert_eq!(is_bullish_candle(&bull_candle), 2.0);
    }

    #[test]
    fn test_bull_ec() {
        let prev_candle = Candle {
            open: &2.0,
            high: &2.0,
            low: &1.0,
            close: &1.0,
        };
        let cur_candle = Candle {
            open: &0.5,
            high: &3.0,
            low: &0.5,
            close: &3.0,
        };
        assert_eq!(
            is_bullish_engulfing(prev_candle, cur_candle),
            Some((1.5, 2.0))
        );
    }
    #[test]
    fn test_baer_ec() {
        let prev_candle = Candle {
            open: &2.5,
            high: &2.0,
            low: &1.0,
            close: &3.0,
        };
        let cur_candle = Candle {
            open: &6.0,
            high: &3.0,
            low: &0.5,
            close: &0.5,
        };
        assert_eq!(
            is_bearish_engulfing(prev_candle, cur_candle),
            Some((2.0, 5.0))
        );
    }
}
