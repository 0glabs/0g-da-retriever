use std::error::Error;

use grpc::retriever::{retriever_client::RetrieverClient, BlobRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // enable backtraces
    std::env::set_var("RUST_BACKTRACE", "1");

    let data_root =
        &hex::decode("1111111111111111111111111111111111111111111111111111111111111111").unwrap();
    let epoch = 38;
    let quorum_id = 0;

    let mut client = RetrieverClient::connect("http://0.0.0.0:34000")
        .await
        .unwrap();

    let reply = client
        .retrieve_blob(BlobRequest {
            epoch,
            quorum_id,
            storage_root: data_root.clone(),
        })
        .await
        .unwrap();

    println!("reply: {:?}", reply);

    Ok(())
}
