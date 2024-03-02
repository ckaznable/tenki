use ratatui::Frame;

use crate::{
    app::{Mode, State},
    widget::{rain::Rain, snow::Snow, stars::Stars, timer::Timer},
};

pub fn ui(f: &mut Frame, state: &mut State) {
    let area = f.size();

    match state.mode {
        Mode::Rain => f.render_widget(Rain::new(&state.buf, state.wind), area),
        Mode::Snow => f.render_widget(Snow::new(&state.buf), area),
        Mode::Stars => f.render_widget(Stars::new(&state.buf, state.wind), area),
    };

    f.render_widget(Timer(state.timer), area);
}
