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

pub struct Timer(pub crate::app::Timer);

impl Timer {
    fn render_colon(area: Rect, buf: &mut Buffer) {
        let left = area.left();
        let top = area.top();

        buf.get_mut(left + 1, top + 1).set_char(COLON_CHAR).set_fg(Color::Reset);
        buf.get_mut(left + 1, top + 3).set_char(COLON_CHAR).set_fg(Color::Reset);
    }

    fn render_decimal(d: u8, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(Direction::Horizontal, Constraint::from_lengths([5, 1, 5])).split(area);
        Self::render_number(d / 10, layout[0], buf);
        Self::render_number(d % 10, layout[2], buf);
    }

    fn render_number(number: u8, area: Rect, buf: &mut Buffer) {
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
                                .set_fg(Color::Reset);
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
        let layout = Layout::new(
            Direction::Horizontal,
            Constraint::from_lengths([11, 3, 11, 3, 11]),
        )
        .split(center_area);

        Self::render_decimal(self.0.hours, layout[0], buf);
        Self::render_colon(layout[1], buf);
        Self::render_decimal(self.0.minutes, layout[2], buf);
        Self::render_colon(layout[3], buf);
        Self::render_decimal(self.0.seconds, layout[4], buf);
    }
}

