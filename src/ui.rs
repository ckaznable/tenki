use ratatui::Frame;

use crate::{app::State, widget::{rain::Rain, timer::Timer}};

pub fn ui(f: &mut Frame, state: &mut State) {
    let area = f.size();
    f.render_widget(Rain::new(&state.buf), area);
    f.render_widget(Timer(state.timer), area);
}
