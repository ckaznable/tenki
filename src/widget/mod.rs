use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::StatefulWidget,
};

use crate::state::{buffer::RenderBuffer, DropCell, DropType};

pub mod fps;
pub mod timer;
pub mod weather;

pub trait WeatherWidgetImpl {
    fn get_char(&self, _: DropType) -> char;
    fn get_render_char(&self, cell: &DropCell) -> char;
    fn get_color(&self) -> Color;

    fn render_background(&self, area: Rect, buf: &mut Buffer, rb: &RenderBuffer) {
        for x in area.left()..area.right() {
            let Some(column) = rb.buf.get(x as usize) else {
                continue;
            };
            let column = column.borrow();

            for y in area.top()..area.bottom() {
                let Some(cell) = column.get(y as usize) else {
                    continue;
                };
                buf.get_mut(x, y)
                    .set_char(self.get_render_char(cell))
                    .set_fg(self.get_color());
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
