use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::StatefulWidget,
};

use crate::state::{buffer::RenderBuffer, Cell, CellType};

pub mod fps;
pub mod timer;
pub mod weather;

pub trait WeatherWidgetImpl {
    fn get_char(&self, _: CellType) -> char;
    fn get_render_cell_type(&self, cell: &Cell) -> CellType;
    fn get_color(&self, cell: CellType) -> Color;

    fn render_background(&self, area: Rect, buf: &mut Buffer, rb: &RenderBuffer) {
        for x in area.left()..area.right() {
            let Some(column) = rb.buf.get(x as usize) else {
                continue;
            };

            let column = column.borrow();
            for y in area.top()..area.bottom() {
                if let Some(cell) = column.get(y as usize) {
                    let cell_type = self.get_render_cell_type(cell);
                    buf.get_mut(x, y)
                        .set_char(self.get_char(cell_type))
                        .set_fg(self.get_color(cell_type));
                }
            }
        }
    }
}

pub trait AsWeatherWidget {
    type Weather: WeatherWidgetImpl;
    fn as_weather_widget(&self) -> Self::Weather;
}

pub struct WeatherWidget<T: WeatherWidgetImpl> {
    implement: T,
}

impl<T: WeatherWidgetImpl> WeatherWidget<T> {
    pub fn new(implement: T) -> Self {
        Self { implement }
    }
}

impl<T: WeatherWidgetImpl> StatefulWidget for WeatherWidget<T> {
    type State = RenderBuffer;
    fn render(self, area: Rect, buf: &mut Buffer, rb: &mut Self::State) {
        self.implement.render_background(area, buf, rb)
    }
}
