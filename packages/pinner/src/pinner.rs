use async_recursion::async_recursion;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::str;
use tokio::sync::mpsc::Receiver;

type URI = String;
pub enum Action {
    Add(URI),
    Remove(URI),
}

pub async fn pinner(mut rx: Receiver<Action>) {
    let client = IpfsClient::default();
    while let Some(action) = rx.recv().await {
        match action {
            Action::Add(uri) => {
                let _ = pin_metadata_recursivly(&client, &uri).await;
            }
            Action::Remove(uri) => {
                let _ = unpin_metadata_recursivly(&client, &uri).await;
            }
        }
    }
}

async fn pin_metadata_recursivly(client: &IpfsClient, uri: &str) -> eyre::Result<()> {
    pin_or_unpin_metadata_recursivly(&client, &uri, &false).await
}

async fn unpin_metadata_recursivly(client: &IpfsClient, uri: &str) -> eyre::Result<()> {
    pin_or_unpin_metadata_recursivly(&client, &uri, &true).await
}

#[async_recursion]
async fn pin_or_unpin_metadata_recursivly(
    client: &IpfsClient,
    uri: &str,
    rm: &bool,
) -> eyre::Result<()> {
    let cid = uri.strip_prefix("ipfs://").unwrap_or(uri);

    // get file and find other uris in that file
    match client
        .get(cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            match str::from_utf8(&res) {
                Ok(res) => {
                    let uris = find_uris(res);

                    for uri in uris {
                        let _ = pin_or_unpin_metadata_recursivly(client, uri, rm).await;
                    }
                }
                Err(_) => println!("cid {:?} is not a utf8", cid),
            };
        }
        Err(e) => eprintln!("error getting file: {}", e),
    };

    if *rm {
        match client.pin_rm(cid, true).await {
            Ok(r) => {
                println!("pinned cid: {:?}", cid);
                println!("result: {:?}", r);
            }
            Err(e) => println!("error {:?} in unpinning cid: {:?}", e, cid),
        };
    } else {
        match client.pin_add(cid, true).await {
            Ok(r) => {
                println!("pinned cid: {:?}", cid);
                println!("result: {:?}", r);
            }
            Err(e) => println!("error {:?} in pinning cid: {:?}", e, cid),
        };
    }

    Ok(())
}

fn find_uris(string: &str) -> Vec<&str> {
    let mut uris = Vec::new();
    let mut remaining_string = string;
    while let Some(start_index) = remaining_string.find("ipfs://") {
        let substring = &remaining_string[start_index..];
        let end_index = substring.find('"').unwrap_or(substring.len());
        let uri = &substring[..end_index];
        uris.push(uri);
        remaining_string = &substring[end_index..];
    }

    uris
}
