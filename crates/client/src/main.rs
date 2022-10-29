use std::{env};

use ipc_rpc::{rpc_call, ConnectionKey, IpcRpcClient};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
enum Message {
    Red, Stop,
    Yellow, Wait,
    Green, Go,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .unwrap();
    let key = env::args().nth(1).expect("First argument is server key");
    let client = IpcRpcClient::initialize_client(ConnectionKey::try_from(key).unwrap(), message_handler)
        .await
        .unwrap();
    let resp = client.send(Message::Green).await.unwrap();
    log::info!("Message from server: {resp:?}");
    match resp {
        Message::Go => {
            let inner_resp = client.send(Message::Yellow).await.unwrap();
            log::info!("Message from server: {inner_resp:?}")
        },
        _ => {
            log::info!("Not shure what to do with that response.");
        }
    }
    rpc_call!(sender: client, to_send: Message::Red, receiver: Message::Stop => {
        log::info!("Time to stop!");
    },)
    .unwrap()
}

async fn message_handler(_message: Message) -> Option<Message> {
    None
}