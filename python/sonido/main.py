import sys
import sonido.core
import sonido.gui


def core():
    sample_rate = 44100
    base_note = 12.0 * -4.0
    notes = [sonido.core.TimedNote(base_note + 0.0, 0.0, 1.0), sonido.core.TimedNote(base_note + 5.0, 1.0, 1.0),
             sonido.core.TimedNote(base_note + 9.0, 2.0, 1.0),
             sonido.core.TimedNote(base_note + 12.0, 3.0, 1.0)]
    instrument = sonido.core.SimpleInstrument(sonido.core.saw)
    weighted = sonido.core.WeightedInstrument(instrument, 1.0)
    line = sonido.core.InstrumentalLine([weighted], 1.0, notes)
    song = sonido.core.Song([line])
    wave = song.to_wave(sample_rate)
    sonido.core.render(wave, "song.wav", sample_rate)


if __name__ == "__main__":
    sonido.core.greet()
    sonido.gui.test()
