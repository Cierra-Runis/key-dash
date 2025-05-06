use std::{path::PathBuf, sync::Arc, time::Duration};

use rustysynth::SoundFont;

#[derive(Debug, Default)]
pub struct Player {
    audio_player: AudioPlayer,
    is_playing: bool,
    soundfont_list: Vec<SoundFont>,
    playlists: Vec<PlayList>,
}

#[derive(Debug, Default)]
struct AudioPlayer {
    soundfont: Option<Arc<SoundFont>>,
}

#[derive(Debug, Default)]
struct PlayList {
    name: String,
    description: String,
    midis: Vec<MidiMeta>,
}

#[derive(Debug, Default)]
struct MidiMeta {
    file_path: PathBuf,
    file_size: Option<u64>,
    duration: Option<Duration>,
}
