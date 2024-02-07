use std::{rc::Rc, cell::RefCell, time::SystemTime, fmt::Display};

use anyhow::Result;
use chrono::{DateTime, Local, Timelike};
use clap::ValueEnum;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use ratatui::{backend::CrosstermBackend, layout::Rect, style::Color, Terminal};
use tinyvec::ArrayVec;

use crate::{tui::{Tui, Event}, ui::ui, cli::Args};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum DropSpeed {
    Fast,
    Normal,
    Slow,
    #[default]
    None,
}

#[derive(Copy, Clone, Default, ValueEnum)]
pub enum Mode {
    #[default]
    Rain,
    Snow
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Mode::Rain => "rain",
            Mode::Snow => "snow",
        };

        s.fmt(f)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum Wind {
    Left(u8),
    Right(u8),
    #[default]
    None,
}

impl Wind {
    pub fn tick(self) -> Self {
        match self {
            Wind::None => Wind::None,
            Wind::Left(tick) => if tick == 0 { Wind::None } else { Wind::Left(tick.saturating_sub(1)) },
            Wind::Right(tick) => if tick == 0 { Wind::None } else { Wind::Right(tick.saturating_sub(1)) },
        }
    }
}

const DROP_TICK_NORMAL: u8 = 2;
const DROP_TICK_SLOW: u8 = 3;

pub type DropCell = ArrayVec<[DropSpeed; 3]>;
pub type DropColumn = Rc<RefCell<Vec<DropCell>>>;

#[derive(Default, Copy, Clone)]
pub struct Timer {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub color: Color,
}

impl Timer {
    pub fn hours(mut self, h: u8) -> Self {
        self.hours = h;
        self
    }

    pub fn minutes(mut self, m: u8) -> Self {
        self.minutes = m;
        self
    }

    pub fn seconds(mut self, s: u8) -> Self {
        self.seconds = s;
        self
    }
}

pub struct State {
    pub buf: Vec<DropColumn>,
    pub timer: Timer,
    pub mode: Mode,
    pub wind: Wind,
    buf_line: Vec<DropSpeed>,
    ticks: u8,
    rng: SmallRng,
    seed: u64,
    threshold: u64,
}

impl State {
    pub fn new(size: Rect, mode: Mode, threshold: u64, timer_color: Color) -> Self {
        let (buf, buf_line) = Self::init_buf(size);

        State {
            buf,
            buf_line,
            rng: SmallRng::from_entropy(),
            ticks: 0u8,
            timer: Timer {
                color: timer_color,
                ..Default::default()
            },
            threshold,
            mode,
            wind: Wind::None,
            seed: 0,
        }
    }

    pub fn on_resize(&mut self, columns: u16, rows: u16) {
        let (buf, buf_line) = Self::init_buf(Rect {
            x: 0,
            y: 0,
            height: rows,
            width: columns,
        });

        self.buf = buf;
        self.buf_line = buf_line;
    }

    pub fn tick_timer(&mut self) {
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        self.timer = self.timer
            .hours(datetime.hour() as u8)
            .minutes(datetime.minute() as u8)
            .seconds(datetime.second() as u8)
    }

    pub fn tick(&mut self) {
        self.gen_seed();
        self.increase_ticks();
        self.tick_wind();

        // each column
        for i in 0..self.buf.len() {
            Self::clean_latest_drop(&mut self.buf[i]);
            Self::tick_drop(&mut self.buf[i], self.ticks);
        }

        self.tick_new_drop();
    }

    fn tick_wind(&mut self) {
        if self.buf.len() == 1 {
            return;
        }

        self.wind = self.wind.tick();

        if self.wind == Wind::None {
            if self.seed % 2024 == 0 {
                self.wind = Wind::Left(255);
            } else if self.seed % 123 == 0 {
                self.wind = Wind::Right(255);
            } else {
                return;
            }
        }

        if let Wind::Left(_) = self.wind {
            self.buf.reverse();
        }

        for i in 1..self.buf.len() {
            self.buf.swap(0, i);
        }

        if let Wind::Left(_) = self.wind {
            self.buf.reverse();
        }
    }

