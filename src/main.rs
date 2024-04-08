mod app;
mod cli;
mod state;
mod tui;
mod ui;
mod util;
mod weather;
mod widget;

use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Args;
use weather::Weather;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut app = App::new(args, Weather::from(args))?;
    app.run().await?;
    Ok(())
}
