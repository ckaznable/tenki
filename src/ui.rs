use ratatui::Frame;
use crate::state::{EachFrameImpl, State};
use crate::cli::Args;

use crate::widget::{AsWeatherWidget, WeatherWidget};
use crate::widget::timer::Timer;

pub fn ui<T: EachFrameImpl + AsWeatherWidget>(f: &mut Frame, state: &mut State<T>, args: Args) {
    let area = f.size();

    f.render_stateful_widget(WeatherWidget::new(state.weather.as_weather_widget()), area, &mut state.rb);
    f.render_widget(Timer(state.timer, args.timer_color), area);
}
