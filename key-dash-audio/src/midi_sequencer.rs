use midi_msg::{ChannelVoiceMsg, Division, Meta, MidiFile, MidiMsg, TimeCodeType, TrackEvent};
use std::{fmt::Display, time::Duration};

/// Ability to receive messages
pub trait MidiSink {
    /// Returns Err if event couldn't be used.
    fn receive_midi(&mut self, msg: &MidiMsg) -> Result<(), ()>;
    fn reset(&mut self);
}

/// [`TrackEvent`] wrapper with some context for debugging.
struct TrackEventWrap {
    track_event: TrackEvent,
    track_idx: usize,
    event_idx: usize,
}

impl Display for TrackEventWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trk = self.track_idx;
        let ev = self.event_idx;
        let event = &self.track_event.event;
        let raw = event.to_midi();
        write!(f, "T{trk}/E{ev} raw: {raw:02X?}, event: {event:?}")
    }
}

/// MIDI Sequencer
#[derive(Debug)]
pub struct MidiSequencer {
    midi_file: Option<MidiFile>,
    bpm: f64,
    /// Index of next event for each track
    track_positions: Vec<usize>,
    /// Song position
    tick: usize,
    since_last_tick: Duration,
    song_length: Duration,
    song_position: Duration,
}

impl MidiSequencer {
    pub const fn new() -> Self {
        Self {
            midi_file: None,
            bpm: 120.,
            track_positions: vec![],
            tick: 0,
            since_last_tick: Duration::ZERO,
            song_length: Duration::ZERO,
            song_position: Duration::ZERO,
        }
    }

    /// Are there no more messages left?
    pub fn end_of_sequence(&self) -> bool {
        let Some(midi_file) = &self.midi_file else {
            println!("bailed: no midi");
            return true;
        };
        for (i, track) in midi_file.tracks.iter().enumerate() {
            if self.track_positions[i] < track.events().len() {
                return false;
            }
        }
        println!("bailed: end reached");
        for (i, track) in midi_file.tracks.iter().enumerate() {
            println!(
                "Track {i:02?} - len: {} position: {}",
                track.events().len(),
                self.track_positions[i]
            );
        }
        true
    }

    pub fn play(&mut self, midi_file: MidiFile) {
        self.tick = 0;
        self.track_positions = vec![0; midi_file.tracks.len()];
        self.midi_file = Some(midi_file);

        self.update_song_length();
    }

    pub fn update_events<R>(&mut self, event_sink: &mut R, delta: Duration)
    where
        R: MidiSink,
    {
        let Some(events) = self.events() else {
            return;
        };

        self.song_position += delta;
        self.since_last_tick += delta;
        let tick_duration = self.current_tick_duration();
        if self.since_last_tick >= tick_duration {
            self.since_last_tick -= tick_duration;
            self.tick += 1;
        }

        for wrap in events {
            match wrap.track_event.event {
                MidiMsg::ChannelVoice { .. }
                | MidiMsg::RunningChannelVoice { .. }
                | MidiMsg::ChannelMode { .. }
                | MidiMsg::RunningChannelMode { .. } => {
                    if event_sink.receive_midi(&wrap.track_event.event).is_err() {
                        println!("Unhandled: {wrap}");
                    }
                }

                midi_msg::MidiMsg::Meta { msg } => self.handle_meta_event(&msg),
                _ => (),
            }
        }
    }

    /// For seeking. Ignore `NoteOn`.
    fn update_events_quiet<R>(&mut self, event_sink: &mut R)
    where
        R: MidiSink,
    {
        let Some(events) = self.events() else {
            return;
        };

        self.song_position += self.current_tick_duration();
        self.tick += 1;

        for wrap in events {
            match wrap.track_event.event {
                MidiMsg::ChannelVoice { msg, .. } | MidiMsg::RunningChannelVoice { msg, .. } => {
                    match msg {
                        ChannelVoiceMsg::NoteOn { .. } | ChannelVoiceMsg::HighResNoteOn { .. } => {}
                        _ => {
                            let _ = event_sink.receive_midi(&wrap.track_event.event);
                        }
                    }
                }
                MidiMsg::ChannelMode { .. } | MidiMsg::RunningChannelMode { .. } => {
                    let _ = event_sink.receive_midi(&wrap.track_event.event);
                }
                midi_msg::MidiMsg::Meta { msg } => self.handle_meta_event(&msg),
                _ => (),
            }
        }
    }

