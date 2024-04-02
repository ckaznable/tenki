use crate::state::{
    buffer::RenderBuffer, dropping::DroppingState, wind::{WindMode, WindState}, EachFrameImpl, Mode
};

pub struct GeneralDropping {
    wind: WindState,
    dropping: DroppingState,
}

impl GeneralDropping {
    pub fn new() -> Self {
        Self {
            wind: WindState::new(WindMode::default()),
            dropping: DroppingState {
                threshold: 50,
                mode: Mode::default(),
            }
        }
    }
}

impl EachFrameImpl for GeneralDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u8) {
        self.wind.on_frame(rb, seed, frame);
        self.dropping.on_frame(rb, seed, frame);
    }
}

