use ascon_aead::aead::Aead;
use ascon_aead::{Ascon128, KeyInit, Nonce};
use rand::{thread_rng, Rng};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};

use socioty_mqtt::NodeConfig;
use std::env;
use std::time::Duration;

use socioty::Tdprf;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg: NodeConfig = confy::load_path(args[1].to_owned()).unwrap();

    let tdprf_key = socioty::Key::from_bytes(cfg.tdprf_key).unwrap();
    let tdprf = Tdprf::new(tdprf_key);

    let aead_key = ascon_aead::Key::<Ascon128>::from_slice(&cfg.aead_key);
    let aead_cipher = Ascon128::new(aead_key);

    let nodetopic = String::from("socioty/tdprf");
    let node_id = cfg.node.clone();
    let mut mqttoptions = MqttOptions::new(cfg.node, cfg.address, 1883); // need pull host from config file
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(nodetopic, QoS::AtLeastOnce).await.unwrap();

    // TODO: AEAD. Follow the same pattern as benchmark_eval.
    // Key can be the same for each node (demo), or generated with init (robust)
    // Each response should include the Tdprf output with the nonce
    // The response should be decrypted and verified in request

    loop {
        let notification = match eventloop.poll().await {
            Ok(v) => v,
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        };
        match notification {
            Event::Incoming(Packet::Publish(Publish { topic, payload, .. }))
                if topic == "socioty/tdprf" =>
            {
                let p = tdprf.partial_eval(payload.as_ref()).to_bytes(); // Not pretty but need to go from Bytes struct to array
                let nonce_bytes: [u8; 16] = thread_rng().gen();
                let nonce = Nonce::<Ascon128>::from_slice(&nonce_bytes); // 128-bits; unique per message

                // let mut full_payload = [0u8; 40];
                // full_payload[..node_id.len()].copy_from_slice(node_id.as_bytes());
                // full_payload[node_id.len()..].copy_from_slice(&p);

                let mut ciphertext = aead_cipher.encrypt(nonce, p.as_ref()).unwrap();
                let mut response = node_id.as_bytes().to_vec();
                response.append(&mut nonce.to_vec());
                response.append(&mut ciphertext);

                let pub_topic = format!("socioty/tdprf/{}", std::str::from_utf8(&payload).unwrap());
                client
                    .publish(pub_topic, QoS::AtLeastOnce, false, response)
                    .await
                    .unwrap();
            }
            _ => {
                continue;
            }
        }
    }
}
