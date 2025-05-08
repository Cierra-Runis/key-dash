use midi_msg::{MidiFile, MidiMsg};
use midi_source::MidiSource;
use rodio::Sink;
use rustysynth::SoundFont;
use std::{sync::Arc, time::Duration};
use strum::Display;

mod midi_sequencer;
mod midi_source;
mod midi_synth;

#[derive(Default)]
pub struct Player {
    soundfont: Option<Arc<SoundFont>>,
    midi_file: Option<MidiFile>,
    midi_duration: Option<Duration>,
    sink: Option<Sink>,
    msg_callback: Option<Box<dyn Fn(MidiMsg)>>,
}

impl Player {
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

impl Player {
    /// Unpause
    pub fn play(&self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::NoSink);
        };
        sink.play();
        Ok(())
    }

    /// Pause
    pub fn pause(&self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::NoSink);
        };
        sink.pause();
        Ok(())
    }

    /// Load currently selected midi & font and start playing
    pub fn start_playback(&mut self) -> Result<(), PlayerError> {
        let Some(soundfont) = &self.soundfont else {
            return Err(PlayerError::NoFont);
        };
        let Some(midi_file) = self.midi_file.clone() else {
            return Err(PlayerError::NoMidi);
        };
        let Some(sink) = &self.sink else {
            return Err(PlayerError::NoSink);
        };
        let source = MidiSource::new(soundfont, midi_file);
        self.midi_duration = Some(source.song_length());

        sink.append(source);
        sink.play();
        Ok(())
    }

    /// Full stop.
    pub fn stop_playback(&mut self) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::NoSink);
        };
        self.midi_duration = None;
        sink.clear();
        sink.pause();
        Ok(())
    }

    pub fn seek_to(&self, position: Duration) -> Result<(), PlayerError> {
        let Some(sink) = &self.sink else {
            return Err(PlayerError::NoSink);
        };
        let _ = sink.try_seek(position);
        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum PlayerError {
    NoSink,
    NoFont,
    NoMidi,
    NoDevice,
}
