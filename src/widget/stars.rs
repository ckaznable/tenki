use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::app::{DropColumn, DropSpeed, Wind};

use super::BackgroundWidget;

pub struct Stars<'a> {
    buf: &'a Vec<DropColumn>,
    wind: Wind,
}

impl<'a> Stars<'a> {
    pub fn new(buf: &'a Vec<DropColumn>, wind: Wind) -> Self {
        Self { buf, wind }
    }
}

impl<'a> BackgroundWidget for Stars<'a> {
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
            Normal(0) => '★',
            Normal(_) => ':',
            Slow => match self.wind {
                Wind::Left(_) => '/',
                Wind::Right(_) => '\\',
                Wind::None => '|',
            },
            None => ' ',
        }
    }

    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            let Some(column) = self.buf().get(x as usize) else {
                continue;
            };
            let column = column.borrow();

            for y in area.top()..area.bottom() {
                let Some(cell) = column.get(y as usize) else {
                    continue;
                };
                buf.get_mut(x, y)
                    .set_char(self.get_render_char(cell))
                    .set_fg(self.get_drop_color());
            }
        }
    }
}

impl<'a> Widget for Stars<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_background(area, buf)
    }
}
