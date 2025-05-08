use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, FromRepr)]
pub enum Tab {
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
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered().border_type(BorderType::Rounded)
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
