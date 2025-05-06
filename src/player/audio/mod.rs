use midi_msg::MidiFile;
use midi_source::MidiSource;
use rodio::Sink;
use rustysynth::SoundFont;
use std::{sync::Arc, time::Duration};

mod midi_sequencer;
mod midi_source;
mod midi_synth;

use super::PlayerError;

#[derive(Default)]
pub struct AudioPlayer {
    soundfont: Option<Arc<SoundFont>>,
    midi_file: Option<MidiFile>,
    midi_duration: Option<Duration>,
    sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn set_sink(&mut self, value: Option<Sink>) {
        if let Some(ref sink) = value {
            sink.pause();
        }
        self.sink = value;
    }

    pub fn set_soundfont(&mut self, value: Arc<SoundFont>) {
        self.soundfont = Some(value);

        if let Some(sink) = &self.sink {
            if !sink.empty() {
                let position = sink.get_pos();
                sink.clear();
                let _ = self.start_playback();
                let _ = self.seek_to(position);
            }
        }
    }
}

impl AudioPlayer {
    /// Unpause
    pub(crate) fn play(&self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::AudioNoSink);
        };
        sink.play();
        Ok(())
    }

    /// Pause
    pub(crate) fn pause(&self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::AudioNoSink);
        };
        sink.pause();
        Ok(())
    }

    /// Load currently selected midi & font and start playing
    pub(crate) fn start_playback(&mut self) -> Result<(), PlayerError> {
        let Some(soundfont) = &self.soundfont else {
            return Err(PlayerError::AudioNoFont);
        };
        let Some(midi_file) = self.midi_file.clone() else {
            return Err(PlayerError::AudioNoMidi);
        };
        let Some(sink) = &self.sink else {
            return Err(PlayerError::AudioNoSink);
        };
        let source = MidiSource::new(soundfont, midi_file);
        self.midi_duration = Some(source.song_length());

        sink.append(source);
        sink.play();
        Ok(())
    }

    /// Full stop.
    pub(crate) fn stop_playback(&mut self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::AudioNoSink);
        };
        self.midi_duration = None;
        sink.clear();
        sink.pause();
        Ok(())
    }

    pub(crate) fn seek_to(&self, position: Duration) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::AudioNoSink);
        };
        let _ = sink.try_seek(position);
        Ok(())
    }
}
