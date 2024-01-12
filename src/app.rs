use anyhow::Result;
use crossterm::{
    cursor,
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};

use crate::{tui::Tui, ui::ui};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DropSpeed {
    Fast,
    Normal,
    Slow,
    #[default]
    None,
}

pub type DropCell = (DropSpeed, DropSpeed, DropSpeed);
pub type DropColumn = Vec<DropCell>;

pub struct State {
    pub buf: Vec<DropColumn>,
    rng: SmallRng,
    ticks: u8,
}

impl State {
    pub fn new(size: Rect) -> Self {
        let buf = Vec::with_capacity(size.width as usize)
            .into_iter()
            .map(|_: DropColumn| Vec::with_capacity(size.height as usize))
            .collect();

        State {
            buf,
            rng: SmallRng::from_entropy(),
            ticks: 0u8,
        }
    }

    pub fn gen_drop(&mut self) -> Vec<DropSpeed> {
        let mut line: Vec<DropSpeed> = Vec::with_capacity(self.buf.len());
        let rng_u64 = self.rng.next_u64();
        for i in 0..64 {
            line.push(if rng_u64 & (1 << i) != 0 {
                State::get_drop_speed(i)
            } else {
                DropSpeed::None
            });
        }

        line
    }

    pub fn tick(&mut self) {
        self.increase_ticks();

        for i in 0..self.buf.len() {
            State::tick_drop(&mut self.buf[i]);
        }
        
        self.tick_new_drop();
    }

    pub fn increase_ticks(&mut self) {
        if self.ticks == u8::MAX {
            self.ticks = 0
        } else {
            self.ticks = self.ticks.saturating_add(1)
        }
    }

    /// generate new drop line
    fn tick_new_drop(&mut self) {
        self.gen_drop()
            .into_iter()
            .enumerate()
            .for_each(|(i, d)| {
                self.buf[i][0] = State::merge_drop_state(self.buf[i][0], d);
            });
    }

    fn tick_drop(column: &mut DropColumn) {
        todo!()
    }

    fn merge_drop_state(cell: DropCell, d: DropSpeed) -> DropCell {
        todo!()
    }

    #[inline]
    fn get_drop_speed(num: i32) -> DropSpeed {
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
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.tui.run();

        loop {
            if let Some(event) = self.tui.next().await {
                todo!()
            }

            self.terminal.draw(|f| ui(f, &mut self.state))?;
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
