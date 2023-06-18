pub mod protocol;

use anyhow::{anyhow, Ok, Result};
use simple_websockets::{Event, EventHub, Message, Responder};
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use ts3plugin::{ClientProperties, ServerId};

use self::protocol::{
    Command, InitiateParameter, ParamMessageType, PlayerStateUpdateParameter, PluginStateParameter,
    ProtocolMessage, SelfStateUpdateParameter, SoundStateParameter, TalkStateParameter,
};

const FAKE_SALTY_VERSION: &str = "2.3.6";

struct InstanceState {
    instances: HashMap<String, InitiateParameter>,
    self_state_by_instance: HashMap<String, SelfStateUpdateParameter>,
    player_states_by_instance: HashMap<String, Vec<PlayerStateUpdateParameter>>,
}

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<u64, Responder>> = Mutex::from(HashMap::new());
    static ref CLIENTS_BY_INSTANCE: Mutex<HashMap<String, u64>> = Mutex::from(HashMap::new());
}
use crate::game::GameHandler;

pub fn start_listen(game_ref: Arc<Mutex<GameHandler>>) {
    let event_hub = simple_websockets::launch(9151).expect("failed to listen on port 9151");

    std::thread::spawn(move || {
        websocket_loop(&event_hub, game_ref);
    });
}

fn websocket_loop(event_hub: &EventHub, game_ref: Arc<Mutex<GameHandler>>) {
    let mut instance_state = InstanceState {
        instances: HashMap::new(),
        self_state_by_instance: HashMap::new(),
        player_states_by_instance: HashMap::new(),
    };
    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                game_ref.lock().unwrap().ws_connected();
                println!("A client connected with id #{}", client_id);
                handle_connect(&responder);
                CLIENTS.lock().unwrap().insert(client_id, responder);
            }
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                CLIENTS.lock().unwrap().remove(&client_id);
            }
            Event::Message(client_id, message) => {
                let clients_locked = CLIENTS.lock().unwrap();
                println!(
                    "Received a message from client #{}: {:?}",
                    client_id, message
                );

                match message {
                    Message::Text(text) => {
                        let parsed_message: serde_json::Result<protocol::ProtocolMessage> =
                            serde_json::from_str(&text.to_owned());

                        match parsed_message {
                            std::result::Result::Ok(parsed_message) => match parsed_message.command
                            {
                                Command::Initiate => {
                                    CLIENTS_BY_INSTANCE.lock().unwrap().insert(
                                        parsed_message.server_unique_identifier.clone().unwrap(),
                                        client_id,
                                    );
                                    handle_init(
                                        parsed_message.parameter.unwrap(),
                                        &mut instance_state.instances,
                                    );
                                }
                                Command::Ping => {
                                    handle_ping(
                                        parsed_message,
                                        CLIENTS.lock().unwrap().get(&client_id).unwrap(),
                                    );
                                }
                                Command::SelfStateUpdate => {
                                    handle_self_state_update(
                                        parsed_message.parameter.unwrap(),
                                        &parsed_message.server_unique_identifier.unwrap(),
                                        &mut instance_state.self_state_by_instance,
                                    );
                                }
                                Command::PlayerStateUpdate => {
                                    handle_player_state_update(
                                        parsed_message.parameter.unwrap(),
                                        &parsed_message.server_unique_identifier.unwrap(),
                                        &mut instance_state.player_states_by_instance,
                                    );
                                }
                                Command::BulkUpdate => {
                                    handle_bulk_update(
                                        parsed_message.parameter.unwrap(),
                                        &parsed_message.server_unique_identifier.unwrap(),
                                        &mut instance_state,
                                    );
                                }
                                Command::RemovePlayer => {
                                    handle_remove_player(
                                        parsed_message.parameter.unwrap(),
                                        &parsed_message.server_unique_identifier.unwrap(),
                                        &mut instance_state,
                                    );
                                }
                                Command::PlaySound => {
                                    handle_sound_play(parsed_message.parameter.unwrap());
                                }
                                Command::StopSound => {
                                    handle_sound_stop(parsed_message.parameter.unwrap());
                                }
                                Command::PhoneCommunicationUpdate => {
                                    handle_phone_communication_update(
                                        parsed_message.parameter.unwrap(),
                                    );
                                }
                                Command::StopPhoneCommunication => {
                                    handle_phone_call_end(parsed_message.parameter.unwrap());
                                }
                                Command::RadioCommunicationUpdate => {
                                    handle_radio_communication_update(
                                        parsed_message.parameter.unwrap(),
                                    );
                                }
                                Command::StopRadioCommunication => {
                                    handle_radio_stop(parsed_message.parameter.unwrap());
                                }
                                Command::RadioTowerUpdate => {
                                    handle_radio_tower_update(parsed_message.parameter.unwrap());
                                }
                                Command::AddRadioChannelMember => {
                                    handle_radio_channel_add(parsed_message.parameter.unwrap());
                                }
                                Command::UpdateRadioChannelMembers => {
                                    handle_radio_channel_update(parsed_message.parameter.unwrap());
                                }
                                Command::RemoveRadioChannelMember => {
                                    handle_radio_channel_remove(parsed_message.parameter.unwrap());
                                }
                                Command::MegaphoneCommunicationUpdate => {
                                    handle_megaphone_update(parsed_message.parameter.unwrap());
                                }
                                Command::StopMegaphoneCommunication => {
                                    handle_megaphone_stop(parsed_message.parameter.unwrap());
                                }
                                _ => {}
                            },
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
                let responder = clients_locked.get(&client_id).unwrap();
                // echo the message back:
                responder.send(Message::Text("Hello World".to_owned()));
            }
        }
    }
}

