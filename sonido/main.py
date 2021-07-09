import sonido

if __name__ == "__main__":
    sonido.greet()
    sample_rate = 44100
    the_wave = sonido.new_wave(sample_rate)
    sonido.normalize(the_wave)
    sonido.render(the_wave, "sonido.wav", sample_rate)
    base_note = 12.0 * -4.0
    notes = [sonido.TimedNote(base_note + 0.0, 0.0, 1.0), sonido.TimedNote(base_note + 5.0, 1.0, 1.0),
             sonido.TimedNote(base_note + 9.0, 2.0, 1.0),
             sonido.TimedNote(base_note + 12.0, 3.0, 1.0)]
    instrument = sonido.SimpleInstrument(sonido.saw)
    weighted = sonido.WeightedInstrument(instrument, 1.0)
    line = sonido.InstrumentalLine([weighted], 1.0, notes)
    song = sonido.Song([line])
    wave = song.to_wave(sample_rate)
    sonido.render(wave, "song.wav", sample_rate)
