use std::{cell::RefCell, fmt::Display, rc::Rc};

use clap::ValueEnum;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use ratatui::layout::Rect;
use tinyvec::ArrayVec;

use self::{
    buffer::RenderBuffer,
    timer::Timer,
};

pub mod buffer;
pub mod dropping;
pub mod tail;
pub mod timer;
pub mod wind;

pub type Cell = ArrayVec<[CellType; 3]>;
pub type Column = Rc<RefCell<Vec<Cell>>>;

pub trait EachFrameImpl {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u8);
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum CellType {
    Fast,
    Normal,
    Slow,
    Tail,
    #[default]
    None,
}

impl CellType {
    pub fn is_dropping_cell(&self) -> bool {
        use CellType::*;
        matches!(*self, Fast | Normal | Slow)
    }
}

#[derive(Copy, Clone, Default, ValueEnum, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Rain,
    Snow,
    Meteor,
    Star,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Mode::Rain => "rain",
            Mode::Snow => "snow",
            Mode::Meteor => "meteor",
            Mode::Star => "star",
        };

        s.fmt(f)
    }
}

impl Mode {
    pub fn get_frame_by_speed(&self, s: CellType) -> u8 {
        use CellType::*;
        use Mode::*;

        match self {
            Rain => match s {
                Fast => 1,
                Normal => 2,
                Slow => 3,
                _ => 0,
            }
            Snow => match s {
                Fast => 2,
                Normal => 4,
                Slow => 6,
                _ => 0,
            }
            Meteor => 4,
            _ => 0
        }
    }
}

pub struct State<T> {
    pub rb: RenderBuffer,
    pub timer: Timer,
    pub weather: T,
    frame: u8,
    rng: SmallRng,
    seed: u64,
}

impl<T: EachFrameImpl> State<T> {
    pub fn new(size: Rect, weather: T) -> Self {
        State {
            rb: RenderBuffer::new(size),
            rng: SmallRng::from_entropy(),
            frame: 0,
            timer: Timer::default(),
            seed: 0,
            weather,
        }
    }

    pub fn on_resize(&mut self, columns: u16, rows: u16) {
        self.rb = RenderBuffer::new(Rect {
            x: 0,
            y: 0,
            height: rows,
            width: columns,
        });
    }

    pub fn tick_timer(&mut self) {
        self.timer = Timer::new();
    }

    pub fn tick(&mut self) {
        self.frame = if self.frame > 240 { 0 } else { self.frame.saturating_add(1) };
        self.seed = self.rng.next_u64();
        self.weather.on_frame(&mut self.rb, self.seed, self.frame);
    }
}

