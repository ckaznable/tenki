use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::state::{wind::WindDirection, DropColumn, DropSpeed};

use super::BackgroundWidget;

pub struct Rain<'a> {
    buf: &'a Vec<DropColumn>,
    wind: WindDirection,
}

impl<'a> Rain<'a> {
    pub fn new(buf: &'a Vec<DropColumn>, wind: WindDirection) -> Self {
        Self { buf, wind }
    }
}

impl<'a> BackgroundWidget for Rain<'a> {
    fn buf(&self) -> &Vec<DropColumn> {
        self.buf
    }

    fn get_drop_color(&self) -> Color {
        Color::Rgb(150, 150, 150)
    }

    fn get_drop_char(&self, d: DropSpeed) -> char {
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

impl<'a> Widget for Rain<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_background(area, buf)
    }
}
