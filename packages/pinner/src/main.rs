//! The Ws transport allows you to send JSON-RPC requests and receive responses over
//! [WebSocket](https://en.wikipedia.org/wiki/WebSocket).
//!
//! This allows to interact with the network in real-time without the need for HTTP
//! polling.

mod command_line_parser;
mod pinner;
mod read_settings;

use crate::command_line_parser::*;
use crate::pinner::pinner;
use crate::read_settings::*;
use ethers::prelude::*;
use foundry_contracts::mts_controller::RemovedResturantFilter;
use foundry_contracts::mts_controller::{
    AddNewResturantFilter, MTSController, MTSControllerEvents,
};
use foundry_contracts::resturant_token::{
    ResturantToken, ResturantTokenEvents, ReviewPostedFilter, TransferFilter,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let (network, controller_address) = parse_args();
    let settings = read_settings();
    let mut streams = HashMap::new();
    let ws_url = &settings[&network];

    let provider = Provider::<Ws>::connect(ws_url).await?;

    let client = Arc::new(provider);

    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(pinner(rx));

    let mts_controller_contract =
        MTSController::new(controller_address.parse::<Address>()?, client.clone());

    let controller_events = mts_controller_contract.events().from_block(0);
    let mut stream = controller_events.subscribe().await?;
    while let Some(Ok(event)) = stream.next().await {
        match event {
            MTSControllerEvents::AddNewResturantFilter(AddNewResturantFilter {
                id: _,
                new_resturant_address: resturant_address,
                resturant_name: _,
            }) => {
                println!("New resturant added with address: {:?}", resturant_address);
                let new_resturant_contract = ResturantToken::new(resturant_address, client.clone());
                let resturant_listener = tokio::spawn(listen_to_resturant_events(
                    new_resturant_contract,
                    tx.clone(),
                ));
                streams.insert(resturant_address, resturant_listener);
            }
            MTSControllerEvents::RemovedResturantFilter(RemovedResturantFilter {
                id: _,
                resturant_address,
                resturant_name: _,
            }) => {
                let resturant_listener = streams.get(&resturant_address);
                match resturant_listener {
                    // The division was valid
                    Some(resturant_listener) => resturant_listener.abort(),
                    // The division was invalid
                    None => println!("Already not listening"),
                }
                println!("Resturant {:?} removed", resturant_address);
            }
            _ => {}
        }
    }

    Ok(())
}

async fn listen_to_resturant_events(
    resturant: ResturantToken<Provider<Ws>>,
    tx: Sender<pinner::Action>,
) -> eyre::Result<()> {
    let resturant_events = resturant.events().from_block(0);

    let mut stream = resturant_events.subscribe().await?;

    println!("Listening on resturant: {:?}", resturant.address());

    while let Some(Ok(event)) = stream.next().await {
        let zero_address = Address::zero();
        match event {
            ResturantTokenEvents::ReviewPostedFilter(ReviewPostedFilter {
                token_id,
                review_uri,
            }) => {
                println!("Added review for Token with id: {:?}", token_id);
                println!("Added review for Token with URI: {:?}", review_uri);
            }
            ResturantTokenEvents::TransferFilter(TransferFilter {
                from: _,
                to,
                token_id,
            }) if to == resturant.address() => {
                // pin
                let token_uri = resturant.token_uri(token_id).call().await;
                match token_uri {
                    Ok(token_uri) => {
                        println!("Added new Token with token_uri: {:?}", token_uri);
                        let _ = tx.send(pinner::Action::Add(token_uri)).await;
                    }
                    Err(e) => {
                        println!("Error retriving URI for token_id {:?}", token_id);
                        println!("Error: {:?}", e);
                    }
                }
                println!("Added new Token with id: {:?}", token_id);
            }
            ResturantTokenEvents::TransferFilter(TransferFilter {
                from: _,
                to,
                token_id,
            }) if to == zero_address => {
                // unpin
                let token_uri = resturant.token_uri(token_id).call().await;
                match token_uri {
                    Ok(token_uri) => {
                        println!("Remove Token with token_uri: {:?}", token_uri);
                        let _ = tx.send(pinner::Action::Remove(token_uri)).await;
                    }
                    Err(e) => {
                        println!("Error retriving URI for token_id {:?}", token_id);
                        println!("Error: {:?}", e);
                    }
                }
                println!("Burned Token with id: {:?}", token_id);
            }
            _ => {}
        }
    }
    Ok(())
}
