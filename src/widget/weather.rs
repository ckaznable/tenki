use ratatui::style::Color;

use crate::state::{tail::TailMode, wind::WindDirection, Cell, CellKind};

use super::WeatherWidgetImpl;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GeneralWeatherWidget {
    Rain(WindDirection),
    Snow,
    Meteor(TailMode),
    Star,
    Disable,
}

impl WeatherWidgetImpl for GeneralWeatherWidget {
    fn get_color(&self, cell: CellKind) -> Color {
        use CellKind::*;
        match self {
            Self::Rain(_) => Color::Rgb(150, 150, 150),
            Self::Meteor(_) => match cell {
                Fast | Normal | Slow => Color::Yellow,
                _ => Color::Reset,
            }
            _ => Color::Reset,
        }
    }

    fn get_char(&self, d: CellKind) -> char {
        use CellKind::*;
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
                CellKind::Normal => '●',
                _ => ' ',
            }
            Self::Meteor(tail) => match d {
                Fast | Normal | Slow => '★',
                Tail => match tail {
                    TailMode::Left => '/',
                    TailMode::Right => '\\',
                    TailMode::Default => '|',
                },
                _ => ' ',
            }
            _ => ' ',
        }
    }

    fn get_render_cell_type(&self, cell: &Cell) -> CellKind {
        if *self == Self::Disable {
            return CellKind::None;
        }

        let cell = cell.kind_collect;
        match self {
            Self::Snow => if !cell.is_empty() && cell.contains(&CellKind::Normal) {
                CellKind::Normal
            } else {
                CellKind::None
            },

            _ => if cell.contains(&CellKind::Slow) {
                CellKind::Slow
            } else if !cell.is_empty() {
                *cell.first().unwrap()
            } else {
                CellKind::None
            },
        }
    }
}

