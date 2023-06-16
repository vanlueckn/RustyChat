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
        ConfigureOffer::No
    }

    fn new(api: &mut TsApi) -> Result<Box<Self>, InitError> {
        api.log_or_print("Inited", "RustyChatTsPlugin", LogLevel::Info);
        Ok(Box::new(Self))
    }

    fn shutdown(&mut self, api: &mut TsApi) {
        api.log_or_print("Shutdown", "MyTsPlugin", LogLevel::Info);
    }
}

create_plugin!(RustyChatTsPlugin);

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
