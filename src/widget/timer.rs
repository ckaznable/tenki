use itertools::Itertools;
use ratatui::{widgets::Widget, layout::{Rect, Layout, Direction, Constraint}, buffer::Buffer, style::Color};

pub const TIMER_CHAR: char = '█';
pub const COLON_CHAR: char = '▀';

static ASCII_0: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

static ASCII_1: [u8; 25] = [
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
    0, 0, 1, 1, 0,
];

static ASCII_2: [u8; 25] = [
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
];

static ASCII_3: [u8; 25] = [
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

static ASCII_4: [u8; 25] = [
    1, 1, 0, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
];

static ASCII_5: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

static ASCII_6: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 0, 0,
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

static ASCII_7: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
    0, 0, 0, 1, 1,
];

static ASCII_8: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
];

static ASCII_9: [u8; 25] = [
    1, 1, 1, 1, 1,
    1, 1, 0, 1, 1,
    1, 1, 1, 1, 1,
    0, 0, 0, 1, 1,
    1, 1, 1, 1, 1,
];

pub struct Timer(pub crate::state::Timer);

impl Timer {
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
                            buf .get_mut(left + x as u16, top + y as u16)
                                .set_char(TIMER_CHAR)
                                .set_fg(color);
                        }
                    })
            });
    }

    fn get_center_area(area: Rect) -> Rect {
        let width = 39u16;
        let height = 5u16;
        let padding_h = (area.width.saturating_sub(width)) / 2;
        let padding_v = (area.height.saturating_sub(height)) / 2;

        Rect {
            x: padding_h,
            y: padding_v,
            height,
            width,
        }
    }
}

impl Widget for Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let center_area = Self::get_center_area(area);
        let [hours, colon_left, minutes, colon_right, seconds] = Layout::new(
            Direction::Horizontal,
            Constraint::from_lengths([11, 3, 11, 3, 11]),
        )
        .areas(center_area);

        Self::render_decimal(self.0.hours, hours, self.0.color, buf);
        Self::render_colon(colon_left, self.0.color, buf);
        Self::render_decimal(self.0.minutes, minutes, self.0.color, buf);
        Self::render_colon(colon_right, self.0.color, buf);
        Self::render_decimal(self.0.seconds, seconds, self.0.color , buf);
    }
}

