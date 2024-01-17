mod app;
mod tui;
mod ui;
mod util;
mod widget;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run().await?;
    Ok(())
}
