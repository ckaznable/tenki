use itertools::Itertools;
use ratatui::{widgets::Widget, layout::{Rect, Layout, Direction, Constraint}, buffer::Buffer, style::Color};

use crate::state::timer::TimerState;

pub const TIMER_CHAR: char = '█';
pub const COLON_CHAR: char = '▀';

const TIMER_LAYOUT: [u16; 5] = [11, 3, 11, 3, 11];
pub static TIMER_LAYOUT_WIDTH: u16 = 39;
pub static TIMER_LAYOUT_HEIGHT: u16 = 5;

const ASCII_0: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

const ASCII_1: [u8; 25] = [
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
];

const ASCII_2: [u8; 25] = [
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
];

const ASCII_3: [u8; 25] = [
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

const ASCII_4: [u8; 25] = [
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
];

const ASCII_5: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

const ASCII_6: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

const ASCII_7: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
];

const ASCII_8: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

const ASCII_9: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

pub struct Timer<'a> {
    pub timer: crate::state::timer::Timer,
    pub color: Color,
    pub state: &'a TimerState,
}

impl<'a> Timer<'a> {
    fn render_colon(area: Rect, color: Color, buf: &mut Buffer) {
        let left = area.left();
        let top = area.top();

        buf.get_mut(left + 1, top + 1).set_char(COLON_CHAR).set_fg(color);
        buf.get_mut(left + 1, top + 3).set_char(COLON_CHAR).set_fg(color);
    }

    fn render_decimal(d: u8, area: Rect, color: Color, buf: &mut Buffer) {
        let layout = Layout::new(Direction::Horizontal, Constraint::from_lengths([5, 1, 5])).split(area);
        Self::render_number(d / 10, layout[0], buf, color);
        Self::render_number(d % 10, layout[2], buf, color);
    }

    fn render_number(number: u8, area: Rect, buf: &mut Buffer, color: Color) {
        let left = area.left();
        let top = area.top();

        let ascii = match number {
            0 => ASCII_0,
            1 => ASCII_1,
            2 => ASCII_2,
            3 => ASCII_3,
            4 => ASCII_4,
            5 => ASCII_5,
            6 => ASCII_6,
            7 => ASCII_7,
            8 => ASCII_8,
            9 => ASCII_9,
            _ => ASCII_0,
        };

        ascii.iter()
            .chunks(5)
            .into_iter()
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter()
                    .enumerate()
                    .for_each(|(x, c)| {
                        if *c > 0 {
                            buf.get_mut(left + x as u16, top + y as u16)
                               .set_char(TIMER_CHAR)
                               .set_fg(color);
                        }
                    })
            });
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, _: Rect, buf: &mut Buffer) {
        let [hours, colon_left, minutes, colon_right, seconds] = Layout::new(
            Direction::Horizontal,
            Constraint::from_lengths(TIMER_LAYOUT),
        )
        .areas(self.state.area);

        Self::render_decimal(self.timer.hours, hours, self.color, buf);
        Self::render_decimal(self.timer.minutes, minutes, self.color, buf);
        Self::render_decimal(self.timer.seconds, seconds, self.color , buf);

        if self.state.show_colon {
            Self::render_colon(colon_left, self.color, buf);
            Self::render_colon(colon_right, self.color, buf);
        }
    }
}

