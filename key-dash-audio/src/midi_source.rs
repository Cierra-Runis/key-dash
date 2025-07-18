use midi_msg::MidiFile;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::{sync::Arc, time::Duration};

use super::midi_sequencer::MidiSequencer;

#[derive(PartialEq)]
enum Channel {
    L,
    R,
}

/// Audio source for Rodio. This takes in soundfont and midi_file, and generates audio samples from
/// them. The disposable struct is consumed by audio sink for each song.
pub struct MidiSource {
    /// The actual audio generator
    synthesizer: Synthesizer,
    /// The midi file sequencer
    sequencer: MidiSequencer,
    /// Sample time
    delta: Duration,
    /// We need to cache the R channel sample.
    cached_sample: f32,
    /// Which channel was played last
    next_channel: Channel,
}

impl MidiSource {
    const DEFAULT_SAMPLE_RATE: i32 = 44100;

    /// New `MidiSource` that immediately starts playing.
    pub fn new(soundfont: &Arc<SoundFont>, midi_file: MidiFile) -> Self {
        Self::with_sample_rate(soundfont, midi_file, Self::DEFAULT_SAMPLE_RATE)
    }

    /// New `MidiSource` that immediately starts playing.
    fn with_sample_rate(soundfont: &Arc<SoundFont>, midi_file: MidiFile, sample_rate: i32) -> Self {
        let settings = SynthesizerSettings::new(sample_rate);
        let mut synthesizer =
            Synthesizer::new(soundfont, &settings).expect("Could not create synthesizer");
        synthesizer.set_master_volume(1.0);
        let mut sequencer = MidiSequencer::new();
        sequencer.play(midi_file);

        let delta_t = Duration::from_secs_f64(1. / f64::from(synthesizer.get_sample_rate()));
        Self {
            synthesizer,
            delta: delta_t,
            sequencer,
            next_channel: Channel::L,
            cached_sample: 0.,
        }
    }

    pub const fn song_length(&self) -> Duration {
        self.sequencer.song_length()
    }
}

// Rodio requires Iterator implementation.
// This is where whe generate the next samples.
impl Iterator for MidiSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sequencer.end_of_sequence() {
            return None;
        }

        // The midi synth generates bot L and R samples simultaneously, but Rodio polls samples
        // separately for each channel.

        // Left: generate both channels and store R channel sample.
        if self.next_channel == Channel::L {
            self.next_channel = Channel::R;

            self.sequencer
                .update_events(&mut self.synthesizer, self.delta);

            let mut left = [0.];
            let mut right = [0.];
            self.synthesizer.render(&mut left, &mut right);

            self.cached_sample = right[0] / 10.;
            Some(left[0] / 10.)
        }
        // Right: Generate nothing and return cached R ch. sample.
        else {
            self.next_channel = Channel::L;
            Some(self.cached_sample)
        }
    }
}

impl rodio::Source for MidiSource {
    fn current_frame_len(&self) -> Option<usize> {
        let time_left = self.sequencer.song_length() - self.sequencer.song_position();
        let samples_left = time_left.as_secs_f64() * f64::from(self.synthesizer.get_sample_rate());
        Some(samples_left as usize)
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        Self::DEFAULT_SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.sequencer.song_length())
    }

    fn try_seek(&mut self, position: Duration) -> Result<(), rodio::source::SeekError> {
        self.sequencer.seek_to(&mut self.synthesizer, position);
        Ok(())
    }
}
