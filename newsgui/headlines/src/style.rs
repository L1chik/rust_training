use iced::{container, Color};


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

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color {
                r: 12.0 / 255.0,
                g: 12.0 / 255.0,
                b: 12.0 / 255.0,
                a: 0.99,
            }
                .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}