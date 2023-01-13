use std::env;

use rand::rngs::OsRng;
use rand::Rng;

use socioty::{Key, Tdprf};
use socioty_coap::{InitConfig, NodeConfig, RequestConfig};

use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: InitConfig = confy::load_path(args[1].to_owned()).unwrap();

    let mut rng = OsRng;
    let (_, subkeys) = Tdprf::generate(&mut rng, &cfg.clients, cfg.k);


    let mut client_map = HashMap::new();
    let node_configs = cfg.clients.iter().zip(subkeys).map(|(c,k)| {
            let nc = NodeConfig {
                address: c.to_string(),
                tdprf_key: k.to_bytes(),
                aead_key: OsRng.gen(),
            };
            client_map.insert(nc.address.to_owned(), nc.aead_key.to_owned());
            nc
        }).collect::<Vec<_>>();

    for (node_config, client) in node_configs.into_iter().zip(cfg.clients.iter()) {
        confy::store_path(format!("{}_config.toml", client.replace(':',"_")), node_config).unwrap();
    }
    let request_config = RequestConfig {
        key: Key::random(&mut rng).to_bytes(),
        addresses: client_map,
    };

    match confy::store_path("request_config.toml", request_config) {
        Ok(_) => {}
        Err(e) => {println!("{:?}", e);}
    }
    
}
