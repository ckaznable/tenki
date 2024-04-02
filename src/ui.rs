use ratatui::Frame;
use crate::state::{EachFrameImpl, Mode, State};
use crate::cli::Args;

use crate::widget::{rain::Rain, snow::Snow, timer::Timer};

pub fn ui<T: EachFrameImpl>(f: &mut Frame, state: &mut State<T>, args: Args) {
    let area = f.size();

    match args.mode {
        Mode::Rain => f.render_widget(Rain::new(&state.rb.buf, crate::state::wind::WindDirection::None), area),
        Mode::Snow => f.render_widget(Snow::new(&state.rb.buf), area),
        _ => (),
    };

    f.render_widget(Timer(state.timer, args.timer_color), area);
}
