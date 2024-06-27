#[macro_use]
extern crate tracing;

use std::vec;

use anyhow::Result;
use signer::{signer_client::SignerClient, BatchRetrieveRequest, RetrieveRequest};

pub mod signer {
    tonic::include_proto!("signer");
}

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
        debug!("request slices from {:?}", socket);
        let mut client = SignerClient::connect(socket).await?;

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

        let response = client.batch_retrieve(request).await?.into_inner();

        let mut res = vec![];
        for slices in response.encoded_slice.into_iter() {
            let mut s = vec![];
            for slice in slices.encoded_slice.into_iter() {
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
