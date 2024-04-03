use super::{buffer::RenderBuffer, EachFrameImpl};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum WindMode {
    #[default]
    Random,
    Disable,
    OnlyRight,
    OnlyLeft,
}

impl WindMode {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "random" => Ok(WindMode::Random),
            "disable" => Ok(WindMode::Disable),
            "only-right" => Ok(WindMode::OnlyRight),
            "only-left" => Ok(WindMode::OnlyLeft),
            _ => Err("Invalid parameter, only accept random, disable, only-right or only-left."),
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum WindDirection {
    Left,
    Right,
    #[default]
    None,
}

type FrameCount = usize;

#[derive(Clone, Copy, Default)]
pub struct WindState {
    pub mode: WindMode,
    pub direction: WindDirection,
    frame: FrameCount,
}

impl WindState {
    pub fn new(mode: WindMode) -> Self {
        Self {
            mode,
            direction: match mode {
                WindMode::OnlyRight => WindDirection::Right,
                WindMode::OnlyLeft => WindDirection::Left,
                _ => WindDirection::None
            },
            ..Default::default()
        }
    }
}

impl EachFrameImpl for WindState {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, _: u8) {
        self.direction = match self.mode {
            WindMode::Disable => WindDirection::None,
            WindMode::OnlyLeft => WindDirection::Left,
            WindMode::OnlyRight => WindDirection::Right,
            _ => self.direction,
        };

        if self.mode != WindMode::Random {
            return;
        }

        if self.frame == 0 || self.direction == WindDirection::None {
            self.frame = 255;
            self.direction = if seed % 2024 == 0 {
                WindDirection::Left
            } else if seed % 123 == 0 {
                WindDirection::Right
            } else {
                WindDirection::None
            }
        }

        if self.direction == WindDirection::None {
            return;
        }

        self.frame = self.frame.saturating_sub(1);

        if self.direction == WindDirection::Left {
            rb.buf.reverse();
        }

        for i in 1..rb.buf.len() {
            rb.buf.swap(0, i);
        }

        if self.direction == WindDirection::Left {
            rb.buf.reverse();
        }
    }
}