fn handle_connect(responder: &Responder) {
    let message = protocol::ProtocolMessage {
        command: Command::PluginState,
        server_unique_identifier: None,
        parameter: Some(ParamMessageType::PluginStateParameter(
            PluginStateParameter {
                version: FAKE_SALTY_VERSION.to_owned(),
                active_instances: 1,
            },
        )),
    };

    responder.send(Message::Text(serde_json::to_string(&message).unwrap()));
}

fn handle_init(message: ParamMessageType, instance_state: &mut HashMap<String, InitiateParameter>) {
    if let ParamMessageType::InitiateParameter(initiate_parameter) = message {
        if instance_state.contains_key(&initiate_parameter.server_unique_identifier) {
            instance_state.remove(&initiate_parameter.server_unique_identifier);
        }

        instance_state.insert(
            initiate_parameter.server_unique_identifier.to_owned(),
            initiate_parameter,
        );
    }
}

fn handle_ping(message: ProtocolMessage, responder: &Responder) {
    let message = protocol::ProtocolMessage {
        command: Command::Pong,
        server_unique_identifier: message.server_unique_identifier,
        parameter: None,
    };

    responder.send(Message::Text(serde_json::to_string(&message).unwrap()));
}

pub fn handle_self_state_update(
    message: ParamMessageType,
    server_id: &String,
    self_state_by_instance: &mut HashMap<String, SelfStateUpdateParameter>,
) {
    if let ParamMessageType::SelfStateUpdateParameter(self_state_update_parameter) = message {
        self_state_by_instance.insert(server_id.to_owned(), self_state_update_parameter);
    }
}

pub fn handle_player_state_update(
    message: ParamMessageType,
    server_id: &String,
    player_states_by_instance: &mut HashMap<String, Vec<PlayerStateUpdateParameter>>,
) {
    if let ParamMessageType::PlayerStateUpdateParameter(player_state_update_parameter) = message {
        let players = player_states_by_instance.get(server_id).unwrap();
        let mut new_players: Vec<PlayerStateUpdateParameter> = players.to_vec();

        let player = players
            .iter()
            .find(|player| player.name == player_state_update_parameter.name);

        match player {
            Some(player) => {
                let index = new_players
                    .iter()
                    .position(|x| x.name == player.name)
                    .unwrap();
                new_players.remove(index);
                new_players.push(player_state_update_parameter);
            }
            None => {
                new_players.push(player_state_update_parameter);
            }
        }

        player_states_by_instance.insert(server_id.to_owned(), new_players);
    }
}

fn handle_bulk_update(
    message: ParamMessageType,
    server_id: &String,
    instance_state: &mut InstanceState,
) {
    if let ParamMessageType::BulkUpdateParameter(bulk_message) = message {
        instance_state
            .self_state_by_instance
            .insert(server_id.to_owned(), bulk_message.self_state);

        instance_state
            .player_states_by_instance
            .insert(server_id.to_owned(), bulk_message.player_states.to_owned());
    }
}

fn handle_remove_player(
    message: ParamMessageType,
    server_id: &String,
    instance_state: &mut InstanceState,
) {
    if let ParamMessageType::RemovePlayerParameter(remove_player_param) = message {
        let players_cloned = instance_state
            .player_states_by_instance
            .get(server_id)
            .unwrap()
            .iter()
            .filter(|player| player.name != remove_player_param.name)
            .cloned()
            .collect::<Vec<PlayerStateUpdateParameter>>();

        instance_state
            .player_states_by_instance
            .insert(server_id.to_owned(), players_cloned.to_owned());
    }
}

fn handle_phone_communication_update(message: ParamMessageType) {
    if let ParamMessageType::PhoneCommunicationUpdateParameter(_phone_communication_update) = message
    {
        // Handle phone communication update
    }
}

fn handle_phone_call_end(message: ParamMessageType) {
    if let ParamMessageType::StopPhoneCommunicationParameter(_phone_call_end) = message {
        // Handle phone communication end
    }
}

