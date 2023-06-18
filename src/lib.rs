mod audiofx;
mod game;
mod gui;
mod websocket;
use std::sync::{Arc, Mutex};

use audiofx::init_high_pass;
use game::GameHandler;
use iir_filters::filter::DirectForm2Transposed;
use ts3plugin::*;

#[macro_use]
extern crate lazy_static;

struct RustyChatTsPlugin {
    low_pass: DirectForm2Transposed,
    band_pass: DirectForm2Transposed,
    high_pass: DirectForm2Transposed,
    vol_follow: f32,
    rusty_handler: Arc<Mutex<GameHandler>>,
}

impl Plugin for RustyChatTsPlugin {
    fn name() -> String {
        String::from("RustyChat")
    }

    fn version() -> String {
        String::from("0.1.0")
    }

    fn author() -> String {
        String::from("RustyChat Contributors")
    }

    fn description() -> String {
        String::from("Voice Plugin for roleplay servers")
    }

    fn command() -> Option<String> {
        Some(String::from("rusty"))
    }

    fn autoload() -> bool {
        true
    }

    fn configurable() -> ConfigureOffer {
        ConfigureOffer::QtThread
    }

    fn configure(&mut self, api: &mut TsApi) {
        gui::show();
    }

    fn connect_status_change(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        status: ConnectStatus,
        error: Error,
    ) {
        match status {
            ConnectStatus::Connected => {
                //self.rusty_handler.lock().unwrap().server_id = Some(server_id.0);
            }
            _ => {}
        }
    }

    fn connection_move(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        old_channel_id: ChannelId,
        new_channel_id: ChannelId,
        visibility: Visibility,
    ) {
        println!("move");
        self.rusty_handler.lock().unwrap().ts_on_channel_switched(
            server_id,
            connection_id,
            new_channel_id,
            visibility,
        );
    }

    fn connection_moved(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        old_channel_id: ChannelId,
        new_channel_id: ChannelId,
        visibility: Visibility,
        invoker: Invoker,
    ) {
        println!("moved");
        self.rusty_handler.lock().unwrap().ts_on_channel_switched(
            server_id,
            connection_id,
            new_channel_id,
            visibility,
        );
    }

    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);

        if win32console::console::WinConsole::alloc_console().is_err() {}

        println!("attached console");

        let gameInst = GameHandler {
            server_id: None,
            in_game: false,
            original_channel: None,
            game_channel: None,
            own_client_id: None,
        };

        let game_ref = Arc::new(Mutex::new(gameInst));

        websocket::start_listen(game_ref.clone());

        let low_pass = audiofx::init_lowpass();
        let band_pass = audiofx::init_band_pass();
        let high_pass = audiofx::init_high_pass();

        Ok(Box::new(Self {
            low_pass,
            band_pass,
            high_pass,
            vol_follow: 0.0,
            rusty_handler: game_ref.clone(),
        }))
    }

    fn post_process_voice_data(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        samples: &mut [i16],
        channels: i32,
        channel_speaker_array: &[Speaker],
        channel_fill_mask: &mut u32,
    ) {
        audiofx::process_radio(samples, &mut self.vol_follow);
    }

    fn captured_voice_data(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        samples: &mut [i16],
        channels: i32,
        send: &mut bool,
    ) -> bool {
        audiofx::process_radio(samples, &mut self.vol_follow);
        true
    }

    fn talking_changed(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        connection_id: ConnectionId,
        talking: TalkStatus,
        whispering: bool,
    ) {
        let is_talking = match talking {
            TalkStatus::NotTalking => false,
            _ => true,
        };

        let server = api.get_server(server_id).unwrap();
        let client = server.get_connection(connection_id).unwrap();
        println!("Is Talking: {}, {}", client.get_id().0, is_talking);
        let _ = websocket::on_talk_state_toggle(
            server.get_uid().unwrap(),
            is_talking,
            client.get_name().unwrap(),
        );
    }

    fn self_variable_update(
        &mut self,
        api: &mut TsApi,
        server_id: ServerId,
        flag: ClientProperties,
        old_value: String,
        new_value: String,
    ) {
       websocket::on_self_variable_update(server_id, flag, old_value, new_value);
    }

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "MyTsPlugin", LogLevel::Info);
        //TODO: shutdown websocket server, ...
    }
}

create_plugin!(RustyChatTsPlugin);
