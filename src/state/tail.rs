use std::cell::RefCell;

use super::{
    buffer::RenderBuffer,
    wind::WindMode,
    CellKind,
    CellKindCollect,
    Column,
    EachFrameImpl,
    ShouldRender
};

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum TailMode {
    Left,
    Right,
    #[default]
    Default,
}

const TAIL_LEN: usize = 3;

impl From<WindMode> for TailMode {
    fn from(value: WindMode) -> Self {
        match value {
            WindMode::Random | WindMode::Disable => Self::default(),
            WindMode::OnlyRight => Self::Right,
            WindMode::OnlyLeft => Self::Left,
        }
    }
}

pub struct TailState {
    pub mode: TailMode,
}

impl TailState {
    pub fn new(mode: TailMode) -> Self {
        Self { mode } 
    }

    fn remove_tail_from_cell(cell: CellKindCollect) -> CellKindCollect {
        cell.into_iter().filter(|t| *t != CellKind::Tail).collect()
    }

    fn append_tail(mut cell: CellKindCollect) -> CellKindCollect {
        if !cell.contains(&CellKind::Tail) {
            cell.push(CellKind::Tail);
        }

        cell
    }

    fn render_default_tail(cols: &mut Vec<Column>) {
        for col in cols {
            let mut col = RefCell::borrow_mut(col);
            for i in 0..col.len() {
                if let Some(cell) = col.get_mut(i) {
                    cell.kind_collect = Self::remove_tail_from_cell(cell.kind_collect);

                    if i == 0 {
                        continue;
                    }

                    if !cell.kind_collect.iter().any(|t| t.is_dropping_cell()) {
                        continue;
                    }

                    for j in i.saturating_sub(TAIL_LEN + 1)..(i - 1) {
                        let cell = col.get_mut(j).unwrap();
                        cell.kind_collect = Self::append_tail(cell.kind_collect)
                    }
                };
            }
        }
    }

    fn render_left_tail(cols: &mut [Column]) {
        cols.reverse();
        Self::render_right_tail(cols);
        cols.reverse()
    }

    fn render_right_tail(cols: &mut [Column]) {
        let mut tail_pos: Vec<(usize, usize)> = vec![];

        for x in 0..cols.len() {
            let mut col = RefCell::borrow_mut(cols.get(x).unwrap());
            for y in 0..col.len() {
                if let Some(cell) = col.get_mut(y) {
                    cell.kind_collect = Self::remove_tail_from_cell(cell.kind_collect);

                    if y == 0 {
                        continue;
                    }

                    if !cell.kind_collect.iter().any(|t| t.is_dropping_cell()) {
                        continue;
                    }

                    for (i, xi) in (x.saturating_sub(TAIL_LEN)..x).enumerate() {
                        let yi = y.saturating_sub(TAIL_LEN.saturating_sub(i));
                        tail_pos.push((xi, yi));
                    }
                };
            }
        }

        tail_pos.into_iter().for_each(|(x, y)| {
            if let Some(col) = cols.get(x) {
                let mut col = RefCell::borrow_mut(col);
                if let Some(cell) = col.get_mut(y) {
                    cell.kind_collect = Self::append_tail(cell.kind_collect)
                }
            }
        });
    }
}

impl EachFrameImpl for TailState {
    fn on_frame(&mut self, rb: &mut RenderBuffer, _seed: u64, _frame: u64) -> ShouldRender {
        match self.mode {
            TailMode::Left => Self::render_left_tail(&mut rb.buf),
            TailMode::Right => Self::render_right_tail(&mut rb.buf),
            TailMode::Default => Self::render_default_tail(&mut rb.buf),
        };

        ShouldRender::Render
    }
}
