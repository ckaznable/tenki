use clap::Parser;

use crate::app::Mode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Mode::Rain)]
    pub mode: Mode,
}

