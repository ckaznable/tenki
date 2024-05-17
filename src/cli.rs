use std::str::FromStr;

use clap::Parser;
use clap_num::number_range;
use ratatui::style::Color;

use crate::state::{timer::TimerMode, wind::WindMode, Mode};

#[derive(Parser, Clone, Copy)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Mode::Rain)]
    pub mode: Mode,

    #[arg(long)]
    pub timer_mode: Option<TimerMode>,

    /// color of the effect. [red, green, blue, yellow, cyan, magenta, white, black]
    #[arg(long, value_parser = Color::from_str, default_value = "white")]
    pub timer_color: Color,

    /// frame per second
    #[arg(short, long, value_parser = process_rate_range, default_value_t = 60)]
    pub fps: u8,

    /// tick per second
    #[arg(short, long, value_parser = process_rate_range, default_value_t = 60)]
    pub tps: u8,

    /// effect level, The lower, the stronger [4-1000]
    #[arg(short, long, value_parser = level_range)]
    pub level: Option<u16>,

    /// wind mode. [random, disable, only-right, only-left, right, left]
    #[arg(long, value_parser = WindMode::from_str, default_value = "random")]
    pub wind: WindMode,

    /// show fps at right-top in screen
    #[arg(long)]
    pub show_fps: bool,

    /// blinking colon of timer
    #[arg(long)]
    pub blink_colon: bool,
}

fn process_rate_range(s: &str) -> Result<u8, String> {
    number_range(s, 1, 240)
}

fn level_range(s: &str) -> Result<u16, String> {
    number_range(s, 0, 1000)
}
