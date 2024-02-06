use clap::Parser;
use clap_num::number_range;

use crate::app::Mode;

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
}

fn fps_range(s: &str) -> Result<u8, String> {
    number_range(s, 1, 60)
}

fn level_range(s: &str) -> Result<u16, String> {
    number_range(s, 4, 1000)
}
