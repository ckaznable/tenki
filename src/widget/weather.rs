use ratatui::style::Color;

use crate::state::{tail::TailMode, wind::WindDirection, Cell, CellType};

use super::WeatherWidgetImpl;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GeneralWeatherWidget {
    Rain(WindDirection),
    Snow,
    Meteor(TailMode),
    Star,
}

impl WeatherWidgetImpl for GeneralWeatherWidget {
    fn get_color(&self, cell: CellType) -> Color {
        use CellType::*;
        match self {
            Self::Rain(_) => Color::Rgb(150, 150, 150),
            Self::Meteor(_) => match cell {
                Fast | Normal | Slow => Color::Yellow,
                _ => Color::Reset,
            }
            _ => Color::Reset,
        }
    }

    fn get_char(&self, d: CellType) -> char {
        use CellType::*;
        match self {
            Self::Rain(wind) => match d {
                Fast => '.',
                Normal => ':',
                Slow => match wind {
                    WindDirection::Left => '/',
                    WindDirection::Right => '\\',
                    WindDirection::None => '|',
                }
                _ => ' ',
            }
            Self::Snow => match d {
                CellType::Normal => '●',
                _ => ' ',
            }
            Self::Meteor(tail) => match d {
                Fast | Normal | Slow => '★',
                Tail => match tail {
                    TailMode::Left => '/',
                    TailMode::Right => '\\',
                    TailMode::Default => '|',
                },
                None => ' ',
            }
            _ => ' ',
        }
    }

    fn get_render_cell_type(&self, cell: &Cell) -> CellType {
        match self {
            Self::Snow => if !cell.is_empty() && cell.contains(&CellType::Normal) {
                CellType::Normal
            } else {
                CellType::None
            },

            _ => if cell.contains(&CellType::Slow) {
                CellType::Slow
            } else if !cell.is_empty() {
                *cell.first().unwrap()
            } else {
                CellType::None
            },
        }
    }
}

