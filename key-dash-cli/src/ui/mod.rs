mod tab;

use std::time::Duration;

use color_eyre::{Result, eyre::Ok};
use crossterm::event::{
    Event as CrosstermEvent, EventStream as CrosstermStream, KeyCode, KeyEvent, KeyEventKind,
};
use key_dash_audio::Player;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    text::Text,
    widgets::Widget,
};
use tokio_stream::StreamExt;

/// The main application which holds the state and logic of the application.
#[derive(Default)]
pub struct App {
    should_quit: bool,
    player: Player,
}

impl App {
    // For controlling frame generate speed
    const FRAMES_PER_SECOND: f32 = 120.0;

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut crosstermStream = CrosstermStream::new();
        // let mut playerStream = self.player;

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => {
                  terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
                },
                Some(event) = crosstermStream.next() => {
                  self.handle_crossterm_events(event?)?;
                },
            }
        }

        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self, event: CrosstermEvent) -> Result<()> {
        match event {
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
            // (_, KeyCode::Char(' ')) => self.player.toggle_play_pause(),
            (_, KeyCode::Char('q')) => self.should_quit = true,
            // For testing purposes, you can uncomment the following lines to trigger a panic or an error.
            // (_, KeyCode::Char('p')) => panic!("User triggered panic"),
            // (_, KeyCode::Char('e')) => bail!("User triggered error"),
            _ => {}
        }
        Ok(())
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
        let text = Text::raw("TODO");
        let area = center(
            area,
            Constraint::Length(text.width() as u16),
            Constraint::Length(1),
        );
        text.render(area, buf);
    }
}

#[derive(Debug, Default)]
struct Settings {}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
