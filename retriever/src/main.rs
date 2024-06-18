#[macro_use]
extern crate tracing;

mod config;
mod runtime;

use std::{error::Error, net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use config::Config;
use contract_provider::ContractProvider;
use grpc::run_server;
use runtime::{make_environment, Environment};
use signer_provider::SignerProvider;
use task_executor::TaskExecutor;
use tracing::Level;

fn main() -> Result<(), Box<dyn Error>> {
    // enable backtraces
    std::env::set_var("RUST_BACKTRACE", "1");

    let (environment, runtime, executor) = make_environment().unwrap();

    let res = runtime.block_on(async { async_main(environment, executor).await });

    if let Err(e) = res {
        error!(reason =?e, "Service exit");
    }

    runtime.shutdown_timeout(std::time::Duration::from_secs(15));
    info!("Stopped");

    Ok(())
}

async fn async_main(
    environment: Environment,
    executor: TaskExecutor,
) -> Result<(), Box<dyn Error>> {
    // CLI, config
    let config = Config::from_cli_file().unwrap();

    // tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.log_level)?)
        // .with_ansi(false)
        .init();

    // let ctx = Context::new(config).await?;

    let rpc_res = start_server(executor, &config).await;
    rpc_res?;

    environment.wait_shutdown_signal().await;

    info!("signal received, stopping..");
    Ok(())
}

async fn start_server(executor: TaskExecutor, cfg: &Config) -> Result<()> {
    let grpc_listen_address = cfg.grpc_listen_address.clone();
    let max_ongoing_retrieve_request = cfg.max_ongoing_retrieve_request;

    let contract_provider = ContractProvider::new(&cfg.eth_rpc_url).await?;
    let signer_provider = Arc::new(SignerProvider::new()?);

    info!("starting grpc server at {:?}", grpc_listen_address);
    tokio::spawn(async move {
        run_server(
            SocketAddr::from_str(&grpc_listen_address).unwrap(),
            contract_provider,
            signer_provider,
            max_ongoing_retrieve_request,
            executor,
        )
        .await
        .map_err(|e| anyhow!(e.to_string()))
        .unwrap();
    });
    Ok(())
}
