use crate::{
    state::EachFrameImpl,
    widget::{weather::GeneralWeatherWidget, AsWeatherWidget},
};

use super::WeatherImpl;

pub struct EmptyWeather;

impl EachFrameImpl for EmptyWeather {
    fn on_frame(&mut self, _: &mut crate::state::buffer::RenderBuffer, _: u64, _: u64) {}
}

impl AsWeatherWidget for EmptyWeather {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        GeneralWeatherWidget::Disable
    }
}

impl WeatherImpl for EmptyWeather {}
