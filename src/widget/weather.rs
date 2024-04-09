use ratatui::style::Color;

use crate::state::{wind::WindDirection, DropCell, DropSpeed};

use super::WeatherWidgetImpl;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GeneralWeatherWidget {
    Rain(WindDirection),
    Snow,
    Meteor,
    Star,
}

impl WeatherWidgetImpl for GeneralWeatherWidget {
    fn get_color(&self) -> Color {
        match self {
            Self::Rain(_) => Color::Rgb(150, 150, 150),
            _ => Color::Reset,
        }
    }

    fn get_char(&self, d: DropSpeed) -> char {
        use DropSpeed::*;
        match self {
            Self::Rain(wind) => match d {
                Fast => '.',
                Normal => ':',
                Slow => match wind {
                    WindDirection::Left => '/',
                    WindDirection::Right => '\\',
                    WindDirection::None => '|',
                },
                _ => ' ',
            },
            Self::Snow => match d {
                DropSpeed::Normal => '●',
                _ => ' ',
            },
            _ => ' ',
        }
    }

    fn get_render_char(&self, cell: &DropCell) -> char {
        match self {
            Self::Snow => self.get_char(if !cell.is_empty() && cell.contains(&DropSpeed::Normal) {
                DropSpeed::Normal
            } else {
                DropSpeed::None
            }),

            _ => self.get_char(if cell.contains(&DropSpeed::Slow) {
                DropSpeed::Slow
            } else if !cell.is_empty() {
                *cell.first().unwrap()
            } else {
                DropSpeed::None
            }),
        }
    }
}

