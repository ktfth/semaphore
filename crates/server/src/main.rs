use std::env;

use std::ffi::{OsString};

use ipc_rpc::IpcRpc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::{thread, time};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
enum Message {
    Red, Stop,
    Yellow, Wait,
    Green, Go,
}

#[tokio::main]
pub async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .unwrap();

    let client_path = env::args().nth(1).expect("Check the path for client");

    let mut server_client_combo = IpcRpc::build()
        .finish(client_path, message_handler, |key, cmd| {
            let key: OsString = key.into();
            cmd.arg(key);
        })
        .await
        .unwrap();

    server_client_combo
        .server
        .wait_for_client_to_connect()
        .await
        .unwrap();

    log::info!("client connected!");

    server_client_combo
        .server
        .wait_for_client_to_disconnect()
        .await
        .unwrap();
}

async fn message_handler(message: Message) -> Option<Message> {
    log::info!("Message from client: {message:?}");

    let five_millis = time::Duration::from_millis(10);

    match message {
        Message::Red => {
            thread::sleep(five_millis);
            Some(Message::Stop)
        },
        Message::Yellow => {
            thread::sleep(five_millis);
            Some(Message::Wait)
        },
        Message::Green => {
            thread::sleep(five_millis);
            Some(Message::Go)
        },
        Message::Stop => None,
        Message::Wait => None,
        Message::Go => None,
    }
}