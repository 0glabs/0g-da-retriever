use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
    time::Instant,
};

use anyhow::bail;
use contract_provider::{ContractProvider, SignerDetail, MIN_REQUIRED_SLICE};
use ethers::types::H160;
use retriever::{retriever_server::Retriever, BlobReply, BlobRequest};
use signer_provider::{RetrieveParam, SignerProvider};
use task_executor::TaskExecutor;
use tokio::sync::{Mutex, RwLock};
use tonic::{Code, Request, Response, Status};
use zg_da_recovery::recover_from_da_slice;

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

        let (signers, signer_first_1024_slices, signer_slices) = self
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
        let invalid_signers = Arc::new(Mutex::new(HashSet::new()));
        for (address, indices) in signer_first_1024_slices.into_iter() {
            let invalid_signers = Arc::clone(&invalid_signers);
            let task = self.spawn_request_task(
                &data_root,
                epoch,
                quorum_id,
                address,
                indices,
                &signers,
                invalid_signers,
            )?;

            tasks.push(task);
        }

        let mut input_slices: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
        for task in tasks {
            wait_request_task(task, &mut input_slices).await;
        }

        info!("ready slices length {:?}", input_slices.len());

        let mut iter = signer_slices.into_iter();
        let mut running = true;
        while input_slices.len() < MIN_REQUIRED_SLICE && running {
            let mut tasks = vec![];
            let mut new_requesting_slice_length = 0;

            loop {
                let (address, indices) = if let Some((address, indices)) = iter.next() {
                    (address, indices)
                } else {
                    error!("no eligible signers available for data request");
                    running = false;
                    break;
                };

                if invalid_signers.lock().await.contains(&address) {
                    continue;
                }

                let slice_len = indices.len();
                let invalid_signers = Arc::clone(&invalid_signers);
                let task = self.spawn_request_task(
                    &data_root,
                    epoch,
                    quorum_id,
                    address,
                    indices,
                    &signers,
                    invalid_signers,
                )?;

                tasks.push(task);

                new_requesting_slice_length += slice_len;
                if new_requesting_slice_length + input_slices.len() >= MIN_REQUIRED_SLICE {
                    break;
                }
            }

            for task in tasks {
                wait_request_task(task, &mut input_slices).await;
            }

            info!("ready slices length {:?}", input_slices.len());
        }

        info!("start recover {:?} ms", ts.elapsed().as_millis());
        let reply = BlobReply {
            data: recover_from_da_slice(&input_slices).map_err(|e| {
                Status::new(Code::NotFound, format!("fail to recover slice: {:?}", e))
            })?,
        };

        info!("response in {:?} ms", ts.elapsed().as_millis());
        Ok(Response::new(reply))
    }

    fn spawn_request_task(
        &self,
        data_root: &Vec<u8>,
        epoch: u64,
        quorum_id: u64,
        address: H160,
        indices: Vec<u32>,
        signers: &HashMap<H160, SignerDetail>,
        invalid_signers: Arc<Mutex<HashSet<H160>>>,
    ) -> Result<
        task_executor::JoinHandle<Option<Result<(Vec<u32>, Vec<Vec<u8>>), anyhow::Error>>>,
        Status,
    > {
        let mut socket = signers
            .get(&address)
            .ok_or(Status::new(Code::InvalidArgument, "signer does't exist"))?
            .socket
            .clone();

        if !socket.to_lowercase().starts_with("http://") {
            socket = format!("http://{}", socket);
        }

        let signer_provider = self.signer_provider.clone();
        let data_root = data_root.clone();
        let task = self
            .executor
            .spawn_handle(
                async move {
                    let mut response = signer_provider
                        .get_slices(
                            socket,
                            vec![RetrieveParam {
                                epoch: epoch,
                                quorum_id,
                                storage_root: data_root,
                                row_indexes: indices.clone(),
                            }],
                        )
                        .await?;

                    match response.pop() {
                        Some(s) => Ok((indices, s)),
                        None => {
                            invalid_signers.lock().await.insert(address);
                            bail!("slice is empty")
                        }
                    }
                },
                "request slice",
            )
            .ok_or(Status::new(Code::Internal, "failed to spawn request slice"))?;

        Ok(task)
    }
}

async fn wait_request_task(
    task: task_executor::JoinHandle<Option<Result<(Vec<u32>, Vec<Vec<u8>>), anyhow::Error>>>,
    slices: &mut BTreeMap<usize, Vec<u8>>,
) {
    match task.await {
        Ok(r) => match r {
            Some(v) => match v {
                Ok((indices, s)) => {
                    indices
                        .into_iter()
                        .zip(s.into_iter())
                        .for_each(|(key, value)| {
                            slices.insert(key as usize, value);
                        });
                }
                Err(e) => {
                    error!("retrieve slice failed, error: {:?}", e);
                }
            },
            None => {
                error!("slice is None");
            }
        },
        Err(e) => {
            error!("join error: {:?}", e);
        }
    }
}
