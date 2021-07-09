export function main(args) {
    let notes = [];
    notes.push(sonido.timedNote(0.0, 0.0, 1.0));
    notes.push(sonido.timedNote(5.0, 1.0, 1.0));
    notes.push(sonido.timedNote(9.0, 2.0, 1.0));
    notes.push(sonido.timedNote(12.0, 3.0, 1.0));

    let instrument = sonido.simpleInstrument(sonido.saw);
    let line = sonido.line([[instrument, 1.0]], 1.0, notes);
    let song = sonido.song([[line, 1.0]]);
    let wave = song.toWave(44100);
    sonido.render(wave, "song.wav")
}