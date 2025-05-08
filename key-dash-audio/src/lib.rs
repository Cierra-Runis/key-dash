use std::{path::PathBuf, time::Duration};
mod audio;
use audio::AudioPlayer;
use rustysynth::SoundFont;
use strum::Display;

#[derive(Default)]
pub struct Player {
    audio_player: AudioPlayer,
    is_playing: bool,
    soundfont_list: Vec<SoundFont>,
    playlists: Vec<PlayList>,
}

impl Player {
    pub fn toggle_play_pause(&mut self) {
        if self.is_playing {
            self.pause();
        } else {
            self.play();
        }
    }

    fn play(&mut self) -> Result<(), PlayerError> {
        self.audio_player.play()?;
        self.is_playing = true;
        Ok(())
    }

    fn pause(&mut self) -> Result<(), PlayerError> {
        self.audio_player.pause()?;
        self.is_playing = false;
        Ok(())
    }
}

#[derive(Debug, Display)]
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
