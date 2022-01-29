use iced::{
    executor, Application, Settings, Text, Command, Clipboard, Subscription, Element, Color,
    Container, Length, Column, Align, Row, Scrollable, Rule, scrollable, Space, window,

};
use iced::futures::io::Window;

#[derive(Debug, Default)]
struct Headlines {
    articles: Vec<NewsData>,
    scroll: scrollable::State,
}

#[derive(Debug, Default, Clone)]
struct NewsData {
    title: String,
    hashtag: String,
    url: String
}

impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsData {
            title: format!("title{}", a),
            hashtag: format!("hashtag{}", a),
            url: format!("url{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
            scroll: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ScrollToTop(usize),
    ScrollToBottom(usize),
    Scrolled(usize, f32),
}

impl Application for Headlines {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Headlines::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Top Headlines")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let Headlines {
            articles,
            ..
        } = self;

        let headlinses = Column::with_children(
            articles
                .iter_mut()
                .map(|article| {
                    Column::new()
                        .push(Text::new(&article.title))
                        .push(Text::new(&article.hashtag))
                        .push(Text::new(&article.url))
                        .into()
                }).collect()
        );

        let mut content = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .align_items(Align::Start)
            .spacing(10)
            .push(headlinses);


        Container::new(content)
            .width(Length::Units(540))
            .height(Length::Units(960))
            .into()
    }
}



fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (540, 960),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    Headlines::run(settings)
}
