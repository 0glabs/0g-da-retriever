#[macro_use]
extern crate tracing;

use anyhow::{anyhow, bail, Result};
use signer::{signer_client::SignerClient, BatchSignRequest, SignRequest};

pub mod signer {
    tonic::include_proto!("signer");
}

pub struct SignerProvider {}

impl SignerProvider {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn get_slices(
        &self,
        socket: String,
        data_root: Vec<u8>,
        slices: Vec<usize>,
    ) -> Result<Vec<u8>> {
        debug!("request slice from {:?}", socket);
        // let mut client = SignerClient::connect(socket).await?;

        // let request = tonic::Request::new(BatchSignRequest {
        //     requests: vec![SignRequest {
        //         epoch: 1,
        //         quorum_id: 1,
        //         erasure_commitment: vec![],
        //         storage_root: vec![],
        //         encoded_slice: vec![],
        //     }],
        // });

        // let response = client.batch_sign(request).await.unwrap();

        // debug!("RESPONSE={:?}", response);

        Ok(vec![1, 2, 3, 4])
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