    fn init_buf(size: Rect) -> (Vec<DropColumn>, Vec<DropSpeed>) {
        let mut buf = Vec::with_capacity(size.width as usize);
        for _ in 0..size.width {
            let mut column = Vec::with_capacity(size.height as usize);
            for _ in 0..size.height {
                column.push(ArrayVec::<[DropSpeed; 3]>::default());
            }

            buf.push(Rc::new(RefCell::new(column)));
        }

        let len = buf.len();
        (buf, Vec::with_capacity(len))
    }

    fn gen_seed(&mut self) {
        self.seed = self.rng.next_u64();
    }

    fn gen_drop(&mut self) {
        self.buf_line.clear();

        const GROUP_SIZE: u64 = 64;
        let len = self.buf.len() as u64;
        let last_group = len % GROUP_SIZE;
        let groups = len / GROUP_SIZE + if last_group > 0 { 1 } else { 0 };

        for g in 0..groups {
            let range = if groups.saturating_sub(1) == g { last_group } else { GROUP_SIZE };
            for i in 0..range {
                self.buf_line.push(if self.seed & (1 << i) != 0 {
                    Self::get_drop_speed(self.seed.saturating_sub(i), self.threshold)
                } else {
                    DropSpeed::None
                });
            }
        }
    }

    #[inline]
    fn increase_ticks(&mut self) {
        self.ticks = if self.ticks == 240 { 1 } else { self.ticks.saturating_add(1) }
    }

    /// generate new drop line
    fn tick_new_drop(&mut self) {
        self.gen_drop();
        self.buf_line
            .iter()
            .enumerate()
            .for_each(|(i, d)| {
                if let Some(cell) = self.buf
                    .get_mut(i)
                    .unwrap()
                    .try_borrow_mut()
                    .ok()
                    .unwrap()
                    .get_mut(0) {

                    *cell = Self::merge_drop_state(*cell, *d)
                };
            });
    }

    fn tick_drop(col: &mut DropColumn, ticks: u8) {
        let len = col.borrow().len();

        for col_index in 0..len {
            let next_index = len.saturating_sub(col_index.saturating_add(1));
            let current_index = len.saturating_sub(col_index.saturating_add(2));
            let current = { col.borrow().get(current_index).cloned() };
            let Some(current) = current else { continue; };
            let mut column = col.try_borrow_mut().unwrap();

            'state: for i in 0..current.len() {
                let state = match current.get(i) {
                    Some(DropSpeed::Fast) => DropSpeed::Fast,
                    Some(DropSpeed::Normal) if ticks % DROP_TICK_NORMAL == 0 => DropSpeed::Normal,
                    Some(DropSpeed::Slow) if ticks % DROP_TICK_SLOW == 0 => DropSpeed::Slow,
                    _ => continue 'state
                };

                column[current_index] = Self::remove_drop_state(column[current_index], state);
                column[next_index] = Self::merge_drop_state(column[next_index], state);
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
    fn merge_drop_state(mut cell: DropCell, state: DropSpeed) -> DropCell {
        if !cell.contains(&state) && state != DropSpeed::None {
            cell.push(state);
        };

        cell
    }

    #[inline]
    fn remove_drop_state(cell: DropCell, state: DropSpeed) -> DropCell {
        cell.into_iter().filter(|c| *c != state).collect()
    }

    #[inline]
    fn get_drop_speed(num: u64, threshold: u64) -> DropSpeed {
        match num % threshold {
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
    pub fn new(args: Args) -> Result<Self> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let state = State::new(
            terminal.size()?, 
            args.mode, 
            args.level as u64, 
            args.timer_color);

        Ok(Self {
            terminal,
            state,
            tui: Tui::new(args.fps as f64)?,
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
                    Timer => self.state.tick_timer(),
                    Resize(columns, rows) => self.state.on_resize(columns, rows),
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
