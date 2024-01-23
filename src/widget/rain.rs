use ratatui::{widgets::Widget, buffer::Buffer, layout::Rect};

use crate::app::{DropColumn, DropSpeed};

use super::BackgroundWidget;

pub struct Rain<'a> {
    buf: &'a Vec<DropColumn>,
}

impl<'a> Rain<'a> {
    pub fn new(buf: &'a Vec<DropColumn>) -> Self {
        Self { buf }
    }
}

impl<'a> BackgroundWidget for Rain<'a> {
    fn buf(&self) -> &Vec<DropColumn> {
        self.buf
    }

    fn get_drop_char(d: DropSpeed) -> char {
        use DropSpeed::*;
        match d {
            Fast => '.',
            Normal => ':',
            Slow => '|',
            None => ' ',
        }
    }
}

impl<'a> Widget for Rain<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_background(area, buf)
    }
}
