use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    #[default]
    Run,
    Quit,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, FromRepr)]
enum Tab {
    #[default]
    #[strum(to_string = "app.tab.player")]
    Player,
    #[strum(to_string = "app.tab.playlist")]
    Playlist,
    #[strum(to_string = "app.tab.soundfont")]
    SoundFont,
    #[strum(to_string = "app.tab.settings")]
    Settings,
    #[strum(to_string = "app.tab.about")]
    About,
}

impl Tab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::default().padding(Padding::uniform(1))
    }
}

impl Widget for Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Player => Paragraph::new("Player")
                .block(self.block())
                .render(area, buf),
            Self::Playlist => Paragraph::new("Playlist")
                .block(self.block())
                .render(area, buf),
            Self::SoundFont => Paragraph::new("SoundFont")
                .block(self.block())
                .render(area, buf),
            Self::Settings => Paragraph::new("Settings")
                .block(self.block())
                .render(area, buf),
            Self::About => Paragraph::new("About")
                .block(self.block())
                .render(area, buf),
        }
    }
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    mode: Mode,
    selected_tab: Tab,
}

impl App {
    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_crossterm_events()?;

            if self.should_quit() {
                break;
            }
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
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        use Constraint::{Length, Min, Percentage};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Percentage(100), Min(20)]);
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
            .render(area, buf);
    }
}
