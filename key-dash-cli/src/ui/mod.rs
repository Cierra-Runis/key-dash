mod tab;

use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, LineGauge, Tabs, Widget},
};
use strum::IntoEnumIterator;
use tab::Tab;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    #[default]
    Run,
    Quit,
}

/// The main application which holds the state and logic of the application.
#[derive(Default)]
pub struct App {
    mode: Mode,
    selected_tab: Tab,
    // player: Player,
    settings: Settings,
}

impl App {
    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_crossterm_events()?;
        }

        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // It's important to check [`KeyEventKind::Press`] to avoid handling key release events
            CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {
                // Handle other events here if needed.
                Ok(())
            }
        }?;
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match (key.modifiers, key.code) {
            // Add other key handlers here.
            (_, KeyCode::Tab) => self.next_tab(),
            (_, KeyCode::BackTab) => self.previous_tab(),
            // (_, KeyCode::Char(' ')) => self.player.toggle_play_pause(),
            (_, KeyCode::Char('q')) => self.quit(),
            // For testing purposes, you can uncomment the following lines to trigger a panic or an error.
            // (_, KeyCode::Char('p')) => panic!("User triggered panic"),
            // (_, KeyCode::Char('e')) => bail!("User triggered error"),
            _ => {}
        }
        Ok(())
    }
}

impl App {
    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn should_quit(&self) -> bool {
        self.mode == Mode::Quit
    }

    fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}

impl Widget for &App {
    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(3), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let titles = Tab::iter()
            .map(|tab| t!(tab.to_string()))
            .collect::<Vec<_>>();
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .select(selected_tab_index)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Line::from(t!("app.name")).centered()),
            )
            .render(header_area, buf);

        self.selected_tab.render(inner_area, buf);

        LineGauge::default().render(footer_area, buf);
    }
}

#[derive(Debug, Default)]
struct Settings {}
