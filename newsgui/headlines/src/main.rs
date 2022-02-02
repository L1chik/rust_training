mod style;

use iced::{
    executor, Application, Settings, Text, Command, Clipboard, Subscription, Element, Color,
    Container, Length, Column, Align, Scrollable, Rule, scrollable, Space, window, HorizontalAlignment,
    VerticalAlignment, Background, container
};
use iced::container::Style;
use iced::futures::io::Window;
use crate::style::Theme::Dark;

const PADDING: u16 = 5;
const WHITE: Color = Color::WHITE;
const CYAN: Color = Color::from_rgb(15.0 / 255.0,
                                    230.0 / 255.0,
                                    60.0 / 255.0);
const BACKGROUND: Color = Color::from_rgb(11.0, 14.0, 16.0);


#[derive(Debug, Default)]
struct Headlines {
    articles: Vec<NewsData>,
    scroll: scrollable::State,
    theme: style::Theme,
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
            theme: Dark,
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
                        .width(Length::Units(520))
                        .padding(PADDING)

                        // Title
                        .push(
                            Text::new(format!("→ {}", &article.title))
                                .color(WHITE)
                                .size(25))
                        .padding(PADDING)

                        // HashTags
                        .push(
                            Text::new(&article.hashtag)
                        )

                        // URL
                        .push(
                            Text::new(&article.url)
                                .color(CYAN)
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Right)
                                )
                        .into()
                }).collect()
        );

        let mut content = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            // .align_items(Align::Start)
            .spacing(10)
            .push(headlinses);

        Container::new(content)
            .max_width(540)
            .height(Length::Units(960))
            .style(self.theme)
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
    Headlines::background_color(&Headlines::new());
    Headlines::run(settings)
}

// impl container::StyleSheet for Container<'_, Message> {
//     fn style(&self) -> container::Style {
//         container::Style {
//             background: Color {
//                 a: 0.99,
//                 ..BACKGROUND
//             }
//                 .into(),
//             text_color: Color::WHITE.into(),
//             ..container::Style::default()
//         }
//     }
// }


