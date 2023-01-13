use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct InitConfig {
    pub clients: Vec<String>,
    pub k: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfig {
    pub address: String,
    pub tdprf_key: [u8; 32],
    pub aead_key: [u8; 16],
}
impl ::std::default::Default for NodeConfig {
    fn default() -> Self {
        let aead_key: [u8; 16] = rand::thread_rng().gen();
        Self {
            address: "localhost:5683".to_string(),
            tdprf_key: socioty::Key::random(&mut rand::thread_rng()).to_bytes(),
            aead_key: aead_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RequestConfig {
    pub key: [u8; 32],
    pub addresses: HashMap<String, [u8; 16]>,
}

impl ::std::default::Default for RequestConfig {
    fn default() -> Self {
        let aead_key: [u8; 16] = rand::thread_rng().gen();
        Self {
            addresses: HashMap::from([("localhost:5683".to_string(), aead_key)]),
            key: socioty::Key::random(&mut rand::thread_rng()).to_bytes(),
        }
    }
}
