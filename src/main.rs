use crate::app::App;

pub(crate) mod app;
pub(crate) mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = App::new()?;
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
