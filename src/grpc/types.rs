use crate::proto;

#[derive(Clone, Debug)]
pub enum IndicatorType {
    BollingerBands,
    ExponentialMovingAverage,
    MaxDrawdown,
    MaxDrawup,
    Maximum,
    MeanAbsoluteDeviation,
    Minimum,
    RateOfChange,
    RelativeStrengthIndex,
    SimpleMovingAverage,
    StandardDeviation,
}

#[derive(Clone, Debug)]
pub struct Opt {
    pub multiplier: Option<f64>,
    pub period: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct Actionss {
    pub l: Option<IndicatorType>,
    pub options: Option<Opt>,
    pub list: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub list: Vec<f64>,
}

impl TryFrom<Vec<f64>> for Action {
    type Error = ();
    fn try_from(v: Vec<f64>) -> Result<Self, Self::Error> {
        Ok(Action { list: v })
    }
}

impl TryFrom<&proto::ListNumbersRequest2> for Actionss {
    type Error = ();
    fn try_from(v: &proto::ListNumbersRequest2) -> Result<Self, Self::Error> {
        let l = v.list.clone();

        let m = match v.id {
            x if x == IndicatorType::BollingerBands as i32 => Some(IndicatorType::BollingerBands),
            x if x == IndicatorType::ExponentialMovingAverage as i32 => {
                Some(IndicatorType::ExponentialMovingAverage)
            }
            x if x == IndicatorType::MaxDrawdown as i32 => Some(IndicatorType::MaxDrawdown),
            x if x == IndicatorType::MaxDrawup as i32 => Some(IndicatorType::MaxDrawup),
            x if x == IndicatorType::Maximum as i32 => Some(IndicatorType::Maximum),
            x if x == IndicatorType::MeanAbsoluteDeviation as i32 => {
                Some(IndicatorType::MeanAbsoluteDeviation)
            }
            x if x == IndicatorType::Minimum as i32 => Some(IndicatorType::Minimum),
            x if x == IndicatorType::RateOfChange as i32 => Some(IndicatorType::RateOfChange),
            x if x == IndicatorType::RelativeStrengthIndex as i32 => {
                Some(IndicatorType::RelativeStrengthIndex)
            }
            x if x == IndicatorType::SimpleMovingAverage as i32 => {
                Some(IndicatorType::SimpleMovingAverage)
            }
            x if x == IndicatorType::StandardDeviation as i32 => {
                Some(IndicatorType::StandardDeviation)
            }
            _e => None,
        };

        let ii = if let Some(ee) = v.opt {
            (Some(ee.multiplier), Some(ee.period))
        } else {
            (None, None)
        };

        Ok(Actionss {
            l: m,
            options: Some(Opt {
                multiplier: ii.0,
                period: ii.1,
            }),
            list: l,
        })
    }
}
