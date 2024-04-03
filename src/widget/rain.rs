use ratatui::style::Color;

use crate::state::{wind::WindDirection, DropSpeed};

use super::WeatherWidgetImpl;

pub struct Rain {
    wind: WindDirection,
}

impl Rain {
    pub fn new(wind: WindDirection) -> Self {
        Self { wind }
    }
}

impl WeatherWidgetImpl for Rain {
    fn get_color(&self) -> Color {
        Color::Rgb(150, 150, 150)
    }

    fn get_char(&self, d: DropSpeed) -> char {
        use DropSpeed::*;
        match d {
            Fast => '.',
            Normal => ':',
            Slow => match self.wind {
                WindDirection::Left => '/',
                WindDirection::Right => '\\',
                WindDirection::None => '|',
            },
            None => ' ',
        }
    }
}

