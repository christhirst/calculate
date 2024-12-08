use calculate::{
    indicators::{
        BollingerBands, ExponentialMovingAverage, MaxDrawdown, MaxDrawup, Maximum,
        MeanAbsoluteDeviation, Minimum, RateOfChange, RelativeStrengthIndex, SimpleMovingAverage,
        StandardDeviation,
    },
    Next,
};
use chrono::{Duration, TimeZone, Utc};
use proto::indicator_server::Indicator;

use crate::{proto, IndicatorService};

use super::types::{Action, Actionss, IndicatorType};

fn bollinger_bands(list: Vec<f64>, multiplier: f64, time_window: i64) -> Vec<f64> {
    let mut bb = BollingerBands::new(Duration::days(time_window), multiplier).unwrap();
    let now = Utc::now();

    list.iter()
        .enumerate()
        .map(|(i, a)| bb.next((now + Duration::days(i as i64), *a)))
        .collect()
}

fn exponential_moving_average(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    list.iter()
        .enumerate()
        .map(|(i, a)| ema.next((now + Duration::days(i as i64), *a)))
        .collect()
}

fn max_drawdown(list: Vec<f64>) -> Vec<f64> {
    //Duration just for the timewindow, needs to be configureable
    let duration = Duration::seconds(10000);
    let mut max = MaxDrawdown::new(duration).unwrap();
    let start_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    list.iter()
        .enumerate()
        .map(|(i, a)| max.next((start_time + Duration::seconds((i) as i64), *a)))
        .collect()
}

fn max_drawup(list: Vec<f64>) -> Vec<f64> {
    //Duration just for the timewindow, needs to be configureable
    let duration = Duration::seconds(2);
    let mut max = MaxDrawup::new(duration).unwrap();
    let start_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    list.iter()
        .enumerate()
        .map(|(i, a)| max.next((start_time + Duration::seconds((i) as i64), *a)))
        .collect()
}

fn maximum(list: Vec<f64>) -> Vec<f64> {
    let duration = Duration::seconds(100);
    let mut max = Maximum::new(duration).unwrap();
    let start_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    list.iter()
        .enumerate()
        .map(|(i, a)| max.next((start_time + Duration::seconds((i * 50) as i64), *a)))
        .collect()
}

fn mean_absolute_deviation(list: Vec<f64>) -> Vec<f64> {
    let duration = Duration::seconds(5);
    let mut mad = MeanAbsoluteDeviation::new(duration).unwrap();

    list.iter()
        .enumerate()
        .map(|(i, a)| mad.next((Utc.timestamp(i as i64, 0), *a)))
        .collect()
}

fn minimum(list: Vec<f64>) -> Vec<f64> {
    let duration = Duration::days(2);
    let mut min = Minimum::new(duration).unwrap();
    //TODO all the same date format
    list.iter()
        .enumerate()
        .map(|(i, a)| {
            min.next((
                Utc.datetime_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
                *a,
            ))
        })
        .collect()
}

fn rate_of_change(list: Vec<f64>) -> Vec<f64> {
    let mut roc = RateOfChange::new(Duration::seconds(3)).unwrap();
    let start_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    list.iter()
        .enumerate()
        .map(|(i, a)| roc.next((start_time + Duration::seconds(i as i64), *a)))
        .collect()
}

fn relative_strength_index(list: Vec<f64>) -> Vec<f64> {
    let mut rsi = RelativeStrengthIndex::new(Duration::days(3)).unwrap();
    let timestamp = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    list.iter()
        .enumerate()
        .map(|(i, a)| rsi.next((timestamp + Duration::days(1), *a)))
        .collect()
}

fn simple_moving_average(list: Vec<f64>) -> Vec<f64> {
    //Duration just for the timewindow, needs to be configureable
    let duration = Duration::seconds(5);
    let mut sma = SimpleMovingAverage::new(duration).unwrap();
    let start_time = Utc::now();
    let elapsed_time = Duration::seconds(1);

    list.iter()
        .enumerate()
        .map(|(i, a)| sma.next((start_time + elapsed_time * (i as i32), *a)))
        .collect()
}

fn standard_deviation(list: Vec<f64>) -> Vec<f64> {
    let duration = Duration::seconds(4);
    let mut sd = StandardDeviation::new(duration).unwrap();
    let now = Utc::now();

    list.iter()
        .enumerate()
        .map(|(i, a)| sd.next(((now + Duration::seconds(3 + i as i64)), *a)))
        .collect()
}

#[tonic::async_trait]
impl Indicator for IndicatorService {
    async fn conf_reload(
        &self,
        _request: tonic::Request<proto::UserRequest>,
    ) -> Result<tonic::Response<proto::ConfigResponse>, tonic::Status> {
        Ok(tonic::Response::new(proto::ConfigResponse {
            result: String::from("ee"),
        }))
    }
    async fn gen_list(
        &self,
        _request: tonic::Request<proto::ListNumbersRequest>,
    ) -> Result<tonic::Response<proto::ListNumbersResponse>, tonic::Status> {
        let r: Action = _request.get_ref().list.clone().try_into().unwrap();
        //println!("{:?}", r);

        let list = bollinger_bands(r.list, 2.0, 5);

        Ok(tonic::Response::new(proto::ListNumbersResponse {
            result: list,
        }))
    }

    async fn gen_liste(
        &self,
        _request: tonic::Request<proto::ListNumbersRequest2>,
    ) -> Result<tonic::Response<proto::ListNumbersResponse>, tonic::Status> {
        let r: Actionss = _request
            .get_ref()
            .try_into()
            .map_err(|e| tonic::Status::new(tonic::Code::NotFound, format!("{:?}", e)))?;
        let oo = r.l.ok_or(tonic::Status::new(
            tonic::Code::NotFound,
            String::from("Action not found"),
        ))?;
        match oo {
            IndicatorType::BollingerBands => {
                let opt = if let Some(r) = r.options {
                    (r.multiplier.unwrap_or(2.0), r.period.unwrap_or(5))
                } else {
                    (2.0, 5)
                };
                println!("{:?}", opt);
                let list = bollinger_bands(r.list, opt.0, opt.1);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::ExponentialMovingAverage => {
                let list = exponential_moving_average(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::MaxDrawdown => {
                let list = max_drawdown(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::MaxDrawup => {
                let list = max_drawup(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::Maximum => {
                let list = maximum(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::MeanAbsoluteDeviation => {
                let list = mean_absolute_deviation(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            IndicatorType::Minimum => {
                let list = minimum(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            IndicatorType::RateOfChange => {
                let list = rate_of_change(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            IndicatorType::RelativeStrengthIndex => {
                let list = relative_strength_index(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            IndicatorType::SimpleMovingAverage => {
                let list = simple_moving_average(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            IndicatorType::StandardDeviation => {
                let list = standard_deviation(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }

            _ => Err(tonic::Status::new(
                tonic::Code::NotFound,
                String::from("Action not found"),
            )),
        }

        //let list = cal_list(r.list);

        /* Ok(tonic::Response::new(proto::ListNumbersResponse {
            result: vec![1.0],
        })) */
    }
}
