use std::vec;

use calculate::indicators::BollingerBands;
use calculate::indicators::ExponentialMovingAverage;
use calculate::Next;
use chrono::Duration;
use chrono::Utc;
use hyper::Error;
use proto::indicator_server::Indicator;
use proto::indicator_server::IndicatorServer;
use strum_macros::Display;
use tonic::transport::Server;
use tracing::span::Id;
use tracing::{debug, info, warn};

mod config;

#[derive(Clone, Debug)]
enum IndicatorType {
    Retry,
    ManualComplete,
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

#[derive(Clone, Debug)]
struct Actions {
    l: String,
    list: Vec<f64>,
}

impl TryFrom<&proto::ListNumbersRequest2> for Actionss {
    type Error = ();
    fn try_from(v: &proto::ListNumbersRequest2) -> Result<Self, Self::Error> {
        let l = v.list.clone();

        let m = match v.id {
            x if x == IndicatorType::Retry as i32 => Some(IndicatorType::Retry),
            x if x == IndicatorType::ManualComplete as i32 => Some(IndicatorType::ManualComplete),
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

fn cal_list(list: Vec<f64>) -> Vec<f64> {
    let mut bb = BollingerBands::new(Duration::days(3), 2.0).unwrap();
    let now = Utc::now();

    list.iter().map(|a| bb.next((now, *a))).collect()
}

fn cal_lis2t(list: Vec<f64>) -> Vec<f64> {
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

        let list = cal_list(r.list);

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
            format!("Action not found"),
        ))?;
        match oo {
            IndicatorType::Retry => {
                let list = cal_list(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            IndicatorType::ManualComplete => {
                let list = cal_lis2t(r.list);
                Ok(tonic::Response::new(proto::ListNumbersResponse {
                    result: list,
                }))
            }
            _ => {
                panic!("Unknown action");
            }
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
