use crate::{
    state::EachFrameImpl,
    widget::{weather::GeneralWeatherWidget, AsWeatherWidget},
};

use super::WeatherImpl;

pub struct EmptyWeather;

impl AsWeatherWidget for EmptyWeather {
    type Weather = GeneralWeatherWidget;

    fn as_weather_widget(&self) -> Self::Weather {
        GeneralWeatherWidget::Disable
    }
}

impl EachFrameImpl for EmptyWeather {}
impl WeatherImpl for EmptyWeather {}
