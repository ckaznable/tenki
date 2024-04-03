use crate::state::{DropCell, DropSpeed};

use super::WeatherWidgetImpl;

pub struct Snow;
impl WeatherWidgetImpl for Snow {
    fn get_char(&self, d: DropSpeed) -> char {
        match d {
            DropSpeed::Normal => '●',
            _ => ' ',
        }
    }

    fn get_render_char(&self, cell: &DropCell) -> char {
        self.get_char(if !cell.is_empty() && cell.contains(&DropSpeed::Normal) {
            DropSpeed::Normal
        } else {
            DropSpeed::None
        })
    }
}
