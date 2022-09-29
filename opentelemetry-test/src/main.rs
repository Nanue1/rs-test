use clap::{Parser, crate_version, crate_authors};
use futures::try_join;
use metrics::{WithMetrics, MetricParams};
use validate::{Validator, Validate};
use tracing::Instrument;



mod metrics;
mod validate;

fn main() -> Result<(), anyhow::Error> {
    let Opts {metrics} = Opts::parse(); 
    let (meter,metrics) = metrics::setup(metrics);

    let validator = Validator::new();
    let validator = WithMetrics(validator,MetricParams::new(&meter,"validate"));

    let _a = validator.validate("a");
    let _b = validator.validate("b");
    let _b = validator.validate("b");
    let _c = validator.validate("c");


    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .enable_all()
        .build()?;

    rt.block_on(
        async move {
            let v = try_join!(
                metrics.run().in_current_span(),
            );
            if let Err(v) = v {
                return Err(v);
            }
            Ok(())
        }
        .in_current_span()
    )?;
    Ok(())
}

#[derive(Parser)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
    propagate_version = true,
)]
pub(crate) struct Opts {
    #[clap(flatten)]
    metrics: metrics::Opts,
}
