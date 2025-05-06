mod tab;

use crate::player::Player;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Tabs, Widget},
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
#[derive(Debug, Default)]
pub struct App {
    mode: Mode,
    selected_tab: Tab,
    player: Player,
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
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Tab) => self.next_tab(),
            (_, KeyCode::BackTab) => self.previous_tab(),
            (_, KeyCode::Char('q')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
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

fn render_title(area: Rect, buf: &mut Buffer) {
    let app_name = t!("app.name");
    Line::from(app_name.blue())
        .bold()
        .right_aligned()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let app_description = t!("app.description");
    Line::from(app_description).centered().render(area, buf);
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
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Min(1)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = Tab::iter()
            .map(|tab| t!(tab.to_string()))
            .collect::<Vec<_>>();
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .select(selected_tab_index)
            .highlight_style(Style::default().yellow())
            .render(area, buf);
    }
}

#[derive(Debug, Default)]
struct Settings {}
