use anyhow::Result;
use asfy_bright::{config::Config, ui::bright_bar::BrightBar};
use gtk4::{Application, prelude::*};

fn main() -> Result<()> {
    let config = Config::setup(None).unwrap_or_else(|e| {
        eprintln!("Error trying to get configuration, using default: {}", e);
        Config::default()
    });

    let app = Application::builder()
        .application_id("com.asfy.bright")
        .build();

    app.connect_activate(move |app| {
        let bright_bar = BrightBar::new(app, config.clone());
        bright_bar.listen();
    });

    app.run();
    Ok(())
}
