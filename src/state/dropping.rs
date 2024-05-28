use super::{
    buffer::RenderBuffer,
    CellKind,
    CellKindCollect,
    Column,
    EachFrameImpl,
    Mode,
    ShouldRender
};

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
                    CellKind::None
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

                    cell.kind_collect = Self::merge_drop_state(cell.kind_collect, *d)
                };
            });
    }

    fn drop(col: &mut Column, frame: u64, mode: Mode) {
        let len = col.borrow().len();

        for col_index in 0..len {
            let next_index = len.saturating_sub(col_index.saturating_add(1));
            let current_index = len.saturating_sub(col_index.saturating_add(2));
            let current = { col.borrow().get(current_index).map(|c| c.kind_collect) };
            let Some(current) = current else { continue; };
            let mut column = col.try_borrow_mut().unwrap();

            'state: for i in 0..current.len() {
                let state = match current.get(i) {
                    Some(s) if frame % mode.get_frame_by_cell(*s) == 0 => s,
                    _ => continue 'state
                };

                column[current_index].kind_collect = Self::remove_drop_state(column[current_index].kind_collect, *state);
                column[next_index].kind_collect = Self::merge_drop_state(column[next_index].kind_collect, *state);
            }
        }
    }

    #[inline]
    fn clean_latest_drop(col: &mut Column) {
        let len = col.borrow().len();
        if len > 0 {
            let mut col = col.try_borrow_mut().unwrap();
            if let Some(c) = col.get_mut(len - 1) {
                c.kind_collect.clear()
            };
        }
    }

    #[inline]
    fn merge_drop_state(mut cell: CellKindCollect, state: CellKind) -> CellKindCollect {
        if !cell.contains(&state) && state != CellKind::None {
            let _ = cell.try_push(state);
        };

        cell
    }

    #[inline]
    fn remove_drop_state(cell: CellKindCollect, state: CellKind) -> CellKindCollect {
        cell.into_iter().filter(|c| *c != state).collect()
    }

    #[inline]
    fn get_drop_speed(num: u64, threshold: u16) -> CellKind {
        match num % threshold as u64 {
            0 => CellKind::Normal,
            1 => CellKind::Fast,
            2 => CellKind::Slow,
            _ => CellKind::None,
        }
    }
}

impl EachFrameImpl for DroppingState {
    fn on_frame(&mut self, rb: &mut super::buffer::RenderBuffer, seed: u64, frame: u64) -> ShouldRender {
        // each column
        for i in 0..rb.buf.len() {
            Self::clean_latest_drop(&mut rb.buf[i]);
            Self::drop(&mut rb.buf[i], frame, self.mode);
        }

        self.new_drop(rb, seed);
        ShouldRender::Render
    }
}
