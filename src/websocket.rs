pub mod protocol;

use simple_websockets::{Event, EventHub, Message, Responder};
use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::game::GameHandler;

use self::protocol::{ProtocolMessage, InitiateParameter, ParamMessageType};

pub fn start_listen(game_ref: Arc<Mutex<GameHandler>>) {
    let event_hub = simple_websockets::launch(9151).expect("failed to listen on port 9151");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    std::thread::spawn(move || {
        websocket_loop(&event_hub, &mut clients, game_ref);
    });
}

fn websocket_loop(event_hub: &EventHub, clients: &mut HashMap<u64, Responder>, game_ref: Arc<Mutex<GameHandler>>) {
    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                game_ref.lock().unwrap().ws_connected();
                println!("A client connected with id #{}", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder);
            }
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                clients.remove(&client_id);
            }
            Event::Message(client_id, message) => {
                println!(
                    "Received a message from client #{}: {:?}",
                    client_id, message
                );

                match message {
                    Message::Text(text) => {
                        let parsed_message: serde_json::Result<protocol::ProtocolMessage> =
                            serde_json::from_str(&text.to_owned());

                        match parsed_message {
                            Ok(parsed_message) => {
                                handle_ws_message(parsed_message, game_ref.clone())
                            }
                            Err(err) => {
                                println!("Error parsing message: {:?}", err);
                            }
                        }
                    }
                    Message::Binary(bin) => {
                        println!("Received binary message: {:?}", bin);
                    }
                }

                // retrieve this client's `Responder`:
                let responder = clients.get(&client_id).unwrap();
                // echo the message back:
                responder.send(Message::Text("Hello World".to_owned()));
            }
        }
    }
}


fn handle_ws_message(message: ProtocolMessage, game: Arc<Mutex<GameHandler>>) {
    match message.parameter.unwrap() {
        ParamMessageType::InitiateParameter(v) => {
            game.lock().unwrap().initiate(v);
        }
        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tungstenite::client::connect;
    use url::Url;

    #[test]
    fn test_websocket_server() {
        //start_listen();
        std::thread::sleep(std::time::Duration::from_millis(250));

        let (mut socket, response) =
            connect(Url::parse("ws://localhost:9151").unwrap()).expect("Can't connect");

        socket
            .write_message(tungstenite::Message::Text("Hello World".to_string()))
            .unwrap();

        let message = socket.read_message().unwrap();

        assert_eq!(message.to_text().unwrap(), "Hello World");
    }
}
