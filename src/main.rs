use app::App;

mod app;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Or you can pass arguments by `i18n!` to override it.
i18n!("locales");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Initialize language
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    rust_i18n::set_locale(&locale);

    // Initialize the terminal
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
