use ratatui::style::Color;

use crate::state::{wind::WindDirection, DropCell, DropType};

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

    fn get_char(&self, d: DropType) -> char {
        use DropType::*;
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
                DropType::Normal => '●',
                _ => ' ',
            },
            _ => ' ',
        }
    }

    fn get_render_char(&self, cell: &DropCell) -> char {
        match self {
            Self::Snow => self.get_char(if !cell.is_empty() && cell.contains(&DropType::Normal) {
                DropType::Normal
            } else {
                DropType::None
            }),

            _ => self.get_char(if cell.contains(&DropType::Slow) {
                DropType::Slow
            } else if !cell.is_empty() {
                *cell.first().unwrap()
            } else {
                DropType::None
            }),
        }
    }
}

