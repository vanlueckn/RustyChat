mod audiofx;
mod game;
mod gui;
mod websocket;

use iir_filters::filter::DirectForm2Transposed;
use ts3plugin::*;

struct RustyChatTsPlugin {
    low_pass: DirectForm2Transposed,
    band_pass: DirectForm2Transposed,
    high_pass: DirectForm2Transposed,
    vol_follow: f32,
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

    fn configure(&mut self, _api: &mut TsApi) {
        let _res = gui::show(&_api.get_plugin_path());
    }

    fn connect_status_change(
        &mut self,
        _api: &mut TsApi,
        _server_id: ServerId,
        status: ConnectStatus,
        _error: Error,
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
        _api: &mut TsApi,
        _server_id: ServerId,
        _connection_id: ConnectionId,
        _old_channel_id: ChannelId,
        _new_channel_id: ChannelId,
        _visibility: Visibility,
    ) {
        println!("move");
    }

    fn connection_moved(
        &mut self,
        _api: &mut TsApi,
        _server_id: ServerId,
        _connection_id: ConnectionId,
        _old_channel_id: ChannelId,
        _new_channel_id: ChannelId,
        _visibility: Visibility,
        _invoker: Invoker,
    ) {
        println!("moved");
    }

    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);

        if win32console::console::WinConsole::alloc_console().is_err() {}
        println!("attached console");

        websocket::start_listen();

        let low_pass = audiofx::init_lowpass().unwrap();
        let band_pass = audiofx::init_band_pass().unwrap();
        let high_pass = audiofx::init_high_pass().unwrap();

        Ok(Box::new(Self {
            low_pass,
            band_pass,
            high_pass,
            vol_follow: 0.0,
        }))
    }

    fn post_process_voice_data(
        &mut self,
        _api: &mut TsApi,
        _server_id: ServerId,
        _connection_id: ConnectionId,
        samples: &mut [i16],
        _channels: i32,
        _channel_speaker_array: &[Speaker],
        _channel_fill_mask: &mut u32,
    ) {
        audiofx::process_radio(samples, &mut self.vol_follow);
    }

    fn captured_voice_data(
        &mut self,
        _api: &mut TsApi,
        _server_id: ServerId,
        samples: &mut [i16],
        _channels: i32,
        _send: &mut bool,
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
        _whispering: bool,
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
        _api: &mut TsApi,
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
