use super::{buffer::RenderBuffer, Cell, Column, CellType, Mode, EachFrameImpl};

pub struct DroppingState {
    pub threshold: u16,
    pub mode: Mode,
}

impl DroppingState {
    fn gen_drop(&self, rb: &mut RenderBuffer, seed: u64) {
        rb.line.clear();

        const GROUP_SIZE: u64 = 64;
        let len = rb.buf.len() as u64;
        let last_group = len % GROUP_SIZE;
        let groups = len / GROUP_SIZE + if last_group > 0 { 1 } else { 0 };

        for g in 0..groups {
            let range = if groups.saturating_sub(1) == g { last_group } else { GROUP_SIZE };
            for i in 0..range {
                rb.line.push(if seed & (1 << i) != 0 {
                    Self::get_drop_speed(seed.saturating_sub(i), self.threshold)
                } else {
                    CellType::None
                });
            }
        }
    }

    /// generate new drop line
    fn new_drop(&self, rb: &mut RenderBuffer, seed: u64) {
        self.gen_drop(rb, seed);
        rb.line
            .iter()
            .enumerate()
            .for_each(|(i, d)| {
                if let Some(cell) = rb.buf
                    .get_mut(i)
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap()
                    .get_mut(0) {

                    *cell = Self::merge_drop_state(*cell, *d)
                };
            });
    }

    fn drop(col: &mut Column, ticks: u8, mode: Mode) {
        let len = col.borrow().len();

        for col_index in 0..len {
            let next_index = len.saturating_sub(col_index.saturating_add(1));
            let current_index = len.saturating_sub(col_index.saturating_add(2));
            let current = { col.borrow().get(current_index).cloned() };
            let Some(current) = current else { continue; };
            let mut column = col.try_borrow_mut().unwrap();

            'state: for i in 0..current.len() {
                let state = match current.get(i) {
                    Some(s) if ticks % mode.get_frame_by_speed(*s) == 0 => s,
                    _ => continue 'state
                };

                column[current_index] = Self::remove_drop_state(column[current_index], *state);
                column[next_index] = Self::merge_drop_state(column[next_index], *state);
            }
        }
    }

    #[inline]
    fn clean_latest_drop(col: &mut Column) {
        let len = col.borrow().len();
        if len > 0 {
            let mut col = col.try_borrow_mut().unwrap();
            if let Some(c) = col.get_mut(len - 1) {
                c.clear()
            };
        }
    }

    #[inline]
    fn merge_drop_state(mut cell: Cell, state: CellType) -> Cell {
        if !cell.contains(&state) && state != CellType::None {
            let _ = cell.try_push(state);
        };

        cell
    }

    #[inline]
    fn remove_drop_state(cell: Cell, state: CellType) -> Cell {
        cell.into_iter().filter(|c| *c != state).collect()
    }

    #[inline]
    fn get_drop_speed(num: u64, threshold: u16) -> CellType {
        match num % threshold as u64 {
            0 => CellType::Normal,
            1 => CellType::Fast,
            2 => CellType::Slow,
            _ => CellType::None,
        }
    }
}

impl EachFrameImpl for DroppingState {
    fn on_frame(&mut self, rb: &mut super::buffer::RenderBuffer, seed: u64, frame: u8) {
        // each column
        for i in 0..rb.buf.len() {
            Self::clean_latest_drop(&mut rb.buf[i]);
            Self::drop(&mut rb.buf[i], frame, self.mode);
        }

        self.new_drop(rb, seed);
    }
}
