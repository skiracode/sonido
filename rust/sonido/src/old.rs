#![allow(warnings)]
use std::any::Any;
use std::collections::LinkedList;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct TimedNote {
    note: f64,
    time: f64,
    duration: f64,
}
#[derive(Clone)]
struct Instrument {
    generators: Vec<fn(f64) -> f64>,
    weights: Vec<f64>,
    time_frequenciators: Vec<fn(f64) -> f64>,
    modifiers: Vec<fn(f64, f64) -> f64>,
    decay: f64,
}
#[derive(Clone)]
struct Wave {
    samples: Vec<f64>,
}

struct InstrumentalLine {
    instrument: Instrument,
    notes: Vec<TimedNote>,
    post_processes: Vec<fn(Wave, f64) -> Wave>,
    amplitude: f64,
}
struct SongDescription {
    instrumental_lines: Vec<InstrumentalLine>,
    //waves: Vec<Wave>,
}
impl InstrumentalLine {
    fn new(instrument: Instrument, amplitude: f64) -> InstrumentalLine {
        let notes = Vec::with_capacity(64);
        let post_processes = Vec::with_capacity(16);
        InstrumentalLine {
            instrument,
            notes,
            post_processes,
            amplitude,
        }
    }
    fn add_note(&mut self, note: f64, time: f64, duration: f64) {
        self.notes.push(TimedNote::new(note, time, duration))
    }
    fn find_end_time(&self) -> f64 {
        let mut end_time = 0.0;
        for note in &self.notes {
            let line_end = note.find_end_time();
            if line_end > end_time {
                end_time = line_end;
            }
        }
        end_time
    }
    fn post_proccess(&self, mut wave: Wave, sample_rate: f64) -> Wave {
        for post_procces_fn in &self.post_processes {
            wave = post_procces_fn(wave, sample_rate);
        }
        wave
    }
    fn add_post_procces(&mut self, post_procces_fn: fn(Wave, f64) -> Wave) {
        self.post_processes.push(post_procces_fn);
    }
}
impl SongDescription {
    fn new() -> SongDescription {
        let instrumental_lines = Vec::with_capacity(16);

        SongDescription { instrumental_lines }
    }
    fn add_line(&mut self, line: InstrumentalLine) {
        self.instrumental_lines.push(line);
    }
    fn find_end_time(&self) -> f64 {
        let mut end_time = 0.0;
        for line in &self.instrumental_lines {
            let line_end = line.find_end_time();
            if line_end > end_time {
                end_time = line_end;
            }
        }
        end_time
    }
    fn render(&self, sample_rate: f64, filename: &str) {
        let mut waves = Vec::with_capacity(self.instrumental_lines.len());
        for line in &self.instrumental_lines {
            let end_time = line.find_end_time();
            let samples_count = (end_time * sample_rate) as usize;
            let mut wave = Wave::new(samples_count);
            for note in &line.notes {
                wave.write_note(line.instrument.clone(), *note, sample_rate, 1.0);
            }
            wave = line.post_proccess(wave, sample_rate);
            wave.normalize();
            waves.push(wave);
        }
        let mut sum = 0.0;
        let mut length = 0;
        for i in 0..waves.len() {
            sum += self.instrumental_lines[i].amplitude;
            if waves[i].samples.len() > length {
                length = waves[i].samples.len();
            }
        }
        let mut song = Wave::new(length);
        for i in 0..waves.len() {
            for si in 0..waves[i].samples.len() {
                let mut sample = waves[i].samples[si] * self.instrumental_lines[i].amplitude;
                if sum > 0.0 {
                    sample /= sum;
                }
                song.samples[si] += sample;
            }
        }
        //song.equilibrate();
        song.normalize();
        song.save(sample_rate, filename)
    }
    fn render_old(&self, sample_rate: f64, filename: &str) {
        let end_time = self.find_end_time();
        let samples_count = (end_time * sample_rate) as usize;
        let mut wave = Wave::new(samples_count);
        for line in &self.instrumental_lines {
            for note in &line.notes {
                wave.write_note(line.instrument.clone(), *note, sample_rate, line.amplitude);
            }
        }
        wave.master(sample_rate);
        //wave.equilibrate();
        wave.normalize();
        wave.save(sample_rate, filename)
    }
}
impl Instrument {
    fn new(decay: f64) -> Instrument {
        let generators = Vec::with_capacity(16);
        let weights = Vec::with_capacity(16);
        let modifiers = Vec::with_capacity(16);
        let time_frequenciators = Vec::with_capacity(16);
        Instrument {
            generators,
            weights,
            modifiers,
            time_frequenciators,
            decay,
        }
    }
    fn modify(&self, mut sample: f64, time: f64, frequency: f64) -> f64 {
        for modifier in &self.modifiers {
            sample *= modifier(time, frequency);
        }
        sample
    }
    fn get_sample(&self, sample_index: usize, sample_rate: f64, note: TimedNote) -> f64 {
        let time = (sample_index as f64) / sample_rate;
        let mut frequency = note.to_frequency();
        for frequenciator in &self.time_frequenciators {
            frequency *= frequenciator(time);
        }
        let freq_time = time * frequency;
        let t = (freq_time) % 1.0;
        let mut sample = self.generate(t);
        sample = self.modify(sample, time, frequency);
        sample /= (1.0 + self.decay * time * 1.0 / 1.0);
        sample
    }
    fn generate(&self, mut time: f64) -> f64 {
        time *= 2.0;
        let mut sample = 0.0;
        let mut sum = 0.0;
        for i in 0..self.generators.len() {
            let weight = self.weights[i];
            sum += weight;
            if time < 1.0 {
                sample -= self.generators[i](1.0 - time);
            } else {
                sample += self.generators[i](time - 1.0);
            }
            sample *= weight;
        }
        sample /= sum;
        sample
    }
    fn add_generator(&mut self, generator: fn(f64) -> f64, weight: f64) {
        self.generators.push(generator);
        self.weights.push(weight);
    }
    fn add_modifier(&mut self, modifier: fn(f64, f64) -> f64) {
        self.modifiers.push(modifier);
    }
    fn add_time_frequenciator(&mut self, frequenciator: fn(f64) -> f64) {
        self.time_frequenciators.push(frequenciator);
    }
}
impl TimedNote {
    fn new(note: f64, time: f64, duration: f64) -> TimedNote {
        TimedNote {
            note,
            time,
            duration,
        }
    }
    fn find_end_time(&self) -> f64 {
        self.time + self.duration
    }
    fn to_frequency(&self) -> f64 {
        512.0 * 2f64.powf(self.note / 12.0)
    }
}
impl Wave {
    fn new(samples_count: usize) -> Wave {
        let mut samples = Vec::with_capacity(samples_count);
        for _i in 0..samples_count {
            samples.push(0.0);
        }
        Wave { samples }
    }
    fn save(&self, sample_rate: f64, filename: &str) {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: sample_rate as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for sample in &self.samples {
            writer.write_sample(*sample as f32).unwrap();
            writer.write_sample(*sample as f32).unwrap();
        }
    }
    fn equilibrate(&mut self) {
        let mut max = 0.0;
        for sample in &self.samples {
            if *sample > max {
                max = *sample;
            }
        }
        let mut min = 0.0;
        for sample in &self.samples {
            if *sample < min {
                min = *sample;
            }
        }
        let difference = max + min;
        if difference != 0.0 {
            let half = difference / 2.0;
            for i in 0..self.samples.len() {
                self.samples[i] -= half;
            }
        }
    }
    fn normalize(&mut self) {
        let mut max = 0.0;
        for sample in &self.samples {
            if sample.abs() > max {
                max = sample.abs();
            }
        }
        if max > 0.0 {
            for i in 0..self.samples.len() {
                self.samples[i] /= max;
            }
        }
    }
    fn reverb(&mut self, sample_rate: f64, mut echo_time: f64, mut echo_factor: f64, depth: usize) {
        for i in 0..depth {
            self.echo(sample_rate, echo_time, echo_factor);
            echo_time *= 4.0 / 2.0;
            echo_factor *= 1.0 / 2.0;
        }
    }
    fn master(&mut self, sample_rate: f64) {
        //self.reverb(sample_rate, 1.0 / 8.0, 1.0 / 2.0, 4);
    }
    fn echo(&mut self, sample_rate: f64, echo_time: f64, echo_factor: f64) {
        let clone = self.clone();
        let starting_index = (echo_time * sample_rate) as usize;
        let samples_count = clone.samples.len() + starting_index;
        self.samples = Vec::with_capacity(samples_count);
        for i in 0..samples_count {
            if i < clone.samples.len() {
                self.samples.push(clone.samples[i]);
            } else {
                self.samples.push(0.0);
            }
        }
        for i in 0..clone.samples.len() {
            let wave_index = starting_index + i;
            self.samples[wave_index] += clone.samples[i] * echo_factor;
        }
    }
    fn write_note(
        &mut self,
        instrument: Instrument,
        note: TimedNote,
        sample_rate: f64,
        amplitude: f64,
    ) {
        let note_samples_count: usize = (note.duration * sample_rate) as usize;
        let starting_index = (note.time * sample_rate) as usize;
        for index in 0..note_samples_count {
            let sample = instrument.get_sample(index, sample_rate, note);
            self.samples[starting_index + index] += sample * amplitude;
        }
    }
}
fn post_proccess_0x1(mut wave: Wave, sample_rate: f64) -> Wave {
    wave.reverb(sample_rate, 1.0 / 32.0, 1.0 / 2.0, 4);
    wave
}
fn post_proccess_0x3(mut wave: Wave, sample_rate: f64) -> Wave {
    wave.reverb(sample_rate, 1.0 / 16.0, 1.0 / 4.0, 4);
    wave.reverb(sample_rate, 1.0 / 8.0, 1.0 / 4.0, 4);
    wave
}
fn post_proccess_0x4(mut wave: Wave, sample_rate: f64) -> Wave {
    wave.reverb(sample_rate, 1.0 / 16.0, 1.0 / 2.0, 4);
    wave
}
fn post_proccess_0x2(mut wave: Wave, sample_rate: f64) -> Wave {
    //wave.reverb(sample_rate, 1.0 / 4.0, 1.0 / 2.0, 4);
    wave.reverb(sample_rate, 1.0 / 8.0, 1.0 / 2.0, 4);
    wave.reverb(sample_rate, 1.0 / 16.0, 1.0 / 2.0, 4);
    wave
}
fn frequenciator_0x1(time: f64) -> f64 {
    1.0 / (time * 64.0 + 1.0)
}
fn frequenciator_0x2(time: f64) -> f64 {
    1.0 / (time * 256.0 / 1.0 + 1.0)
}
fn frequenciator_0x3(time: f64) -> f64 {
    (sine((time * 8.0) % 1.0) + 1.0) / 2.0
}
fn frequenciator_0x4(time: f64) -> f64 {
    (time * 1.0 / 64.0) + 1.0
}
fn modifier_0x1(time: f64, frequency: f64) -> f64 {
    sine((frequency * time * 1.0 / 16.0) % 1.0)
}
fn modifier_0x2(time: f64, frequency: f64) -> f64 {
    sine((frequency * time * 1.0 / 4.0) % 1.0)
}
fn modifier_0x3(time: f64, frequency: f64) -> f64 {
    sine((time * 16.0 / 1.0) % 1.0)
}
fn sine(time: f64) -> f64 {
    (time * PI).sin()
}
fn square(time: f64) -> f64 {
    1.0
}
fn saw(time: f64) -> f64 {
    time
}
fn ellipsis(mut time: f64) -> f64 {
    return 1.0 - (1.0 - time * time).sqrt();
}

