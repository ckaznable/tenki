use ratatui::{widgets::Widget, buffer::Buffer, layout::Rect};

use crate::app::{DropColumn, DropSpeed};

pub struct Rain<'a> {
    buf: &'a Vec<DropColumn>,
}

impl<'a> Rain<'a> {
    pub fn new(buf: &'a Vec<DropColumn>) -> Self {
        Self { buf }
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
        for x in area.left()..area.right() {
            let Some(column) = self.buf.get(x as usize) else { continue; };
            let column = column.borrow();

            for y in area.top()..area.bottom() {
                let Some(cell) = column.get(y as usize) else { continue; };
                let d = if cell.contains(&DropSpeed::Slow) {
                    DropSpeed::Slow
                } else if !cell.is_empty() {
                    *cell.first().unwrap()
                } else {
                    DropSpeed::None
                };

                buf.get_mut(x, y).set_char(Self::get_drop_char(d));
            }
        }
    }
}
