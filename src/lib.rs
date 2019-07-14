extern crate base64;
extern crate crypto;
extern crate websocket;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde_json::{json, Value};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use websocket::message::Message;
use websocket::OwnedMessage::Text;
use websocket::{receiver, sender, ClientBuilder};

pub struct Obs {
    pub receiver: Receiver<serde_json::value::Value>,
    sender: sender::Writer<TcpStream>,
    response: Receiver<serde_json::value::Value>,
    requests: u32,
}

impl Obs {
    pub fn connect(domain: &str, password: &str) -> Obs {
        let client = ClientBuilder::new(domain)
            .unwrap()
            .connect_insecure()
            .unwrap();

        let (mut receiver, sender) = client.split().unwrap();
        let (tx, rx) = mpsc::channel();

        // channel for requests callback.
        let (restx, resrx) = mpsc::channel();

        let mut obs = Obs {
            receiver: rx,
            sender: sender,
            response: resrx,
            requests: 0,
        };

        thread::spawn(move || {
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

                    if parsed_message["message-id"] != Value::Null {
                        // callback for requests
                        restx.send(parsed_message.to_owned()).unwrap();
                    }

                    // to send back to main
                    tx.send(parsed_message.to_owned()).unwrap();
                    // println!("{}", parsed_message);
                }
            }
        });

        // Send some requests
        let res = &obs.send("GetAuthRequired".to_string(), None);

        if res["authRequired"].as_bool().unwrap() {
            let success = obs.authenticate(
                password,
                &res["challenge"].as_str().unwrap(),
                &res["salt"].as_str().unwrap(),
            );

            println!("{:?}", success);
        } else {
            println!("No authentication required");
        }

        obs
    }

    pub fn send(&mut self, request: String, fields: Option<String>) -> serde_json::value::Value {
        let mut req = json!({
            "message-id": self.requests.to_string(),
            "request-type": request
        });

        if let Some(fields) = fields {
            // turn it back into json?
            let mut a: Value = serde_json::from_str(&fields).unwrap();

            for (k, v) in a.as_object_mut().unwrap().iter() {
                req.as_object_mut()
                    .unwrap()
                    .insert(k.to_owned(), v.to_owned());
            }
        }

        self.sender
            .send_message(&Message::text(req.to_string()))
            .unwrap();

        self.requests += 1;

        // wait for response
        let response = self.response.recv().unwrap();
        println!("{}", response);

        response
    }

    fn authenticate(&mut self, password: &str, challenge: &str, salt: &str) -> Result<&str, &str> {
        // secret
        let mut secret_hash = Sha256::new();
        secret_hash.input_str(&password);
        secret_hash.input_str(&salt);

        let mut secret_bytes = vec![0; secret_hash.output_bytes()];
        secret_hash.result(&mut secret_bytes);

        let secret = base64::encode(&secret_bytes);

        // auth
        let mut auth_response_hash = Sha256::new();
        auth_response_hash.input_str(&secret);
        auth_response_hash.input_str(&challenge);

        let mut auth_response_bytes = vec![0; secret_hash.output_bytes()];
        auth_response_hash.result(&mut auth_response_bytes);

        let auth_response = base64::encode(&auth_response_bytes);

        // send
        let authenticated = &self.send(
            "Authenticate".to_string(),
            Some(json!({ "auth": auth_response }).to_string()),
        );

        if authenticated["status"] == "ok".to_string() {
            println!("Successfully authenticated");

            Ok("Successfully authenticated")
        } else {
            // handle errors
            println!("Error authenticating");

            Err("Error authenticating")
        }
    }
}
