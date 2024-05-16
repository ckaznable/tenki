use ratatui::Frame;
use crate::app::AppRuntimeInfo;
use crate::state::{EachFrameImpl, State};
use crate::cli::Args;

use crate::widget::fps::FpsWidget;
use crate::widget::{AsWeatherWidget, WeatherWidget};
use crate::widget::timer::Timer;

pub fn ui<T: EachFrameImpl + AsWeatherWidget>(f: &mut Frame, state: &mut State<T>, args: Args, runtime_info: AppRuntimeInfo) {
    let area = f.size();

    f.render_stateful_widget(WeatherWidget::new(state.weather.as_weather_widget()), area, &mut state.rb);
    f.render_widget(Timer {
        timer: state.timer,
        color: args.timer_color,
        state: &state.timer_state,
    }, area);

    if args.show_fps {
        f.render_widget(FpsWidget(runtime_info.fps), area)
    }
}
