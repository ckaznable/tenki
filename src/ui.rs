use ratatui::Frame;

use crate::{app::{State, Mode}, widget::{rain::Rain, timer::Timer, snow::Snow}};

pub fn ui(f: &mut Frame, state: &mut State) {
    let area = f.size();

    match state.mode {
        Mode::Rain => f.render_widget(Rain::new(&state.buf, state.wind), area),
        Mode::Snow => f.render_widget(Snow::new(&state.buf), area),
    };

    f.render_widget(Timer(state.timer), area);
}

