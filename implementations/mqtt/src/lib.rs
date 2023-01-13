use std::collections::HashMap;

use rand::Rng;
use data_encoding::BASE32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct InitConfig {
    pub broker: String,
    pub clients: Vec<String>,
    pub k: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfig {
    pub address: String,
    pub node: String,
    pub tdprf_key: [u8; 32],
    pub tdprf_b32: String,
    pub aead_key: [u8; 16],
    pub aead_b32: String,
}

impl ::std::default::Default for NodeConfig {
    fn default() -> Self {
        let aead_key: [u8; 16] = rand::thread_rng().gen();
        let tdprf_key = socioty::Key::random(&mut rand::thread_rng()).to_bytes();
        Self {
            address: "localhost:1883".to_string(), // 1883 is the MQTT port
            node: "node".to_string(),
            // tdprf_key: socioty::Key::random(&mut rand::thread_rng()).to_bytes(),
            tdprf_key: tdprf_key,
            tdprf_b32: BASE32.encode(&tdprf_key),
            aead_key: aead_key,
            aead_b32: BASE32.encode(&aead_key),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestConfig {
    pub address: String,
    pub key: [u8; 32],
    pub key_b32: String,
    pub clients: HashMap<String, [u8; 16]>,
}

impl ::std::default::Default for RequestConfig {
    fn default() -> Self {
        let key = socioty::Key::random(&mut rand::thread_rng()).to_bytes();
        Self {
            address: "localhost:1883".to_string(),
            clients: HashMap::new(),
            // key: socioty::Key::random(&mut rand::thread_rng()).to_bytes(),
            key: key,
            key_b32: BASE32.encode(&key),
        }
    }
}
