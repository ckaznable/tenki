use ratatui::{widgets::Widget, buffer::Buffer, layout::Rect};

use crate::app::{DropColumn, DropSpeed, DropCell};

use super::BackgroundWidget;

pub struct Snow<'a> {
    buf: &'a Vec<DropColumn>,
}

impl<'a> Snow<'a> {
    pub fn new(buf: &'a Vec<DropColumn>) -> Self {
        Self { buf }
    }
}

impl<'a> BackgroundWidget for Snow<'a> {
    fn buf(&self) -> &Vec<DropColumn> {
        self.buf
    }

    fn get_drop_char(&self, d: DropSpeed) -> char {
        match d {
            DropSpeed::Normal => '●',
            _ => ' ',
        }
    }

    fn get_render_char(&self, cell: &DropCell) -> char {
        self.get_drop_char(if !cell.is_empty() && cell.contains(&DropSpeed::Normal) {
            DropSpeed::Normal
        } else {
            DropSpeed::None
        })
    }
}

impl<'a> Widget for Snow<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_background(area, buf)
    }
}
