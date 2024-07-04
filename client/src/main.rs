use std::error::Error;

use clap::{arg, command};
use grpc::retriever::{retriever_client::RetrieverClient, BlobRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // enable backtraces
    std::env::set_var("RUST_BACKTRACE", "1");

    let matches = command!()
        .args(&[
            arg!(-r --"data-root" <HASH> "Set data root"),
            arg!(-e --epoch <NUM> "Set epoch").value_parser(clap::value_parser!(u64)),
            arg!(-q --"quorum-id" <ID> "Set quorum id").value_parser(clap::value_parser!(u64)),
            arg!(-s --server <URL> "Retriever server address"),
        ])
        .allow_external_subcommands(true)
        .get_matches();

    let data_root = hex::decode(
        &matches
            .get_one::<String>("data-root")
            .expect("data root must provide")[2..],
    )?;
    let epoch = *matches.get_one::<u64>("epoch").expect("epoch must provide");
    let quorum_id = *matches
        .get_one::<u64>("quorum-id")
        .expect("quorum id must provide");
    let server = matches
        .get_one::<String>("server")
        .expect("server must provide")
        .clone();

    let mut client = RetrieverClient::connect(server).await.unwrap();
    let reply = client
        .retrieve_blob(BlobRequest {
            epoch,
            quorum_id,
            storage_root: data_root,
        })
        .await
        .unwrap();

    println!("reply: {:?}", reply);

    Ok(())
}
