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
pub mod timer;
pub mod wind;

const SEED_BUF_SIZE: usize = 1024;

pub type DropCell = ArrayVec<[DropSpeed; 3]>;
pub type DropColumn = Rc<RefCell<Vec<DropCell>>>;

pub trait EachFrameImpl {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u8);
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum DropSpeed {
    Fast,
    Normal,
    Slow,
    #[default]
    None,
}

#[derive(Copy, Clone, Default, ValueEnum)]
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
    pub fn get_frame_by_speed(&self, s: DropSpeed) -> u8 {
        use DropSpeed::*;
        use Mode::*;

        match self {
            Rain => match s {
                Fast => 1,
                Normal => 2,
                Slow => 3,
                _ => 0,
            },
            Snow => match s {
                Fast => 2,
                Normal => 4,
                Slow => 8,
                _ => 0,
            },
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
    seed_buf: ArrayVec<[u64; SEED_BUF_SIZE]>,
    seed_buf_index: usize,
}

impl<T: EachFrameImpl> State<T> {
    pub fn new(size: Rect, weather: T) -> Self {
        State {
            rb: RenderBuffer::new(size),
            rng: SmallRng::from_entropy(),
            frame: 0,
            timer: Timer::default(),
            seed: 0,
            seed_buf: ArrayVec::<[u64; SEED_BUF_SIZE]>::new(),
            seed_buf_index: 0,
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
        self.seed = self.get_seed();
        self.weather.on_frame(&mut self.rb, self.seed, self.frame);
    }

    fn get_seed(&mut self) -> u64 {
        if self.seed_buf.len() == SEED_BUF_SIZE {
            self.seed_buf_index = if self.seed_buf_index >= SEED_BUF_SIZE { 0 } else { self.seed_buf_index.saturating_add(1) };
            return if let Some(s) = self.seed_buf.get(self.seed_buf_index) { *s } else { self.rng.next_u64() };
        }

        let seed = self.rng.next_u64();
        self.seed_buf.push(seed);
        seed
    }
}

