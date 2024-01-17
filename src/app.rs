use std::{rc::Rc, cell::RefCell};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use tinyvec::ArrayVec;

use crate::{tui::{Tui, Event}, ui::ui};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DropSpeed {
    Fast,
    Normal,
    Slow,
    #[default]
    None,
}

const DROP_TICK_NORMAL: u8 = 4;
const DROP_TICK_SLOW: u8 = 6;

pub type DropCell = ArrayVec<[DropSpeed; 3]>;
pub type DropColumn = Rc<RefCell<Vec<DropCell>>>;

pub struct State {
    pub buf: Vec<DropColumn>,
    pub ticks: u8,
    rng: SmallRng,
}

impl State {
    pub fn new(size: Rect) -> Self {
        let mut buf = Vec::with_capacity(size.width as usize);
        for _ in 0..size.width {
            let mut column = Vec::with_capacity(size.height as usize);
            for _ in 0..size.height {
                column.push(ArrayVec::<[DropSpeed; 3]>::default());
            }

            buf.push(Rc::new(RefCell::new(column)));
        }

        State {
            buf,
            rng: SmallRng::from_entropy(),
            ticks: 0u8,
        }
    }

    pub fn tick(&mut self) {
        self.increase_ticks();

        // each column
        for i in 0..self.buf.len() {
            Self::clean_latest_drop(&mut self.buf[i]);
            Self::tick_drop(&mut self.buf[i], self.ticks);
        }

        self.tick_new_drop();
    }

    fn gen_drop(&mut self) -> Vec<DropSpeed> {
        let mut line: Vec<DropSpeed> = Vec::with_capacity(self.buf.len());
        let rng_u64 = self.rng.next_u64();
        for i in 0..64u64 {
            line.push(if rng_u64 & (1 << i) != 0 {
                Self::get_drop_speed(i)
            } else {
                DropSpeed::None
            });
        }

        line
    }

    fn increase_ticks(&mut self) {
        if self.ticks == 252 {
            self.ticks = 1
        } else {
            self.ticks = self.ticks.saturating_add(1)
        }
    }

    /// generate new drop line
    fn tick_new_drop(&mut self) {
        let _ = self.gen_drop()
            .into_iter()
            .enumerate()
            .filter_map(|(i, d)| {
                self.buf
                    .get_mut(i)?
                    .try_borrow_mut()
                    .ok()?
                    .get_mut(i)
                    .map(|cell| Self::merge_drop_state(*cell, d));

                Some(())
            });
    }

    fn tick_drop(col: &mut DropColumn, ticks: u8) {
        let len = { col.borrow().len() };

        for col_index in 0..len {
            let dist = col.clone();
            let dist = dist.borrow();
            let Some(dist_state) = dist.get(len.saturating_sub(col_index + 1)) else {
                continue;
            };

            let previous = col.clone();
            let previous = previous.borrow();
            let Some(previous) = previous.get(len.saturating_sub(col_index + 2)) else {
                continue;
            };

            let Ok(mut column) = col.try_borrow_mut() else {
                continue;
            };

            for p_index in 0..previous.len() {
                let state = match previous.get(p_index) {
                    Some(DropSpeed::Fast) => DropSpeed::Fast,
                    Some(DropSpeed::Normal) if ticks % DROP_TICK_NORMAL == 0 => DropSpeed::Normal,
                    Some(DropSpeed::Slow) if ticks % DROP_TICK_SLOW == 0 => DropSpeed::Slow,
                    _ => continue
                };

                column[len.saturating_sub(p_index + 1)] = Self::merge_drop_state(*dist_state, state);
                column[len.saturating_sub(p_index + 2)] = Self::remove_drop_state(*dist_state, state);
            }
        }
    }

    #[inline]
    fn clean_latest_drop(col: &mut DropColumn) {
        let len = col.borrow().len();
        if len > 0 {
            let mut col = col.try_borrow_mut().unwrap();
            if let Some(c) = col.get_mut(len - 1) {
                c.clear()
            };
        }
    }

    #[inline]
    fn merge_drop_state(mut cell: DropCell, d: DropSpeed) -> DropCell {
        if !cell.contains(&d) {
            cell.push(d);
        };

        cell
    }

    #[inline]
    fn remove_drop_state(cell: DropCell, d: DropSpeed) -> DropCell {
        cell.into_iter().filter(|c| *c != d).collect()
    }

    #[inline]
    fn get_drop_speed(num: u64) -> DropSpeed {
        match num % 3 {
            0 => DropSpeed::Normal,
            1 => DropSpeed::Fast,
            2 => DropSpeed::Slow,
            _ => DropSpeed::None,
        }
    }
}

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    tui: Tui,
    state: State,
    should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let state = State::new(terminal.size()?);

        Ok(Self {
            terminal,
            state,
            tui: Tui::new()?,
            should_quit: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        use Event::*;
        self.tui.run();

        loop {
            if let Some(event) = self.tui.next().await {
                match event {
                    Init => (),
                    Quit => self.should_quit = true,
                    Error => self.should_quit = true,
                    Render => self.state.tick(),
                    Key(key) => self.handle_keyboard(key),
                    Timer => (),
                    Resize(_, _) => (),
                };
            };

            if self.should_quit {
                break;
            }

            self.terminal.draw(|f| ui(f, &mut self.state))?;
        };

        Ok(())
    }

    fn handle_keyboard(&mut self, key: KeyEvent) {
        if let KeyCode::Char('q') = key.code {
            self.should_quit = true; 
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // restore terminal
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = disable_raw_mode();
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture,
                cursor::Show
            );
        }
    }
}
