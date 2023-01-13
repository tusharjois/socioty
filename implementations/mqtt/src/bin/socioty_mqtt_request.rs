use ascon_aead::aead::Aead;
use ascon_aead::{Ascon128, KeyInit, Nonce};
//use rumqttc::tokio_rustls::client;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use socioty_mqtt::RequestConfig;

use socioty::{Key, PartialEval, Tdprf};
use std::env;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: RequestConfig = confy::load_path(args[1].to_owned()).unwrap();
    let timestamp: i64 = args[2].parse().unwrap();
    let counter = (timestamp / 30).to_string();

    let threshold: u32 = args[3].parse().unwrap();

    // Argument checking, clients > 0 and threshold <= clients
    if cfg.clients.len() == 0 {
        println!("number of clients is equal to 0 in {}", args[1]);
        return;
    }
    if threshold as usize > cfg.clients.len() {
        println!(
            "threshold {} is greater than the number of clients {}",
            threshold,
            cfg.clients.len()
        );
        return;
    }

    let client_count = cfg.clients.len();
    let mut responses = Vec::with_capacity(client_count);
    let mut received = Vec::with_capacity(client_count);

    let mut mqtt_options = MqttOptions::new("requestor", cfg.address, 1883);

    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

    let subtopic = format!("socioty/tdprf/{}", counter.to_string());
    client.subscribe(subtopic, QoS::AtLeastOnce).await.unwrap();

    let now = Instant::now();

    client
        .publish(
            String::from("socioty/tdprf"),
            QoS::AtLeastOnce,
            false,
            counter.to_string().as_bytes(),
        )
        .await
        .unwrap();

    while responses.len() < threshold.try_into().unwrap() {
        let notification = match event_loop.poll().await {
            Ok(v) => v,
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        };
        match notification {
            Event::Incoming(Packet::Publish(Publish { topic, payload, .. }))
                if topic == format!("socioty/tdprf/{}", counter) =>
            {
                let payload = payload.to_vec();

                // First 8 bytes of payload are the node_id
                let node_id = String::from_utf8(payload[..8].to_vec()).unwrap();

                if !received.contains(&node_id) {
                    // Next 16 bytes of the payload are the nonce
                    let nonce = Nonce::<Ascon128>::from_slice(&payload[8..24]);

                    // Get the AEAD key associated with this client
                    let aead_key =
                        ascon_aead::Key::<Ascon128>::from_slice(cfg.clients.get(&node_id).unwrap());
                    let aead_cipher = Ascon128::new(aead_key);

                    // Rest of the payload is the ciphertext
                    let plaintext = aead_cipher.decrypt(nonce, &payload[24..]).unwrap();
                    let partial = <[u8; 32]>::try_from(plaintext).unwrap();

                    responses.push(PartialEval::from_bytes(partial).unwrap());
                    received.push(node_id.to_owned());
                }
            }
            _ => {
                continue;
            }
        }
    }
    let key = Key::from_bytes(cfg.key).unwrap();
    let tdprf = Tdprf::new(key);
    let tdprf_output = tdprf.reconstruct(counter.as_bytes(), &received, &responses);
    let duration = now.elapsed();
    // println!("Duration: {:?}", now.elapsed());
    println!("TDPRF Result: {:?}", tdprf_output.to_bytes());
    println!("Duration: {:?}", duration);
}
