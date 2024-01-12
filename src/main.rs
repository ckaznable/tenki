mod app;
mod tui;
mod ui;
mod util;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new()?;
    app.run().await?;
    Ok(())
}
