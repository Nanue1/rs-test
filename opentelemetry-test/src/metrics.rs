use std::net::SocketAddr;

use axum::{Extension, handler::Handler, Router, routing::get};
use clap::Args;
use futures::{Future, future::OptionFuture,FutureExt};
use hyper::{Request, Body, Response, StatusCode};
use opentelemetry::{metrics::{Counter, Meter, Histogram}, global, sdk::{metrics::{controllers, processors, selectors}, export::metrics::aggregation}, KeyValue, Context};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{TextEncoder, Encoder};

use crate::validate::Validate;


#[derive(Args)]
pub struct Opts {
    /// Address to expose Prometheus metrics on
    /// Examples: 127.0.0.1:9090, [::1]:9090
    #[clap(long)]
    metrics_addr: Option<SocketAddr>,
}

pub struct WithMetrics<T>(pub T, pub MetricParams);

pub struct MetricParams {
    pub cx : Context,
    pub counter: Counter<u64>,
    pub gauge: Histogram<u64>,
    pub histogram: Histogram<f64>,
}

impl MetricParams {
    pub fn new(meter: &Meter, name: &str) -> Self {
        Self {
            cx: Context::current(),
            counter: meter
                .u64_counter(format!("{name}.total"))
                .with_description(format!("Counts occurences of {name} calls"))
                .init(),
            gauge: meter
                .u64_histogram(format!("{name}.response_size_bytes"))
                .with_description(format!("The metrics of {name} response sizes in bytes."))
                .init(),
            histogram: meter
                .f64_histogram(format!("{name}.request_duration_seconds"))
                .with_description(format!("The {name} request latencies in seconds."))
                .init(),
        }
    }
}

#[derive(Clone)]
struct HandlerArgs {
    exporter: PrometheusExporter,
}

async fn metrics_handler(
    Extension(HandlerArgs { exporter }): Extension<HandlerArgs>,
    _: Request<Body>,
) -> Response<Body> {
    let metric_families = exporter.registry().gather();

    let encoder = TextEncoder::new();

    let mut metrics_text = Vec::new();
    if encoder.encode(&metric_families, &mut metrics_text).is_err() {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal Server Error".into())
            .unwrap();
    };

    Response::builder()
        .status(200)
        .body(metrics_text.into())
        .unwrap()
}

pub struct Runner {
    exporter: PrometheusExporter,
    metrics_addr: Option<SocketAddr>,
}

impl Runner {
    pub fn run(self) -> impl Future<Output = Result<Option<()>, hyper::Error>> {
        let exporter = self.exporter;
        OptionFuture::from(self.metrics_addr.map(|metrics_addr| {
            let metrics_handler = metrics_handler.layer(Extension(HandlerArgs { exporter }));
            let metrics_router = Router::new().route("/metrics", get(metrics_handler));

            axum::Server::bind(&metrics_addr).serve(metrics_router.into_make_service())
        }))
        .map(|v| v.transpose())
    }
}

fn _init_meter() -> PrometheusExporter {
    let controller = controllers::basic(
        processors::factory(
            selectors::simple::histogram([1.0, 2.0, 5.0, 10.0, 20.0, 50.0]),
            aggregation::cumulative_temporality_selector(),
        )
        
        .with_memory(true),
    )
    .build();

    opentelemetry_prometheus::exporter(controller).init()
}

pub fn setup(opts: Opts) -> (Meter, Runner) {
    let exporter = _init_meter();
    (
        global::meter("opentelemetry_test"),
        Runner {
            exporter,
            metrics_addr: opts.metrics_addr,
        },
    )
}

// =================================================================打点场景================================================================

impl <T: Validate>Validate for WithMetrics<T>{
    fn validate(&self, val: &str) ->Result<(),String> {
        let MetricParams{cx,counter,..} = &self.1;
        let data = &[KeyValue::new("validate_data", val.to_string()),KeyValue::new("service_name", "val.to_string()")];
        counter.add(cx, 1, data);

        self.0.validate(val)
    }
}