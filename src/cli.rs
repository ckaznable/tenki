use std::str::FromStr;

use clap::Parser;
use clap_num::number_range;
use ratatui::style::Color;
use crate::app::{Mode, WindMode};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Mode::Rain)]
    pub mode: Mode,

    #[arg(long, value_parser = fps_range, default_value_t = 60)]
    pub fps: u8,

    /// effect level, The lower, the stronger [4-1000]
    #[arg(short, long, value_parser = level_range, default_value_t = 50)]
    pub level: u16,

    /// color of the effect. [red, green, blue, yellow, cyan, magenta, white, black]
    #[arg(long, value_parser = Color::from_str, default_value = "white")]
    pub timer_color: Color,

    /// wind mode. [random, disable, only-right, only-left]
    #[arg(long, value_parser = WindMode::from_str, default_value = "random")]
    pub wind: WindMode,
}

fn fps_range(s: &str) -> Result<u8, String> {
    number_range(s, 1, 60)
}

fn level_range(s: &str) -> Result<u16, String> {
    number_range(s, 4, 1000)
}
