use ratatui::Frame;

use crate::{app::State, widget::rain::Rain};

pub fn ui(f: &mut Frame, state: &mut State) {
    let area = f.size();
    f.render_widget(Rain::new(&state.buf), area);
}
