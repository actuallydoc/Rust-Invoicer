use std::{fmt::Debug, fs};

use discord_rpc_client::Client;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq)]
pub enum Status {
    CONNECTED,
    DISCONNECTED,
    CLEARED,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientId {
    client_id: u64,
}

impl ClientId {
    fn new() -> Self {
        //fs::read_to_string("rpc.json").expect("Something went wrong reading the file");
        let content = fs::read_to_string("rpc.json").unwrap();
        let serialized: ClientId = serde_json::from_str(&content).unwrap();
        //println!("Client id: {}", serialized.client_id);
        Self {
            client_id: serialized.client_id,
        }
    }
}
impl Debug for DiscordRPC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscordRPC")
            .field("status", &self.status)
            .finish()
    }
}

pub struct DiscordRPC {
    pub client: Client,
    pub status: Status,
}
pub trait RPC {
    fn new() -> DiscordRPC;
}

impl RPC for DiscordRPC {
    fn new() -> Self {
        Self {
            client: Client::new(ClientId::new().client_id),
            status: Status::CONNECTED,
        }
    }
}
