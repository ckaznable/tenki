use std::time::SystemTime;

use chrono::{DateTime, Local, Timelike};
use ratatui::layout::Rect;

use crate::widget::timer::{TIMER_LAYOUT_HEIGHT, TIMER_LAYOUT_WIDTH};

use super::{buffer::RenderBuffer, Direction, EachFrameImpl, Position};

#[derive(Copy, Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Timer {
    fn default() -> Self {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        Self {
            hours: datetime.hour() as u8,
            minutes: datetime.minute() as u8,
            seconds: datetime.second() as u8,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TimerRenderMode {
    Dvd(Direction),
}
impl Default for TimerRenderMode {
    fn default() -> Self {
        Self::Dvd(Direction::default())
    }
}

pub struct TimerState {
    pub mode: Option<TimerRenderMode>,
    pub area: Rect,
    pub pos: Position,
    pub boundary: Rect,
    pub show_colon: bool,
}

impl TimerState {
    pub fn new(area: Rect, mode: Option<TimerRenderMode>) -> Self {
        let boundary = area;
        let area = Self::get_center_area(area);

        Self {
            mode,
            area,
            boundary,
            pos: area.into(),
            show_colon: true,
        }
    }

    fn on_dvd_frame(&mut self) {
        let Some(TimerRenderMode::Dvd(dir)) = self.mode else {
            return;
        };

        let is_collision_h = self.is_collision_h();
        let is_collision_v = self.is_collision_v();

        let dir =
            if is_collision_h && is_collision_v {
                dir.reflection_reverse()
            } else if is_collision_h {
                dir.reflection_h()
            } else if is_collision_v {
                dir.reflection_v()
            } else {
                dir
            };

        self.pos = self.pos.mv(dir);
        self.mode = Some(TimerRenderMode::Dvd(dir));
    }

    fn get_center_area(area: Rect) -> Rect {
        let padding_h = (area.width.saturating_sub(TIMER_LAYOUT_WIDTH)) / 2;
        let padding_v = (area.height.saturating_sub(TIMER_LAYOUT_HEIGHT)) / 2;

        Rect {
            x: padding_h,
            y: padding_v,
            height: TIMER_LAYOUT_HEIGHT,
            width: TIMER_LAYOUT_WIDTH,
        }
    }

    fn get_area_with_pos(pos: Position) -> Rect {
        pos.into_rect(TIMER_LAYOUT_WIDTH, TIMER_LAYOUT_HEIGHT)
    }

    fn is_collision_v(&self) -> bool {
        self.pos.0 == 0 || (self.pos.0 + TIMER_LAYOUT_WIDTH) >= self.boundary.width
    }

    fn is_collision_h(&self) -> bool {
        self.pos.1 == 0 || (self.pos.1 + TIMER_LAYOUT_HEIGHT) >= self.boundary.height
    }

    fn handle_mode(&mut self, frame: u64) {
        if self.mode.is_none() {
            return;
        }

        if frame % 8 > 0 {
            return;
        }

        match self.mode.unwrap() {
            TimerRenderMode::Dvd(_) => self.on_dvd_frame(),
        }

        self.area = Self::get_area_with_pos(self.pos);
    }

    fn handle_decimal(&mut self, frame: u64) {
        if frame % 24 == 0 {
            self.show_colon = !self.show_colon;
        }
    }
}

impl EachFrameImpl for TimerState {
    fn on_frame(&mut self, _: &mut RenderBuffer, _: u64, frame: u64) {
        self.handle_mode(frame);
        self.handle_decimal(frame);
    }
}
