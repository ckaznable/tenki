use super::{wind::WindMode, EachFrameImpl};

#[derive(Default, Copy, Clone)]
pub enum TailMode {
    Left,
    Right,
    #[default]
    Default,
}

impl From<WindMode> for TailMode {
    fn from(value: WindMode) -> Self {
        match value {
            WindMode::Random | WindMode::Disable => Self::default(),
            WindMode::OnlyRight => Self::Right,
            WindMode::OnlyLeft => Self::Left,
        }
    }
}

pub struct TailState {
    pub mode: TailMode,
}

impl TailState {
    pub fn new(mode: TailMode) -> Self {
        Self { mode } 
    }
}

impl EachFrameImpl for TailState {
    fn on_frame(&mut self, rb: &mut super::buffer::RenderBuffer, _: u64, _: u8) {
        todo!()
    }
}
