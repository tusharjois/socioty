use coap::CoAPClient;
use socioty::{Key, PartialEval, Tdprf};
use socioty_coap::RequestConfig;
use tokio::sync::mpsc;
use ascon_aead::aead::{Aead, KeyInit};
use ascon_aead::{Ascon128, Nonce};
use ascon_aead::Key as aKey;

use std::env;
use std::time::Instant;

async fn make_request(url: &str) -> Result<Vec<u8>, std::io::Error> {
    let response = CoAPClient::get(url)?;
    return Ok(response.message.payload);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: RequestConfig = confy::load_path(args[1].to_owned()).unwrap();
    let timestamp: i64 = args[2].parse().unwrap();
    let counter = timestamp / 30;

    let threshold: u32 = args[3].parse().unwrap();
    let num_addrs = cfg.addresses.len();
    if num_addrs == 0 {
        println!("no addresses found in {}", args[1]);
        return;
    }
    if threshold as usize > num_addrs {
        println!(
            "threshold {} is greater than the number of clients {}",
            threshold,
            num_addrs
        );
        return;
    }

    let now = Instant::now();

    let (tx, mut rx) = mpsc::channel(num_addrs);
    
    for (addr,_) in cfg.addresses.clone() {
        let tx = tx.clone();
        tokio::spawn(async move {
            let url = format!("coap://{}/socioty/{}", addr, counter);
            let response = make_request(&url).await;
            //tx.send((url, response)).await.unwrap();
            let _ = tx.try_send((url, response));
        });
    }

    // Get rid of our copy of the Sender
    drop(tx);

    let mut responses = Vec::with_capacity(num_addrs);
    let mut clients = Vec::with_capacity(num_addrs);

    while let Some((addr, r)) = rx.recv().await {
        match r {
            Ok(v) => {
                let mut value = v.clone();

                let client_id = addr.split("/").nth(2).unwrap().to_string();

                let aead_key = aKey::<Ascon128>::from_slice(cfg.addresses.get(&client_id).unwrap());
                let aead_cipher = Ascon128::new(aead_key);
                let ciphertext = value.split_off(16);
                let nonce = Nonce::<Ascon128>::from_slice(&value);
                let plaintext = aead_cipher.decrypt(nonce, ciphertext.as_slice())
                    .expect("decryption failure!");

                responses.push(plaintext);
                clients.push(client_id);
                // println!(
                //     "{} response {:?}",
                //     clients.last().unwrap(),
                //     responses.last().unwrap()
                // );
            }
            Err(e) => println!("{} ERROR {}", addr, e),
        }
        if clients.len() >= threshold as usize {
            break;
        }
    }
    // println!("Responses: {:?}", responses);

    let partial_outputs = responses
        .into_iter()
        .map(|v| PartialEval::from_bytes(v.try_into().unwrap()).unwrap())
        .collect::<Vec<_>>();

    let key = Key::from_bytes(cfg.key).unwrap();
    let tdprf = Tdprf::new(key);
    let tdprf_output = tdprf.reconstruct(format!("{}", counter).as_bytes(), &clients, &partial_outputs);
    let duration = now.elapsed();
    println!("TDPRF Result: {:?}", tdprf_output.to_bytes());
    println!("Duration: {:?}", duration);
}
