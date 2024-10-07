use calculate::indicators::BollingerBands;
use calculate::indicators::ExponentialMovingAverage;
use calculate::indicators::MaxDrawdown;
use calculate::Next;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;
use hyper::Error;
use proto::indicator_server::Indicator;
use proto::indicator_server::IndicatorServer;
use tonic::transport::Server;
use tracing::span::Id;
use tracing::{debug, info, warn};

mod config;

#[derive(Clone, Debug)]
enum IndicatorType {
    BollingerBands,
    ExponentialMovingAverage,
    MaxDrawdown,
    MaxDrawup,
    Maximum,
    Minimum,
    RateOfChange,
    RelativeStrengthIndex,
    SimpleMovingAverage,
    StandardDeviation,
}

#[derive(Clone, Debug)]
struct Actionss {
    l: Option<IndicatorType>,
    list: Vec<f64>,
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
#[derive(Clone, Debug)]
// If we don't care about inner capitals, we don't need to set `serialize_all`
// and can leave parenthesis empty.
//#[strum(serialize_all = "lowercase")]
struct Action {
    list: Vec<f64>,
}

impl TryFrom<Vec<f64>> for Action {
    type Error = ();
    fn try_from(v: Vec<f64>) -> Result<Self, Self::Error> {
        Ok(Action { list: v })
    }
}

/* #[derive(Clone, Debug)]
struct Actions {
    l: String,
    list: Vec<f64>,
} */

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
            e => None,
        };

        Ok(Actionss { l: m, list: l })
    }
}

/* impl From<proto::ListNumbersRequest2> for Actionss {
    fn from(rust_form: proto::ListNumbersRequest2) -> Self {
        Actionss {
            l: IndicatorType::ManualComplete,
            list: vec![2.0],
        }
    }
}
 */
/* fn action_mapper(re: tonic::Request<proto::ListNumbersRequest>) -> Result<String, Error> {
    //let action = match &re.get_ref().l

    /* .action.try_into() {
        Ok(Action::Retry) => Some(Action::Retry.to_string()),
        Ok(Action::ManualComplete) => Some(Action::ManualComplete.to_string()),
        Err(_) => {
            panic!("Unknown action");
        }
    }
    .ok_or(CliError::EntityNotFound { entity: "", id: 1 })?; */
    Ok(action)
} */

#[derive(Debug, Default)]
pub struct IndicatorService;

pub mod proto {
    tonic::include_proto!("calculate");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("indicator_descriptor");
}

fn bollinger_bands(list: Vec<f64>) -> Vec<f64> {
    let mut bb = BollingerBands::new(Duration::days(3), 2.0).unwrap();
    let now = Utc::now();

    list.iter().map(|a| bb.next((now, *a))).collect()
}

fn exponential_moving_average(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn max_drawdown(list: Vec<f64>) -> Vec<f64> {
    let duration = Duration::seconds(2);
    let mut max = MaxDrawdown::new(duration).unwrap();
    let start_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

    max.next((start_time, 4.0));

    list.iter()
        .enumerate()
        .map(|(i, a)| max.next((start_time + Duration::seconds((i + 1) as i64), *a)))
        .collect()
}

fn max_drawup(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn maximum(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn mean_absolute_deviation(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn minimum(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn rate_of_change(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn relative_strength_index(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn simple_moving_average(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
}

fn standard_deviation(list: Vec<f64>) -> Vec<f64> {
    let mut ema = ExponentialMovingAverage::new(Duration::days(5)).unwrap();
    let now = Utc::now();

    assert_eq!(ema.next((now, 4.0)), 4.0);
    ema.next((now + Duration::days(1), 10.0));

    list.iter().map(|a| ema.next((now, *a))).collect()
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
        println!("{:?}", r);

        let list = bollinger_bands(r.list);

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
        println!("{:?}", r);
        let oo = r.l.ok_or(tonic::Status::new(
            tonic::Code::NotFound,
            String::from("Action not found"),
        ))?;
        match oo {
            IndicatorType::BollingerBands => {
                let list = bollinger_bands(r.list);
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "[::1]:50051".parse().unwrap();
    //GRPC server
    let calc = IndicatorService::default();
    //GRPC reflection
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .accept_http1(true)
        //.layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(IndicatorServer::new(calc))
        //.add_service(tonic_web::enable(CalculatorServer::new(calc)))
        //.add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await
        .unwrap();
    todo!()
}
