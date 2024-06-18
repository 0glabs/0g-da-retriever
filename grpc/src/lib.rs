#[macro_use]
extern crate tracing;

mod service;

pub use service::retriever;

use crate::service::retriever::retriever_server::RetrieverServer;
use contract_provider::ContractProvider;
use service::RetrieverService;
use signer_provider::SignerProvider;
use std::{net::SocketAddr, sync::Arc};
use task_executor::TaskExecutor;
use tonic::transport::Server;

const MESSAGE_SIZE_LIMIT: usize = 1024 * 1024 * 1024; // 1G

pub async fn run_server(
    addr: SocketAddr,
    contract_provider: ContractProvider,
    signer_provider: Arc<SignerProvider>,
    max_ongoing_sign_request: Option<u64>,
    executor: TaskExecutor,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer_service = RetrieverService::new(
        contract_provider,
        signer_provider,
        max_ongoing_sign_request,
        executor,
    );

    info!("grpc server listening {:?}", addr);
    Server::builder()
        .add_service(
            RetrieverServer::new(signer_service)
                .max_decoding_message_size(MESSAGE_SIZE_LIMIT)
                .max_encoding_message_size(MESSAGE_SIZE_LIMIT),
        )
        .serve(addr)
        .await?;
    Ok(())
}
