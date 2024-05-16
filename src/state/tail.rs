use std::cell::RefCell;

use super::{buffer::RenderBuffer, wind::WindMode, Cell, Column, CellType, EachFrameImpl};

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

    fn remove_tail_from_cell(cell: Cell) -> Cell {
        cell.into_iter().filter(|t| *t != CellType::Tail).collect()
    }

    fn append_tail(mut cell: Cell) -> Cell {
        if !cell.contains(&CellType::Tail) {
            cell.push(CellType::Tail);
        }

        cell
    }

    fn render_default_tail(cols: &mut Vec<Column>) {
        for col in cols {
            let mut col = RefCell::borrow_mut(col);
            for i in 0..col.len() {
                if let Some(cell) = col.get_mut(i) {
                    *cell = Self::remove_tail_from_cell(*cell);

                    if i == 0 {
                        continue;
                    }

                    if !cell.iter().any(|t| t.is_dropping_cell()) {
                        continue;
                    }

                    for j in i.saturating_sub(TAIL_LEN + 1)..(i - 1) {
                        let cell = col.get_mut(j).unwrap();
                        *cell = Self::append_tail(*cell)
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
                    *cell = Self::remove_tail_from_cell(*cell);

                    if y == 0 {
                        continue;
                    }

                    if !cell.iter().any(|t| t.is_dropping_cell()) {
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
                    *cell = Self::append_tail(*cell)
                }
            }

        });
    }
}

impl EachFrameImpl for TailState {
    fn on_frame(&mut self, rb: &mut RenderBuffer, _seed: u64, _frame: u64) {
        match self.mode {
            TailMode::Left => Self::render_left_tail(&mut rb.buf),
            TailMode::Right => Self::render_right_tail(&mut rb.buf),
            TailMode::Default => Self::render_default_tail(&mut rb.buf),
        }
    }
}
