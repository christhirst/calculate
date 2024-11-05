// golden cross: low crosses high and high crosses low and low crosses high
fn gc(high: (f64, f64), low: (f64, f64)) -> Option<bool> {
    if high.0 > low.0 && low.1 > high.1 {
        return Some(true);
    } else if high.0 < low.0 && low.1 < high.1 {
        return Some(false);
    }
    None
}

fn dc(high: (f64, f64), low: (f64, f64)) -> Option<bool> {
    if high.0 < low.0 && low.1 < high.1 {
        return Some(true);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_new() {
        let low = (1.0, 2.0);
        let high = (2.0, 1.0);
        assert!(gc(high, low).unwrap());
    }

    #[test]
    fn test_dc() {
        let low = (2.0, 1.0);
        let high = (1.0, 2.0);
        assert!(dc(high, low).unwrap());
    }
}