fn quad(mut time: f64) -> f64 {
    time * time
}

fn triangle(time: f64) -> f64 {
    if time < 0.5 {
        return time * 2.0;
    }
    return 2.0 - 2.0 * time;
}
fn old() {
    let sample_rate = 41000.0;
    let mut instrument_0x1 = Instrument::new(16.0 / 1.0);
    let mut instrument_0x2 = Instrument::new(16.0 / 1.0);
    let mut instrument_0x3 = Instrument::new(64.0 / 1.0);
    let mut instrument_0x4 = Instrument::new(16.0 / 1.0);

    instrument_0x1.add_generator(sine, 1.0);
    instrument_0x1.add_generator(square, 4.0);
    //instrument_0x1.add_time_frequenciator(frequenciator_0x2);
    instrument_0x1.add_time_frequenciator(frequenciator_0x2);

    instrument_0x2.add_generator(saw, 4.0);
    instrument_0x2.add_generator(square, 16.0);
    instrument_0x2.add_generator(ellipsis, 1.0);
    instrument_0x2.add_time_frequenciator(frequenciator_0x4);

    instrument_0x3.add_generator(sine, 4.0);
    instrument_0x3.add_generator(ellipsis, 1.0);
    instrument_0x3.add_time_frequenciator(frequenciator_0x1);

    instrument_0x4.add_generator(sine, 1.0);
    instrument_0x4.add_generator(quad, 1.0);

    let mut line_0x1 = InstrumentalLine::new(instrument_0x1, 2.0 / 2.0);
    let mut line_0x2 = InstrumentalLine::new(instrument_0x2, 1.0 / 2.0);
    let mut line_0x3 = InstrumentalLine::new(instrument_0x3, 4.0 / 2.0);
    let mut line_0x4 = InstrumentalLine::new(instrument_0x4, 1.0 / 2.0);
    //line_0x1.add_post_procces(post_proccess_0x3);
    line_0x1.add_post_procces(post_proccess_0x3);
    line_0x2.add_post_procces(post_proccess_0x4);
    line_0x3.add_post_procces(post_proccess_0x1);
    line_0x4.add_post_procces(post_proccess_0x2);
    let base_note = 12.0 * 4.0;
    let base_duration = 1.0 / 2.0 / 2.0;
    let mut time = 8.0 / 2.0;

    for j in 0..6 {
        for i in 0..2 {
            time += base_duration;
            line_0x1.add_note(base_note + 0.0, time, base_duration / 1.0);
            time += base_duration;
            time += base_duration;
            time += base_duration;

        }
    }
    let base_duration = 1.0 / 4.0 / 2.0;
    let base_note = 12.0 * 0.0;
    time = 24.0 / 2.0;
    for j in 0..2 {
        for i in 0..4 {
            line_0x4.add_note(base_note + 12.0, time, base_duration);
            time += base_duration;
            line_0x4.add_note(base_note + 7.0, time, base_duration);
            time += base_duration;
            line_0x4.add_note(base_note + 3.0, time, base_duration);
            time += base_duration;
            line_0x4.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
        }
    }
    let base_duration = 1.0 / 2.0 / 2.0;
    let base_note = 12.0 * -3.0;
    time = 16.0 / 2.0;
    for j in 0..2 {
        for i in 0..4 {
            //line_0x2.add_note(base_note + 12.0, time, base_duration);
            //time += base_duration;
            line_0x2.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
            line_0x2.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
            line_0x2.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
            line_0x2.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
        }
    }
    let base_duration = 1.0 / 1.0 / 2.0;
    let base_note = 12.0 * -0.0;
    time = 0.0;
    for j in 0..8 {
        for i in 0..4 {
            /*
            line_0x3.add_note(base_note + 7.0, time, base_duration);
            time += base_duration;
            line_0x3.add_note(base_note + 3.0, time, base_duration);
            time += base_duration;
            */
            //time += base_duration;
            line_0x3.add_note(base_note + 0.0, time, base_duration);
            time += base_duration;
        }
    }

    let mut song_description = SongDescription::new();
    song_description.add_line(line_0x1);
    song_description.add_line(line_0x2);
    song_description.add_line(line_0x3);
    song_description.add_line(line_0x4);
    song_description.render(sample_rate, "song.wav")
}
