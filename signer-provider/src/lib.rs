#[macro_use]
extern crate tracing;

use std::vec;

use anyhow::{bail, Result};
use signer::{signer_client::SignerClient, BatchRetrieveRequest, RetrieveRequest};

pub mod signer {
    tonic::include_proto!("signer");
}

const MESSAGE_SIZE_LIMIT: usize = 1024 * 1024 * 1024; // 1G

pub struct RetrieveParam {
    pub epoch: u64,
    pub quorum_id: u64,
    pub storage_root: Vec<u8>,
    pub row_indexes: Vec<u32>,
}

pub struct SignerProvider {}

impl SignerProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn get_slices(
        &self,
        socket: String,
        retrieve_params: Vec<RetrieveParam>,
    ) -> Result<Vec<Vec<Vec<u8>>>> {
        info!("request slices from {:?}", socket);
        let mut client = SignerClient::connect(socket.clone())
            .await?
            .max_decoding_message_size(MESSAGE_SIZE_LIMIT)
            .max_encoding_message_size(MESSAGE_SIZE_LIMIT);

        let request = tonic::Request::new(BatchRetrieveRequest {
            requests: retrieve_params
                .into_iter()
                .map(|p| RetrieveRequest {
                    epoch: p.epoch,
                    quorum_id: p.quorum_id,
                    storage_root: p.storage_root,
                    row_indexes: p.row_indexes,
                })
                .collect(),
        });

        let response = match client.batch_retrieve(request).await {
            Ok(v) => v.into_inner(),
            Err(e) => {
                bail!("socket: {:?}, err: {:?}", socket, e);
            }
        };

        let mut res = vec![];
        for slices in response.encoded_slice.into_iter() {
            let mut s = vec![];
            for mut slice in slices.encoded_slice.into_iter() {
                slice.drain(0..8); // first 8 byte is length
                s.push(slice);
            }

            res.push(s);
        }

        Ok(res)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
