use simple_websockets::{Event, EventHub, Responder};
use std::collections::HashMap;

pub fn start_listen() {
    let event_hub = simple_websockets::launch(9151).expect("failed to listen on port 9151");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    std::thread::spawn(move || {
        websocket_loop(&event_hub, &mut clients);
    });
}

fn websocket_loop(event_hub: &EventHub, clients: &mut HashMap<u64, Responder>) {
    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
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
                // retrieve this client's `Responder`:
                let responder = clients.get(&client_id).unwrap();
                // echo the message back:
                responder.send(message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tungstenite::client::connect;
    use url::Url;

    #[test]
    fn test_websocket_server() {
        start_listen();
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