fn handle_radio_communication_update(message: ParamMessageType) {
    if let ParamMessageType::RadioCommunicationUpdateParameter(_radio_update_param) = message {
        // Handle phone communication update
    }
}

fn handle_radio_stop(message: ParamMessageType) {
    if let ParamMessageType::StopRadioCommunicationParameter(_radio_call_end) = message {
        // Handle phone communication end
    }
}

fn handle_radio_tower_update(message: ParamMessageType) {
    if let ParamMessageType::RadioTowerUpdateParameter(_radio_tower_update) = message {
        // Handle radio tower update
    }
}

fn handle_radio_channel_add(message: ParamMessageType) {
    if let ParamMessageType::AddRadioChannelMemberParameter(_radio_cannel_member_add) = message {
        // Handle add radio channel member
    }
}

fn handle_radio_channel_update(message: ParamMessageType) {
    if let ParamMessageType::UpdateRadioChannelMembersParameter(_radio_channel_update) = message {
        // Handle add radio channel update
    }
}

fn handle_radio_channel_remove(message: ParamMessageType) {
    if let ParamMessageType::RemoveRadioChannelMemberParameter(_radio_cannel_member_remove) = message
    {
        // Handle remove radio channel member
    }
}

fn handle_megaphone_update(message: ParamMessageType) {
    if let ParamMessageType::MegaphoneCommunicationUpdateParameter(_megaphone_update) = message {
        // Handle megaphone update
    }
}

fn handle_megaphone_stop(message: ParamMessageType) {
    if let ParamMessageType::MegaphoneCommunicationUpdateParameter(_megaphone_stop) = message {
        // Handle megaphone stop
    }
}

fn handle_sound_play(message: ParamMessageType) {
    if let ParamMessageType::PlaySoundParameter(_play_sound) = message {
        // Handle play sound
    }
}

fn handle_sound_stop(message: ParamMessageType) {
    if let ParamMessageType::StopSoundParameter(_stop_sound) = message {
        // Handle stop sound
    }
}

//Events RustyChat Outgoing to be handled by the plugin:
// 1. SoundState (on mic and speaker toggle) X
// 2. TalkState (on start and on stop talking)
// 3. RadioTrafficState (Sent by the plugin when radio traffic is received, breaks up or ends.)

pub fn on_sound_state_toggle(
    server_id: &String,
    is_microphone_enabled: bool,
    is_microphone_muted: bool,
    is_sound_enabled: bool,
    is_sound_muted: bool,
) -> Result<()> {
    let clients_by_instance_locked = CLIENTS_BY_INSTANCE.lock().unwrap();

    let sound_state_message = ParamMessageType::SoundStateParameter(SoundStateParameter {
        is_microphone_enabled,
        is_microphone_muted,
        is_sound_enabled,
        is_sound_muted,
    });

    let message = ProtocolMessage {
        command: Command::SoundState,
        server_unique_identifier: Some(server_id.to_owned()),
        parameter: Some(sound_state_message),
    };

    let message = serde_json::to_string(&message)?;

    let client_id = clients_by_instance_locked.get(server_id).ok_or(anyhow!(
        "ws client for server {} not found in list",
        server_id
    ))?;

    CLIENTS
        .lock()
        .unwrap()
        .get(&client_id)
        .ok_or(anyhow!(
            "responder for client {} not found in list",
            client_id
        ))?
        .send(Message::Text(message));

    Ok(())
}

pub fn on_talk_state_toggle(server_id: &String, is_talking: bool, name: &str) -> Result<()> {
    let clients_by_instance_locked = CLIENTS_BY_INSTANCE.lock().unwrap();
    let talk_state_message = ParamMessageType::TalkStateParameter(TalkStateParameter {
        is_talking,
        name: name.to_owned(),
    });

    let message = ProtocolMessage {
        command: Command::TalkState,
        server_unique_identifier: Some(server_id.to_owned()),
        parameter: Some(talk_state_message),
    };

    let message = serde_json::to_string(&message)?;

    let client_id = clients_by_instance_locked
        .get(server_id)
        .ok_or(anyhow!("err"))?;

    CLIENTS
        .lock()
        .unwrap()
        .get(&client_id)
        .ok_or(anyhow!("not connected"))?
        .send(Message::Text(message));

    Ok(())
}

pub fn on_self_variable_update(
    _server_id: ServerId,
    flag: ClientProperties,
    _old_value: String,
    _new_value: String,
) {
    match flag {
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use tungstenite::client::connect;
    use url::Url;

    #[test]
    fn test_websocket_server() {
        //start_listen();
        std::thread::sleep(std::time::Duration::from_millis(250));

        let (mut socket, _response) =
            connect(Url::parse("ws://localhost:9151").unwrap()).expect("Can't connect");

        socket
            .write_message(tungstenite::Message::Text("Hello World".to_string()))
            .unwrap();

        let message = socket.read_message().unwrap();

        assert_eq!(message.to_text().unwrap(), "Hello World");
    }
}
