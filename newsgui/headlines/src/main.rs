use iced::{executor, Application, Settings, Text, Command, Clipboard, Subscription, Element, Color};
use iced::window::Mode;
use opencv::{
    Result,
    prelude::*,
    videoio,
    highgui
};

struct Headlines;

impl Application for Headlines {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Headlines, Command::none())
    }

    fn title(&self) -> String {
        String::from("Top Headlines")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Test").into()
    }
    
}



fn main() -> Result<()> {
    // Headlines::run(Settings::default())
    let cam = videoio::VideoCapture::new(
        0, videio::CAP_ANY)?;
    highgui::named_window("test", highgui::WINDOW_FULLSCREEN);

    Ok(())
}
