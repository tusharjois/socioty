use coap::Server;
use coap_lite::RequestType as Method;
use socioty_coap::NodeConfig;
use rand::{rngs::OsRng, Rng};
use ascon_aead::aead::{Aead, KeyInit};
use ascon_aead::{Ascon128, Nonce};
use ascon_aead::Key as aKey;


use std::env;

use socioty::{Key, Tdprf};

async fn handle_requests(addr: &str, tdprf_key: [u8; 32], aead_key: [u8; 16]) {
    let mut server = Server::new(addr).unwrap();

    // TODO: AEAD. Follow the same pattern as benchmark_eval.
    // Key can be the same for each node (demo), or generated with init (robust)
    // Each response should include the Tdprf output with the nonce
    // The response should be decrypted and verified in request

    server
        .run(|request| async {

            let random_nonce: [u8; 16] = OsRng.gen();
            let nonce = Nonce::<Ascon128>::from_slice(&random_nonce);

            let mut response = b"Err".to_vec();
            let path = match request.get_method() {
                &Method::Get => request.get_path_as_vec().ok(),
                _ => None,
            };

            if let Some(p) = path {
                if let Some(s) = p.get(1) {
                    let cipherkey = aKey::<Ascon128>::from_slice(&aead_key);
                    let cipher = Ascon128::new(cipherkey); 
                    let plaintext = Tdprf::new(Key::from_bytes(tdprf_key).unwrap())
                        .partial_eval(s.as_bytes())
                        .to_bytes()
                        .to_vec();
                    let mut ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap();
                    response = nonce.to_vec();
                    response.append(&mut ciphertext);
                }
            }

            return match request.response {
                Some(mut message) => {
                    message.message.payload = response;
                    Some(message)
                }
                _ => None,
            };
        })
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: NodeConfig = confy::load_path(args[1].to_owned()).unwrap();

    handle_requests(&cfg.address, cfg.tdprf_key, cfg.aead_key).await;
}
