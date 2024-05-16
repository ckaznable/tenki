use crate::{
    cli::Args,
    state::{EachFrameImpl, Mode},
    weather::{dropping::{GeneralDropping, TailDropping}, empty::EmptyWeather}, widget::{weather::GeneralWeatherWidget, AsWeatherWidget},
};

pub mod dropping;
pub mod empty;

pub trait WeatherImpl: EachFrameImpl + AsWeatherWidget<Weather=GeneralWeatherWidget> {}

pub struct Weather(Box<dyn WeatherImpl>);

impl Weather {
    pub fn from(args: Args) -> impl EachFrameImpl + AsWeatherWidget {
        use Mode::*;
        match args.mode {
            Rain | Snow => Self(Box::new(GeneralDropping::new(args))),
            Meteor => Self(Box::new(TailDropping::new(args))),
            Disable => Self(Box::new(EmptyWeather)),
            _ => panic!("has not been implemented yet for this mode"),
        }
    }
}

impl EachFrameImpl for Weather {
    fn on_frame(&mut self, rb: &mut crate::state::buffer::RenderBuffer, seed: u64, frame: u64) {
        self.0.on_frame(rb, seed, frame)
    }
}

impl AsWeatherWidget for Weather {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        self.0.as_weather_widget()
    }
}

