extern crate pitch_calc as pitch;

pub use pitch::{Letter, Octave};

pub type Velocity = f32;

/// A struct used for creating musical `Note`s via the computer keyboard.
#[derive(Clone, Debug)]
pub struct MusicalKeyboard {
    /// The current base octave for the keyboard.
    pub octave: Octave,
    /// The current velocity for the generated notes.
    pub velocity: Velocity,
    /// The currently pressed keys.
    pub currently_pressed_keys: std::collections::HashMap<Key, Octave>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoteOn {
    pub letter: Letter,
    pub octave: Octave,
    pub velocity: Velocity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoteOff {
    pub letter: Letter,
    pub octave: Octave,
}

/// The event that is returned from 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteEvent {
    On(NoteOn),
    Off(NoteOff),
}

/// Keys accepted by the keyboard.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Key {
    // Keys associated with `Note`s.
    A,
    W,
    S,
    E,
    D,
    F,
    T,
    G,
    Y,
    H,
    U,
    J,
    K,
    O,
    L,
    P,
    Semicolon,
    Quote,

    // Octave.
    Z,
    X,

    // Velocity.
    C,
    V,
}

impl Default for MusicalKeyboard {
    fn default() -> Self {
        MusicalKeyboard::new(2, 1.0)
    }
}

impl From<NoteOn> for NoteEvent {
    fn from(on: NoteOn) -> Self {
        NoteEvent::On(on)
    }
}

impl From<NoteOff> for NoteEvent {
    fn from(off: NoteOff) -> Self {
        NoteEvent::Off(off)
    }
}

impl MusicalKeyboard {

    /// Constructor for MusicalKeyboard.
    pub fn new(octave: Octave, velocity: Velocity) -> Self {
        MusicalKeyboard {
            octave: octave,
            velocity: velocity,
            currently_pressed_keys: std::collections::HashMap::new(),
        }
    }

    /// Return a NoteOn given some pressed key.
    ///
    /// - Z will step the octave down.
    /// - X will step the octave up.
    /// - C will step the velocity down.
    /// - V will step the velocity up.
    /// - Home-row and some of the top row will trigger notes or release them depending on is_pressed.
    pub fn key_pressed(&mut self, key: Key) -> Option<NoteOn> {
        match key {
            Key::Z => if self.octave > -2 { self.octave -= 1 },
            Key::X => if self.octave < 12 { self.octave += 1 },
            Key::C => if self.velocity > 0.0 { self.velocity -= 0.05 },
            Key::V => if self.velocity < 1.0 { self.velocity += 0.05 },
            other => return self.maybe_note_on(other),
        }
        None
    }

    /// Return a NoteOff given some released key.
    pub fn key_released(&mut self, key: Key) -> Option<NoteOff> {
        self.maybe_note_off(key)
    }

    /// Translates a key into it's respective note.
    /// This key pattern is an attempt at modelling a piano's keys, where Key::A is a piano's C.
    pub fn maybe_note(&mut self, key: Key) -> Option<(Letter, Octave)> {
        let (octave, letter): (Octave, Letter) = match key {
            Key::A         => (0, Letter::C),
            Key::W         => (0, Letter::Csh),
            Key::S         => (0, Letter::D),
            Key::E         => (0, Letter::Dsh),
            Key::D         => (0, Letter::E),
            Key::F         => (0, Letter::F),
            Key::T         => (0, Letter::Fsh),
            Key::G         => (0, Letter::G),
            Key::Y         => (0, Letter::Gsh),
            Key::H         => (0, Letter::A),
            Key::U         => (0, Letter::Ash),
            Key::J         => (0, Letter::B),
            Key::K         => (1, Letter::C),
            Key::O         => (1, Letter::Csh),
            Key::L         => (1, Letter::D),
            Key::P         => (1, Letter::Dsh),
            Key::Semicolon => (1, Letter::E),
            Key::Quote     => (1, Letter::F),
            _ => return None,
        };
        Some((letter, octave + self.octave))
    }

    /// Translates a pressed key to a note on event.
    ///
    /// If the given key is already pressed, it is ignored. This helps to avoid triggering notes
    /// from a window's key-repeat function.
    pub fn maybe_note_on(&mut self, key: Key) -> Option<NoteOn> {
        self.maybe_note(key).and_then(|(letter, octave)| {
            match self.currently_pressed_keys.insert(key, octave) {
                Some(_existing_note) => None,
                None => Some(NoteOn {
                    letter: letter,
                    octave: octave,
                    velocity: self.velocity,
                }),
            }
        })
    }

    /// Translates a released key to a note off event.
    pub fn maybe_note_off(&mut self, key: Key) -> Option<NoteOff> {
        self.maybe_note(key).map(|(letter, octave)| {
            match self.currently_pressed_keys.remove(&key) {
                None             => NoteOff { letter: letter, octave: octave },
                Some(old_octave) => NoteOff { letter: letter, octave: old_octave },
            }
        })
    }
}
