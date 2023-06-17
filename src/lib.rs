mod audiofx;
mod gui;
mod websocket;

use audiofx::init_high_pass;
use ts3plugin::*;

struct RustyChatTsPlugin;

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

    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);
        websocket::start_listen();
        Ok(Box::new(Self))
    }

    fn post_process_voice_data(&mut self, api: &mut TsApi, server_id: ServerId,
            connection_id: ConnectionId, samples: &mut [i16], channels: i32,
            channel_speaker_array: &[Speaker], channel_fill_mask: &mut u32) {
                audiofx::process_radio(samples, &mut audiofx::init_band_pass(), &mut init_high_pass());
    }

    fn captured_voice_data(&mut self, api: &mut TsApi, server_id: ServerId,
            samples: &mut [i16], channels: i32, send: &mut bool) -> bool {
        audiofx::process_radio(samples, &mut audiofx::init_band_pass(), &mut init_high_pass());
        true
    }

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "MyTsPlugin", LogLevel::Info);
    }
}

create_plugin!(RustyChatTsPlugin);
