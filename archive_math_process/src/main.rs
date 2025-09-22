mod config_logger;
mod config_math;
mod config_timescaledb;
mod config_timescaledb_reader;
mod message;

use rsiot::logging::{LogConfig, LogConfigFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogConfig {
        filter: LogConfigFilter::String("info"),
    }
    .run()
    .unwrap();

    use tokio::time::Duration;

    use rsiot::executor::{ComponentExecutor, ComponentExecutorConfig};

    let executor_config = ComponentExecutorConfig {
        buffer_size: 1000,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::new(executor_config)
        .add_cmp(config_timescaledb_reader::cmp())
        .add_cmp(config_logger::cmp())
        .add_cmp(config_math::cmp())
        .add_cmp(config_timescaledb::cmp())
        .wait_result()
        .await?;

    Ok(())
}
