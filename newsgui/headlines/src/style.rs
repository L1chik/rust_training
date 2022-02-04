use iced::{ container, Color, Rectangle, Length, Point, Size, rule, Font, button };
use iced_native::{Widget, renderer, Layout, Hasher, Event, Clipboard, Element, Background};
use iced_native::layout::{ Limits, Node, self};

use std::hash::Hash;
use iced::rule::Style;
use iced_native::event::Status;

pub const PADDING: u16 = 10;
pub const WHITE: Color = Color::WHITE;
pub const GREEN: Color = Color::from_rgb(
                15.0 / 255.0,
                230.0 / 255.0,
                60.0 / 255.0
            );
pub const CYAN: Color = Color::from_rgb(0.0, 1.0, 1.0);
pub const GRAY: Color = Color::from_rgb(
                155.0 / 255.0,
                162.0 / 255.0,
                167.0 / 255.0,
            );

pub const TITLE: Font = Font::External {
    name: "Title",
    bytes: include_bytes!("../fonts/Hack-Bold.ttf"),
};

pub const DESCRIPTION: Font = Font::External {
    name: "Description",
    bytes: include_bytes!("../fonts/Hack-Regular.ttf"),
};

pub const URL: Font = Font::External {
    name: "Url",
    bytes: include_bytes!("../fonts/Hack-BoldItalic.ttf"),
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 1] = [Theme::Dark];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark
    }
}

impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Container.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn rule::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => Rule.into()
        }
    }
}

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                r: 27.0 / 255.0,
                g: 35.0 / 255.0,
                b: 40.0 / 255.0,
                a: 0.99,
            }
                .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> Style {
        rule::Style {
            color: Color::from_rgb(
                49.0 / 255.0,
                62.0 / 255.0,
                71.0 / 255.0,
            ),
            width: 2,
            radius: 1.0,
            fill_mode: rule::FillMode::Percent(90.0),
        }
    }
}

pub enum Button {
    Primary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::TRANSPARENT,
            })),
            border_radius: 0.0,
            text_color: GREEN,
            ..button::Style::default()
        }
    }
}