mod audiofx;
mod gui;
mod websocket;

use audiofx::init_high_pass;
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

    fn configure(&mut self, api: &mut TsApi) {
        gui::show();
    }

    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);
        websocket::start_listen();

        let low_pass = audiofx::init_lowpass();
        let band_pass = audiofx::init_band_pass();
        let high_pass = audiofx::init_high_pass();

        Ok(Box::new(Self {
            low_pass,
            band_pass,
            high_pass,
            vol_follow: 0.0,
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

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "MyTsPlugin", LogLevel::Info);
    }
}

create_plugin!(RustyChatTsPlugin);
