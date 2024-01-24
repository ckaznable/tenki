mod app;
mod cli;
mod tui;
mod ui;
mod util;
mod widget;

use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut app = App::new(args)?;
    app.run().await?;
    Ok(())
}

