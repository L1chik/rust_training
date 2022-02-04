mod style;

use iced::{executor, Application, Settings, Text, Command, Clipboard, Subscription, Element, Color, Container, Length, Column, Align, Scrollable, scrollable, Space, window, HorizontalAlignment, VerticalAlignment, Rectangle, Point, Size, Button, Rule, button, Row};



use crate::style::{GREEN, WHITE, PADDING, TITLE, DESCRIPTION, URL, GRAY, Theme::Dark};


#[derive(Debug, Default)]
struct Headlines {
    articles: Vec<NewsData>,
    scroll: scrollable::State,
    refresh: button::State,
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
            title: format!("Title{}", a),
            hashtag: format!("Hashtag{}", a),
            url: format!("Read more ↳{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
            scroll: Default::default(),
            theme: Dark,
            refresh: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Refresh,
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
            refresh,
            ..
        } = self;

        let header = Column::new()
            .width(Length::Units(540))
            .spacing(10)
            .push(Row::new()
                .width(Length::Units(540))
                .push(
                Text::new("Headlines")
                    .width(Length::Fill)
                    .height(Length::Units(35))
                    .color(GRAY)
                    .font(DESCRIPTION)
                    .size(30)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center))
                .push(
                    Button::new(&mut self.refresh, Text::new("↻")
                        .vertical_alignment(VerticalAlignment::Bottom)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25))
                        .padding(5)
                        .style(style::Button::Primary)
                        .on_press(Message::Refresh)
            ))

            .push(Rule::horizontal(1).style(self.theme)
            );

        let headlines = Column::with_children(
            articles
                .iter_mut()
                .map(|article| {
                    Column::new()
                        .width(Length::Units(520))
                        .padding(PADDING)

                        // Title
                        .push(
                            Text::new(format!("→ {}", &article.title))
                                .color(GREEN)
                                .size(22)
                                .font(TITLE))

                        // HashTags
                        .push(
                            Text::new(&article.hashtag)
                                .font(DESCRIPTION))

                        // URL
                        .push(
                            Text::new(&article.url)
                                .color(GREEN)
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Right)
                                .size(16)
                                .font(URL))
                        .push(Space::with_height(Length::Units(20)))
                        .push(Rule::horizontal(1).style(self.theme))
                        .push(Space::with_height(Length::Units(20)))
                        .into()

                    // Container::new(col)
                    //     .style(self.theme)
                    //     .into()
                }).collect()
        );

        let scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .spacing(10)
            .padding(PADDING)
            .push(headlines);

        let content = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .padding(PADDING)
            .push(header)
            .push(scrollable);


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


