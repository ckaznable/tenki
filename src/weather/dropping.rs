use crate::{
    cli::Args,
    state::{buffer::RenderBuffer, dropping::DroppingState, wind::WindState, EachFrameImpl},
    widget::{rain::Rain, AsWeatherWidget},
};

pub struct GeneralDropping {
    wind: WindState,
    dropping: DroppingState,
}

impl GeneralDropping {
    pub fn new(args: Args) -> Self {
        Self {
            wind: WindState::new(args.wind),
            dropping: DroppingState {
                threshold: args.level,
                mode: args.mode,
            },
        }
    }
}

impl EachFrameImpl for GeneralDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u8) {
        self.wind.on_frame(rb, seed, frame);
        self.dropping.on_frame(rb, seed, frame);
    }
}

impl AsWeatherWidget for GeneralDropping {
    type Weather = Rain;
    fn as_weather_widget(&self) -> Self::Weather {
        Rain::new(self.wind.direction)
    }
}
