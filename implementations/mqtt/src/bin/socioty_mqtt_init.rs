use rand::{rngs::OsRng, Rng};
use socioty::{Key, Tdprf};
use socioty_mqtt::{InitConfig, NodeConfig, RequestConfig};
use std::{collections::HashMap, env};
use data_encoding::BASE32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: InitConfig = confy::load_path(args[1].to_owned()).unwrap();

    // let clientcount: u32 = <u32>::try_from(cfg.clients.len()).unwrap();

    let mut rng = OsRng;
    let (_, subkeys) = Tdprf::generate(&mut rng, &cfg.clients, cfg.k);

    let mut client_map = HashMap::new();

    let node_configs = cfg
        .clients
        .iter()
        .zip(subkeys)
        .map(|(c, k)| {
            let aead_key = OsRng.gen();
            let nc = NodeConfig {
                address: cfg.broker.to_string(),
                node: c.to_string(),
                tdprf_key: k.to_bytes(),
                tdprf_b32: BASE32.encode(&k.to_bytes()),
                // aead_key: OsRng.gen(),
                aead_key: aead_key,
                aead_b32: BASE32.encode(&aead_key),

            };
            client_map.insert(nc.node.to_owned(), nc.aead_key.to_owned());
            nc
        })
        .collect::<Vec<_>>();
    for (node_config, client) in node_configs.into_iter().zip(cfg.clients.iter()) {
        confy::store_path(format!("{}_config.toml", client), node_config).unwrap();
    }
    let request_key =  Key::random(&mut rng).to_bytes();
    let request_config = RequestConfig {
        address: cfg.broker.to_string(),
        clients: client_map,
        // key: Key::random(&mut rng).to_bytes(),
        key: request_key,
        key_b32: BASE32.encode(&request_key),
    };

    confy::store_path("request_config.toml", request_config).unwrap();
}
