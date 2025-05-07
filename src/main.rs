use configs::{
    config_file::ConfigFile,
    custom::{
        app_custom::AppConfig, keymap_custom::KeymapConfig, logger_custom::LoggerConfig,
        palette_custom::PaletteConfig, theme_custom::ThemeConfig,
    },
};
use lazy_static::lazy_static;

pub mod action;
pub mod app_error;
pub mod component_name;
pub mod configs;
pub mod event;
pub mod player;
pub mod ui;
pub mod utils;

lazy_static! {
    pub static ref LOGGER_CONFIG: LoggerConfig = LoggerConfig::get_config();
    pub static ref KEYMAP_CONFIG: KeymapConfig = KeymapConfig::get_config();
    pub static ref APP_CONFIG: AppConfig = AppConfig::get_config();
    pub static ref PALETTE_CONFIG: PaletteConfig = PaletteConfig::get_config();
    pub static ref THEME_CONFIG: ThemeConfig = ThemeConfig::get_config();
}

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Or you can pass arguments by `i18n!` to override it.
// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
i18n!("locales", fallback = "en");

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // Initialize the lazy static variables
    // This is done to ensure that the configuration files are read only once
    // and the values are shared across the application.
    lazy_static::initialize(&LOGGER_CONFIG);
    lazy_static::initialize(&KEYMAP_CONFIG);
    lazy_static::initialize(&APP_CONFIG);
    lazy_static::initialize(&PALETTE_CONFIG);
    lazy_static::initialize(&THEME_CONFIG);

    // See: [panic example](https://github.com/ratatui/ratatui/blob/main/examples/apps/panic/src/main.rs)
    color_eyre::install()?;

    // Initialize language
    let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
    rust_i18n::set_locale(&locale);

    // Initialize the terminal
    //
    // - Line-buffered behavior is disabled
    // - Input passed directly to the application as it is typed
    // - No line editing
    // - No input echoing
    // - No special key combinations (e.g. Ctrl+C)
    //
    // See: [`ratatui::init`]
    let terminal = ratatui::init();

    let result = ui::App::default().run(terminal);

    restore();
    result
}

/// Restores the terminal to its original state.
///
/// See: [`ratatui::restore`]
fn restore() {
    if let Err(err) = ratatui::try_restore() {
        // There's not much we can do if restoring the terminal fails, so we just print the error
        eprintln!(
            "Failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }
}
