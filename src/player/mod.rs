use std::{path::PathBuf, time::Duration};
mod audio;
use audio::AudioPlayer;
use rustysynth::SoundFont;

#[derive(Default)]
pub struct Player {
    audio_player: AudioPlayer,
    is_playing: bool,
    soundfont_list: Vec<SoundFont>,
    playlists: Vec<PlayList>,
}

impl Player {}

pub enum PlayerError {
    AudioNoSink,
    AudioNoFont,
    AudioNoMidi,
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