    fn events(&mut self) -> Option<Vec<TrackEventWrap>> {
        let Some(midi_file) = &self.midi_file else {
            return None;
        };

        let mut events = vec![];
        for (track_idx, track) in midi_file.tracks.iter().enumerate() {
            loop {
                let event_idx = self.track_positions[track_idx];
                if event_idx >= track.len() {
                    break;
                }
                let event = &track.events()[event_idx];
                let event_tick = midi_file
                    .header
                    .division
                    .beat_or_frame_to_tick(event.beat_or_frame)
                    as usize;
                if self.tick >= event_tick {
                    events.push(TrackEventWrap {
                        track_event: event.clone(),
                        track_idx,
                        event_idx: self.track_positions[track_idx],
                    });
                    if self.tick > event_tick {
                        let late = self.tick - event_tick;
                        println!(
                            "Somehow an event was missed! Playing it late ({late} ticks). {event:?}"
                        );
                    }
                    self.track_positions[track_idx] += 1;
                } else {
                    break;
                }
            }
        }
        Some(events)
    }

    fn handle_meta_event(&mut self, msg: &Meta) {
        if let Meta::SetTempo(tempo) = msg {
            self.bpm = 60_000_000. / f64::from(*tempo);
        }
    }

    fn current_tick_duration(&self) -> Duration {
        let Some(midi_file) = &self.midi_file else {
            return Duration::ZERO;
        };
        let in_secs = match midi_file.header.division {
            Division::TicksPerQuarterNote(ticks) => 60. / self.bpm / f64::from(ticks),
            Division::TimeCode {
                frames_per_second,
                ticks_per_frame,
            } => {
                let fps = match frames_per_second {
                    TimeCodeType::FPS24 => 24.,
                    TimeCodeType::FPS25 => 25.,
                    TimeCodeType::DF30 | TimeCodeType::NDF30 => 30.,
                };
                1. / fps / f64::from(ticks_per_frame)
            }
        };
        Duration::from_secs_f64(in_secs)
    }

    /// Updates the `song_length` field by calculating the total duration of the MIDI file.
    ///
    /// This function iterates through all events in all tracks of the loaded MIDI file,
    /// respecting tempo changes ([`Meta::SetTempo`] meta messages) and the timing division format
    /// specified in the file header. It simulates playback tick-by-tick and accumulates
    /// the real-world duration based on current tempo and time division.
    ///
    /// If no MIDI file is loaded, sets the duration to `Duration::ZERO`.
    ///
    /// ### Tempo Handling
    ///
    /// - Default tempo is 120 BPM.
    /// - If a [Meta::SetTempo] meta message is encountered, the tempo is updated accordingly.
    ///
    /// ### Time Division Handling
    ///
    /// Supports both:
    ///
    /// - [`Division::TicksPerQuarterNote`]: standard MIDI ticks.
    /// - [`Division::TimeCode`]: SMPTE timecode format.
    ///
    /// ### Performance Note
    ///
    /// This method simulates playback at single-tick resolution,
    /// which may be inefficient for very long MIDI files.
    fn update_song_length(&mut self) {
        let Some(midi_file) = &self.midi_file else {
            self.song_length = Duration::ZERO;
            return;
        };

        let mut track_positions = vec![0; midi_file.tracks.len()];
        let mut tick = 0usize;
        let mut duration = Duration::ZERO;
        let mut bpm = 120.0;

        loop {
            let mut done = true;

            for (i, track) in midi_file.tracks.iter().enumerate() {
                let events = track.events();

                while let Some(event) = events.get(track_positions[i]) {
                    let event_tick = midi_file
                        .header
                        .division
                        .beat_or_frame_to_tick(event.beat_or_frame)
                        as usize;

                    if tick < event_tick {
                        break;
                    }

                    track_positions[i] += 1;
                    done = false;

                    if let MidiMsg::Meta {
                        msg: Meta::SetTempo(tempo),
                    } = &event.event
                    {
                        bpm = 60_000_000. / f64::from(*tempo);
                    }
                }
            }

            if done {
                break;
            }

            let tick_duration = match midi_file.header.division {
                Division::TicksPerQuarterNote(ticks) => 60. / bpm / f64::from(ticks),
                Division::TimeCode {
                    frames_per_second,
                    ticks_per_frame,
                } => {
                    let fps = match frames_per_second {
                        TimeCodeType::FPS24 => 24.,
                        TimeCodeType::FPS25 => 25.,
                        TimeCodeType::DF30 | TimeCodeType::NDF30 => 30.,
                    };
                    1. / fps / f64::from(ticks_per_frame)
                }
            };

            duration += Duration::from_secs_f64(tick_duration);
            tick += 1;
        }

        self.song_length = duration;
    }

    pub const fn song_length(&self) -> Duration {
        self.song_length
    }

    pub const fn song_position(&self) -> Duration {
        self.song_position
    }

    pub fn seek_to<R>(&mut self, event_sink: &mut R, position: Duration)
    where
        R: MidiSink,
    {
        let Some(midi_file) = &self.midi_file else {
            return;
        };

        if position < self.song_position {
            self.bpm = 120.;
            self.track_positions = vec![0; midi_file.tracks.len()];
            self.tick = 0;
            self.song_position = Duration::ZERO;
            event_sink.reset();
        }

        self.since_last_tick = Duration::ZERO;

        while self.song_position < position {
            self.update_events_quiet(event_sink);
        }
    }
}
