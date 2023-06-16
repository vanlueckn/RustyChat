use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, column, container, text};
use iced::Application;
use iced::{Command, Element, Length};

pub fn show() -> iced::Result {
    Settings::run(iced::Settings::default())
}

pub struct Settings {
    web_socket_address: String,
    instance_timeout: u32,
    is_3d_enabled: bool,
    phone_offset: StereoMode,
    radio_offset: StereoMode,
    secondary_radio_offset: StereoMode,
    mic_click_mode: MicClickMode,
}

enum StereoMode {
    Stereo,
    LeftOnly,
    RightOnly,
}

enum MicClickMode {
    ScriptDependent,
    Never,
    Always,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Save,
    Close,
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
        (
            Self {
                web_socket_address: String::from(""),
                instance_timeout: 0,
                is_3d_enabled: false,
                phone_offset: StereoMode::Stereo,
                radio_offset: StereoMode::Stereo,
                secondary_radio_offset: StereoMode::Stereo,
                mic_click_mode: MicClickMode::ScriptDependent,
            },
            Command::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let content = column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::Save),
            // We show the value of the counter here
            text(self.instance_timeout).size(50),
            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::Save),
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Save => {}
            Message::Close => {}
        }

        Command::none()
    }
}
