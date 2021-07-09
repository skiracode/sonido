import numpy as np
from scipy.io import wavfile
from abc import ABC, abstractmethod
from enum import Enum
import math


class Channel(Enum):
    LEFT = 0
    RIGHT = 1


class Instrument(ABC):

    @abstractmethod
    def generate(self, timed_note, frame_index, sample_rate):
        pass


def greet():
    print("Hello from sonido")


def normalize(wave):
    maximum = [0.0, 0.0]
    for si in range(wave.shape[0]):
        for ci in range(wave.shape[1]):
            absolute = abs(wave[si, ci])
            if maximum[ci] < absolute:
                maximum[ci] = absolute
    for ci in range(wave.shape[1]):
        if maximum[ci] > 0.0:
            for si in range(wave.shape[0]):
                wave[si, ci] /= maximum[ci]


def render(wave, file_name, sample_rate):
    wavfile.write(file_name, sample_rate, wave.astype(np.float32))


def new_wave(frames_count):
    return np.zeros((frames_count, 2), dtype=np.double)


def saw(time):
    return time


def generate(generator, time):
    time *= 2.0
    if time < 1.0:
        left_sample = generator(1.0 - time)
        right_sample = generator(1.0 - time)
    else:
        left_sample = generator(1.0 - time)
        right_sample = generator(1.0 - time)
    return np.array([left_sample, right_sample])


class WeightedInstrument:
    def __init__(self, instrument, weight):
        self.instrument = instrument
        self.weight = weight


class SimpleInstrument(Instrument):
    def __init__(self, generator):
        self.generator = generator

    def generate(self, timed_note, frame_index, sample_rate):
        time = float(frame_index) / sample_rate
        frequency_time = time * timed_note.frequency()
        return generate(self.generator, frequency_time % 1.0)


def write_note(timed_note, wave, line, sample_rate):
    frames_count = int(timed_note.duration * sample_rate)
    starting_index = int(timed_note.time * sample_rate)
    for frame_index in range(frames_count):
        for weighted_instrument in line.instruments:
            instrument = weighted_instrument.instrument
            weight = weighted_instrument.weight
            frame = instrument.generate(timed_note, frame_index, sample_rate)
            wave[starting_index + frame_index, 0] += frame[0] * weight
            wave[starting_index + frame_index, 1] += frame[1] * weight
    summation = line.summation()
    if summation > 0:
        for frame_index in range(frames_count):
            wave[starting_index + frame_index, 0] /= summation
            wave[starting_index + frame_index, 1] /= summation


class Song:
    def __init__(self, lines):
        self.lines = lines

    def to_wave(self, sample_rate):
        waves = []
        summation = 0.0
        for line in self.lines:
            summation += line.weight
            end_time = line.end_time()
            frames_count = int(end_time * sample_rate)
            wave = new_wave(frames_count)
            for timed_note in line.notes:
                write_note(timed_note, wave, line, sample_rate)
            normalize(wave)
            waves.append(wave)
        frames_count = 0
        for wave in waves:
            if frames_count < wave.shape[0]:
                frames_count = wave.shape[0]
        song_wave = new_wave(frames_count)
        line_index = 0
        for wave in waves:
            for ci in range(wave.shape[1]):
                for si in range(wave.shape[0]):
                    sample = wave[si, ci] * self.lines[line_index].weight
                    if summation > 0.0:
                        sample /= summation
                    song_wave[si, ci] += sample
        return song_wave


class InstrumentalLine:
    def __init__(self, instruments, weight, notes):
        self.instruments = instruments
        self.weight = weight
        self.notes = notes

    def end_time(self):
        end_time = 0.0
        for timed_note in self.notes:
            note_end = timed_note.end_time()
            if end_time < note_end:
                end_time = note_end
        return end_time

    def summation(self):
        summation = 0.0
        for weighted_instrument in self.instruments:
            summation += weighted_instrument.weight
        return summation


class TimedNote:
    def __init__(self, note, time, duration):
        self.note = note
        self.time = time
        self.duration = duration

    def end_time(self):
        return self.time + self.duration

    def frequency(self):
        return 440.0 * math.pow(2.0, self.note / 12.0)