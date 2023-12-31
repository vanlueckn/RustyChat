use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, checkbox, column, container, pick_list, text, text_input};
use iced::window;
use iced::Application;
use iced::{Command, Element, Length};

pub fn show() -> iced::Result {
    Settings::run(iced::Settings {
        window: window::Settings {
            size: (350, 500),
            ..window::Settings::default()
        },
        ..iced::Settings::default()
    })
}
pub struct Settings {
    web_socket_address: String,
    is_3d_enabled: bool,
    phone_offset: Option<StereoMode>,
    radio_offset: Option<StereoMode>,
    secondary_radio_offset: Option<StereoMode>,
    mic_click_mode: Option<MicClickMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StereoMode {
    Stereo,
    LeftOnly,
    RightOnly,
}

impl StereoMode {
    const ALL: [Self; 3] = [Self::Stereo, Self::LeftOnly, Self::RightOnly];
}

impl std::fmt::Display for StereoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Stereo => "Stereo",
                Self::LeftOnly => "Left Only",
                Self::RightOnly => "Right Only",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MicClickMode {
    #[default]
    ScriptDependent,
    Never,
    Always,
}

impl MicClickMode {
    const ALL: [Self; 3] = [Self::ScriptDependent, Self::Never, Self::Always];
}

impl std::fmt::Display for MicClickMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ScriptDependent => "Script Dependent",
                Self::Never => "Never",
                Self::Always => "Always",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Ok,
    WebSocketChanged(String),
    Has3dChanged(bool),
    PhoneOffset(StereoMode),
    RadioOffset(StereoMode),
    SecondaryRadioOffset(StereoMode),
    MicClickMode(MicClickMode),
}

impl Application for Settings {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn title(&self) -> String {
        String::from("RustyChat - Settings")
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let me = Self {
            web_socket_address: String::from("ws://localhost:31850"),
            is_3d_enabled: false,
            phone_offset: Some(StereoMode::Stereo),
            radio_offset: Some(StereoMode::Stereo),
            secondary_radio_offset: Some(StereoMode::Stereo),
            mic_click_mode: Some(MicClickMode::ScriptDependent),
        };
        (me, Command::none())
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("WebSocket URL:"),
            text_input(&self.web_socket_address, &self.web_socket_address)
                .on_input(Message::WebSocketChanged),
            checkbox("3D Audio", self.is_3d_enabled, Message::Has3dChanged),
            text("Phone Offset:"),
            pick_list(
                &StereoMode::ALL[..],
                self.phone_offset,
                Message::PhoneOffset
            ),
            text("Primary Radio Offset:"),
            pick_list(
                &StereoMode::ALL[..],
                self.radio_offset,
                Message::RadioOffset
            ),
            text("Secondary Radio Offset:"),
            pick_list(
                &StereoMode::ALL[..],
                self.secondary_radio_offset,
                Message::SecondaryRadioOffset
            ),
            text("Mic Click Mode:"),
            pick_list(
                &MicClickMode::ALL[..],
                self.mic_click_mode,
                Message::MicClickMode
            ),
            button("Ok").on_press(Message::Ok),
        ]
        .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(15)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Ok => {}
            Message::WebSocketChanged(value) => self.web_socket_address = value,
            Message::Has3dChanged(value) => self.is_3d_enabled = value,
            Message::PhoneOffset(value) => self.phone_offset = Some(value),
            Message::RadioOffset(value) => self.radio_offset = Some(value),
            Message::SecondaryRadioOffset(value) => self.secondary_radio_offset = Some(value),
            Message::MicClickMode(value) => self.mic_click_mode = Some(value),
        }

        Command::none()
    }
}
