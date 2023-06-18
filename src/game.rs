use std::{
    ffi::{c_char, CString},
};

use ts3plugin::{ChannelId, ConnectionId, ServerId, TsApi, Visibility, TsApiLock};

use crate::websocket::protocol::InitiateParameter;

pub struct GameHandler {
    pub server_id: Option<u64>,
    pub own_client_id: Option<u16>,
    pub original_channel: Option<u16>,
    pub game_channel: Option<u16>,
    pub in_game: bool,
}

impl GameHandler {
    pub fn initiate(&mut self, params: InitiateParameter) {
        println!(
            "initiate rustychat for server {}",
            params.server_unique_identifier
        );

        let ts_api = TsApi::lock_api().unwrap();
        let servers = ts_api.get_server_ids();

        for server in servers {
            if *ts_api.get_server(server).unwrap().get_uid().unwrap()
                == params.server_unique_identifier
            {
                self.server_id = Some(server.0);
            }
        }

        let server = match self.server_id {
            Some(id) => ts_api
                .get_server(ServerId(id))
                .unwrap(),
            None => {
                println!("no server found matching the uid of the client request");
                return;
            }
        };

        if *server.get_uid().unwrap() != *params.server_unique_identifier {
            println!("this is the wrong server.");
            return;
        }

        let own_client = server.get_own_connection_id().unwrap();

        self.own_client_id = Some(own_client.0);

        let server_id = server.get_id().0;

        self.ts_rename_client(&ts_api, params.name);
        self.ts_join_channel(&ts_api, params.channel_id);

    }

    pub fn ws_connected(&mut self) {
        TsApi::lock_api().unwrap().log_or_print(
            "WS Connected!",
            "RustyChat",
            ts3plugin::LogLevel::Info,
        )
    }

    // this functions should be integrated into rust ts3 plugin via PR
    pub fn ts_rename_client(&mut self, ts_api: &TsApiLock, nick: String) {
        unsafe {
            let raw_api: &ts3plugin::Ts3Functions = ts_api.get_raw_api();
            let server_id = self.server_id.unwrap();
            let name: *const c_char = CString::new(nick).unwrap().into_raw();
            (raw_api.set_client_self_variable_as_string)(server_id, 1, name);
        }
    }

    pub fn ts_join_channel(&mut self, ts_api: &TsApiLock, channel_id: u64) {
        unsafe {
            let raw_api: &ts3plugin::Ts3Functions = ts_api.get_raw_api();
            let c: i8 = 0;
            (raw_api.request_client_move)(self.server_id.unwrap(), self.own_client_id.unwrap(), channel_id, &c, &c);
        }
    }

    pub fn ts_on_channel_switched(
        &mut self,
        server_id: ServerId,
        connection_id: ConnectionId,
        channel_id: ChannelId,
        _visibility: Visibility,
    ) {
        println!(
            "[SERVER-{}] con {} moved to channel {}",
            server_id.0, connection_id.0, channel_id.0
        );
    }
}
