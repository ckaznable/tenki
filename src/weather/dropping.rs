use crate::{
    cli::Args,
    state::{buffer::RenderBuffer, dropping::DroppingState, wind::WindState, EachFrameImpl, Mode}, widget::{weather::GeneralWeatherWidget, AsWeatherWidget},
};

pub struct GeneralDropping {
    wind: WindState,
    dropping: DroppingState,
}

impl GeneralDropping {
    pub fn new(args: Args) -> Self {
        let wind = WindState::new(args.wind);

        Self {
            wind,
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
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        use Mode::*;
        match self.dropping.mode {
            Rain => GeneralWeatherWidget::Rain(self.wind.direction),
            Snow => GeneralWeatherWidget::Snow,
            _ => panic!("has not been implemented yet"),
        }
    }
}

