#[macro_use]
extern crate tracing;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, bail, Result};
use ethers::{
    prelude::abigen,
    providers::{Http, HttpRateLimitRetryPolicy, Provider, RetryClient, RetryClientBuilder},
    types::{H160, U256},
};

pub const DA_SIGNER_ADDRESS: &str = "0x0000000000000000000000000000000000001000";
pub const MIN_REQUIRED_SLICE: usize = 1024;

abigen!(DASigners, "./contract-provider/abi/IDASigners.json");

pub struct ContractProvider {
    #[allow(dead_code)]
    provider: Arc<Provider<RetryClient<Http>>>,

    da_signers: Arc<DASigners<Provider<RetryClient<Http>>>>,
}

impl ContractProvider {
    pub async fn new(eth_rpc_url: &str) -> Result<Self> {
        let provider = Arc::new(Provider::new(
            RetryClientBuilder::default()
                .rate_limit_retries(100)
                .timeout_retries(100)
                .initial_backoff(Duration::from_millis(500))
                .build(
                    Http::from_str(eth_rpc_url)?,
                    Box::new(HttpRateLimitRetryPolicy),
                ),
        ));

        let da_signers = Arc::new(DASigners::new(
            H160::from_str(DA_SIGNER_ADDRESS).unwrap(),
            provider.clone(),
        ));

        Ok(Self {
            provider,
            da_signers,
        })
    }

    pub async fn get_signers(
        &self,
        epoch: u64,
        quorum_id: u64,
    ) -> Result<(
        HashMap<H160, SignerDetail>,
        HashMap<H160, Vec<u32>>,
        HashMap<H160, Vec<u32>>,
    )> {
        debug!("get signers fro epoch {}, quorum id {}", epoch, quorum_id);

        let quorums = self
            .da_signers
            .get_quorum(U256::from(epoch), U256::from(quorum_id))
            .call()
            .await?;

        if quorums.len() == 0 {
            bail!(anyhow!("quorum is empty"));
        }

        debug!("quorum size {}", quorums.len());

        let mut signer_first_1024_slices: HashMap<H160, Vec<u32>> = HashMap::new();
        let mut signer_slices: HashMap<H160, Vec<u32>> = HashMap::new();
        let mut unique_signer = HashSet::new();
        quorums.into_iter().enumerate().for_each(|(i, addr)| {
            if i >= MIN_REQUIRED_SLICE {
                signer_slices
                    .entry(addr)
                    .and_modify(|e| e.push(i as u32))
                    .or_insert(vec![i as u32]);
            } else {
                signer_first_1024_slices
                    .entry(addr)
                    .and_modify(|e| e.push(i as u32))
                    .or_insert(vec![i as u32]);
            }

            unique_signer.insert(addr);
        });

        let signers = self
            .da_signers
            .get_signer(unique_signer.into_iter().collect())
            .call()
            .await?;

        debug!("signer size {}", signers.len());
        let signers = signers
            .into_iter()
            .map(|t| (t.signer, t))
            .collect::<HashMap<_, _>>();

        Ok((signers, signer_first_1024_slices, signer_slices))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ethers::contract::Abigen::new("api", "./abi/IDASigners.json")
            .unwrap()
            .generate()
            .unwrap()
            .write_to_file("api.rs")
            .unwrap();
    }
}
