use std::{
    collections::HashMap,
    ffi::{c_char, CString},
    sync::Mutex,
};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use ts3plugin::{Server, TsApi, TsApiLock};

use crate::websocket::protocol::InitiateParameter;

static GAME_INSTANCES: Lazy<Mutex<HashMap<String, GameHandler>>> =
    Lazy::new(|| Mutex::from(HashMap::new()));

pub struct GameHandler {
    pub server_id: u64,
    pub own_client_id: u16,
    pub original_channel: u64,
    pub game_channel: u64,
    pub in_game: bool,
}

impl GameHandler {}

pub fn initiate_rusty_server(params: InitiateParameter) {
    let mut games_lock = GAME_INSTANCES.lock().unwrap();
    if games_lock.contains_key(&params.server_unique_identifier) {
        println!(
            "instance already exists for server uid {}",
            params.server_unique_identifier
        );
        return;
    }

    let mut api_lock = TsApi::lock_api().unwrap();
    let server = get_server_by_uid_lock(&mut api_lock, &params.server_unique_identifier);

    let server = match server {
        Ok(srv) => srv,
        _ => {
            println!("client doesnt seem to be connected to provided uid");
            return;
        }
    };

    let own_connection = server
        .get_connection(server.get_own_connection_id().unwrap())
        .unwrap();

    let inst = GameHandler {
        in_game: false,
        server_id: server.get_id().0,
        own_client_id: own_connection.get_id().0,
        original_channel: own_connection.get_channel_id().unwrap().0,
        game_channel: params.channel_id,
    };

    ts_rename_client(&inst, &api_lock, params.name);
    ts_join_channel(&inst, &api_lock);

    games_lock.insert(params.server_unique_identifier, inst);
}

pub fn get_server_by_uid<'a>(ts_api: &'a mut TsApi, uid: &String) -> Result<&'a Server> {
    let server_ids = ts_api.get_server_ids();

    for server_id in server_ids {
        let server = ts_api.get_server(server_id).unwrap();
        if *server.get_uid().unwrap() == *uid {
            return Ok(server);
        }
    }

    Err(anyhow!("No server found with uid {}", uid.to_string()))
}

pub fn get_server_by_uid_lock<'a>(ts_api: &'a mut TsApiLock, uid: &String) -> Result<&'a Server> {
    let server_ids = ts_api.get_server_ids();

    for server_id in server_ids {
        let server = ts_api.get_server(server_id).unwrap();
        if *server.get_uid().unwrap() == *uid {
            return Ok(server);
        }
    }

    Err(anyhow!("No server found with uid {}", uid.to_string()))
}

pub fn ts_rename_client(game: &GameHandler, ts_api: &TsApiLock, nick: String) {
    unsafe {
        let raw_api: &ts3plugin::Ts3Functions = ts_api.get_raw_api();
        let server_id = game.server_id;
        let name: *const c_char = CString::new(nick).unwrap().into_raw();
        (raw_api.set_client_self_variable_as_string)(server_id, 1, name);
    }
}

pub fn ts_join_channel(game: &GameHandler, ts_api: &TsApiLock) {
    unsafe {
        let raw_api: &ts3plugin::Ts3Functions = ts_api.get_raw_api();
        let c: i8 = 0;
        (raw_api.request_client_move)(
            game.server_id,
            game.own_client_id,
            game.game_channel,
            &c,
            &c,
        );
    }
}
