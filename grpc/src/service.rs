use std::{sync::Arc, time::Instant};

use contract_provider::ContractProvider;
use retriever::{retriever_server::Retriever, BlobReply, BlobRequest};
use signer_provider::SignerProvider;
use task_executor::TaskExecutor;
use tokio::sync::RwLock;
use tonic::{Code, Request, Response, Status};

const DEFAULT_MAX_ONGOING_SIGN_REQUEST: u64 = 10;

pub mod retriever {
    tonic::include_proto!("retriever");
}

pub struct RetrieverService {
    contract_provider: ContractProvider,
    signer_provider: Arc<SignerProvider>,
    executor: TaskExecutor,

    max_ongoing_retrieve_request: u64,
    ongoing_retrieve_request_cnt: Arc<RwLock<u64>>,
}

#[tonic::async_trait]
impl Retriever for RetrieverService {
    async fn retrieve_blob(
        &self,
        request: Request<BlobRequest>,
    ) -> Result<Response<BlobReply>, Status> {
        self.on_incoming_retrieve_request().await?;

        let remote_addr = request.remote_addr();
        info!(?remote_addr, "Received request");

        let message = request.into_inner();
        let reply = self
            .retrieve_blob_inner(message.storage_root, message.epoch, message.quorum_id)
            .await;

        self.on_complete_retrieve_request().await;

        reply
    }
}

impl RetrieverService {
    pub fn new(
        contract_provider: ContractProvider,
        signer_provider: Arc<SignerProvider>,
        max_ongoing_sign_request: Option<u64>,
        executor: TaskExecutor,
    ) -> Self {
        Self {
            contract_provider,
            signer_provider,
            executor,
            max_ongoing_retrieve_request: max_ongoing_sign_request
                .unwrap_or(DEFAULT_MAX_ONGOING_SIGN_REQUEST),
            ongoing_retrieve_request_cnt: Arc::new(RwLock::new(0)),
        }
    }

    async fn on_incoming_retrieve_request(&self) -> Result<(), Status> {
        let mut cnt = self.ongoing_retrieve_request_cnt.write().await;
        if *cnt > self.max_ongoing_retrieve_request {
            return Err(Status::new(Code::ResourceExhausted, "request pool is full"));
        }
        *cnt += 1;
        Ok(())
    }

    async fn on_complete_retrieve_request(&self) {
        let mut cnt = self.ongoing_retrieve_request_cnt.write().await;
        *cnt -= 1;
    }

    async fn retrieve_blob_inner(
        &self,
        data_root: Vec<u8>,
        epoch: u64,
        quorum_id: u64,
    ) -> Result<Response<BlobReply>, Status> {
        let ts = Instant::now();

        let (signers, signer_slices) = self
            .contract_provider
            .get_signers(epoch, quorum_id)
            .await
            .map_err(|e| {
            Status::new(
                Code::NotFound,
                format!("failed get signers from block chain: {:?}", e),
            )
        })?;

        let mut tasks = vec![];
        for (address, slices) in signer_slices.into_iter() {
            let socket = signers
                .get(&address)
                .ok_or(Status::new(Code::InvalidArgument, "signer does't exist"))?
                .socket
                .clone();

            let signer_provider = self.signer_provider.clone();
            let data_root = data_root.clone();

            let task = self
                .executor
                .spawn_handle(
                    async move { signer_provider.get_slices(socket, data_root, slices).await },
                    "request slice",
                )
                .ok_or(Status::new(Code::Internal, "failed to spawn request slice"))?;

            tasks.push(task);
        }

        let mut slices = vec![];
        for task in tasks {
            let slice = task
                .await
                .map_err(|e| Status::new(Code::Internal, format!("join error: {:?}", e)))?
                .ok_or(Status::new(Code::Internal, "failed to run slice"))?
                .map_err(|e| Status::new(Code::NotFound, format!("fail to get slice: {:?}", e)))?;

            slices.push(slice);
        }

        // todo: construct original data
        let reply = BlobReply {
            data: slices[0].clone(),
        };

        info!("response in {:?} ms", ts.elapsed().as_millis());
        Ok(Response::new(reply))
    }
}
