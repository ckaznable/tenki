use crate::{
    cli::Args,
    state::{buffer::RenderBuffer, dropping::DroppingState, tail::TailState, wind::WindState, EachFrameImpl, Mode},
    widget::{weather::GeneralWeatherWidget, AsWeatherWidget},
};

use super::WeatherImpl;

const DEF_LEVEL: u16 = 50;
const DEF_TAIL_LEVEL: u16 = 500;

pub struct GeneralDropping {
    wind: WindState,
    dropping: DroppingState,
}

impl GeneralDropping {
    pub fn new(args: Args) -> Self {
        Self {
            wind: WindState::new(args.wind),
            dropping: DroppingState {
                threshold: args.level.unwrap_or(DEF_LEVEL),
                mode: args.mode,
            },
        }
    }
}

impl WeatherImpl for GeneralDropping {}

impl EachFrameImpl for GeneralDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u64) {
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

pub struct TailDropping {
    wind: WindState,
    dropping: DroppingState,
    tail: TailState,
}

impl TailDropping {
    pub fn new(args: Args) -> Self {
        Self {
            wind: WindState::new(args.wind.without_random()),
            tail: TailState::new(args.wind.into()),
            dropping: DroppingState {
                threshold: args.level.unwrap_or(DEF_TAIL_LEVEL),
                mode: args.mode,
            },
        }
    }
}

impl WeatherImpl for TailDropping {}

impl EachFrameImpl for TailDropping {
    fn on_frame(&mut self, rb: &mut RenderBuffer, seed: u64, frame: u64) {
        self.wind.on_frame(rb, seed, frame);
        self.dropping.on_frame(rb, seed, frame);
        self.tail.on_frame(rb, seed, frame);
    }
}

impl AsWeatherWidget for TailDropping {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        use Mode::*;
        match self.dropping.mode {
            Meteor => GeneralWeatherWidget::Meteor(self.tail.mode),
            _ => panic!("has not been implemented yet"),
        }
    }
}
