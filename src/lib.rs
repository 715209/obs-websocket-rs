extern crate base64;
extern crate crypto;
extern crate websocket;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use serde_json::json;
use serde_json::Value;
use std::net::TcpStream;
use std::str;
use websocket::client::sync::Client;
use websocket::OwnedMessage::Text;
use websocket::{ClientBuilder, Message};

pub struct Obs {
    pub client: Client<TcpStream>,
    requests: usize,
}

impl Obs {
    pub fn new(domain: &str, password: &str) {
        let client = ClientBuilder::new(domain)
            .unwrap()
            .connect_insecure()
            .unwrap();

        let (mut receiver, mut sender) = client.split().unwrap();

        // send auth request
        // sender.send_message(&Message::text(r#"{"message-id": "1", "request-type": "GetAuthRequired"}"#)).unwrap();
        sender
            .send_message(&Message::text(
                json!({
                    "message-id": "1",
                    "request-type": "GetAuthRequired"
                })
                .to_string(),
            ))
            .unwrap();

        for message in receiver.incoming_messages() {

            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Received an error? {}", e);
                    return;
                }
            };

            if let Text(m) = message {
                let parsed_message: Value = serde_json::from_str(&m).unwrap();
                println!("{}", parsed_message);

                if parsed_message["message-id"] == "1" {
                    if parsed_message["authRequired"] == true {
                        let challenge = &parsed_message["challenge"].as_str().unwrap();
                        let salt = &parsed_message["salt"].as_str().unwrap();

                        // secret
                        let mut secret_hash = Sha256::new();
                        secret_hash.input_str(&password);
                        secret_hash.input_str(&salt);

                        let mut secret_bytes = vec![0; secret_hash.output_bytes()];
                        secret_hash.result(&mut secret_bytes);

                        let secret = base64::encode(&secret_bytes);

                        println!("Secret: {}", secret);

                        // auth
                        let mut auth_response_hash = Sha256::new();
                        auth_response_hash.input_str(&secret);
                        auth_response_hash.input_str(&challenge);

                        let mut auth_response_bytes = vec![0; secret_hash.output_bytes()];
                        auth_response_hash.result(&mut auth_response_bytes);

                        let auth_response = base64::encode(&auth_response_bytes);

                        println!("{}", auth_response);

                        sender
                            .send_message(&Message::text(
                                json!({
                                    "message-id": "2",
                                    "request-type": "Authenticate",
                                    "auth": auth_response
                                })
                                .to_string(),
                            ))
                            .unwrap();
                    } else {
                        println!("No authentication requiered");
                    }
                }
            }
        }

        // Obs {
        //     client,
        //     requests: 2,
        // }
    }

    pub fn send() {
        // todo
    }
}