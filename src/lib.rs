#[macro_use]
extern crate ts3plugin;
#[macro_use]
extern crate lazy_static;

use ts3plugin::*;

struct RustyChatTsPlugin;

impl Plugin for RustyChatTsPlugin {
    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);
        Ok(Box::new(Self))
    }

    // Implement callbacks here

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "MyTsPlugin", LogLevel::Info);
    }
}

create_plugin!(
    "RustyChat",
    "0.1.0",
    "RustyChat Contributors",
    "Voice plugin for roleplay servers",
    ConfigureOffer::No,
    false,
    RustyChatTsPlugin
);

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
