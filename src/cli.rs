use clap::Parser;
use clap_num::number_range;

use crate::app::Mode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Mode::Rain)]
    pub mode: Mode,

    #[arg(long, value_parser = fps_range, default_value_t = 30)]
    pub fps: u8,
}

fn fps_range(s: &str) -> Result<u8, String> {
    number_range(s, 1, 60)
}
