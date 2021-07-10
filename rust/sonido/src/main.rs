#![allow(warnings)]
mod sonido;
mod old;
use crate::sonido::{core::{TimedNote, DefualtInstrument, WeightedInstrument, InstrumentalLine, SongDescription}, generators};

fn main() {
    sonido::core::greet();
    let mut instrument = DefualtInstrument::new(0.0);
    instrument.add_generator(generators::saw, 1.0);
    let weighted = WeightedInstrument::new(Box::new(instrument), 1.0);
    let mut line = InstrumentalLine::new(1.0);
    line.add_instrument(weighted);
    line.add_note(0.0, 0.0, 1.0);
    let mut song = SongDescription::new();
    song.add_line(line);
    song.render(44100.0, "song.wav");
}