use clap::Parser;
use cli::Cli;

use crate::app::App;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
pub mod ingress;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    // TODO: handle this
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.init().await?;
    app.run().await?;
    Ok(())
}
