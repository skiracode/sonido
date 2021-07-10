pub fn greet() {
    println!("Hello from sonido.core")
}

pub struct SongDescription {
    instrumental_lines: Vec<InstrumentalLine>,
    //waves: Vec<Wave>,
}

impl SongDescription {
    pub fn new() -> SongDescription {
        let instrumental_lines = Vec::with_capacity(16);

        SongDescription { instrumental_lines }
    }
    pub fn add_line(&mut self, line: InstrumentalLine) {
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
    pub fn render(&self, sample_rate: f64, filename: &str) {
        let mut waves = Vec::with_capacity(self.instrumental_lines.len());
        let mut sum = 0.0;
        for line in &self.instrumental_lines {
            sum += line.weight;
            let end_time = line.find_end_time();
            let samples_count = (end_time * sample_rate) as usize;
            let mut wave = Wave::new(samples_count);
            for note in &line.notes {
                wave.write_note(line, *note, sample_rate, 1.0);
            }
            wave = line.post_proccess(wave, sample_rate);
            wave.normalize();
            waves.push(wave);
        }
        let mut length = 0;
        for i in 0..waves.len() {
            if waves[i].frames_count > length {
                length = waves[i].frames_count;
            }
        }
        let mut song = Wave::new(length);
        for i in 0..waves.len() {
            for c in 0..waves[i].channels.len() {
                for si in 0..waves[i].frames_count {
                    let mut sample = waves[i].channels[c][si] * self.instrumental_lines[i].weight;
                    if sum > 0.0 {
                        sample /= sum;
                    }
                    song.channels[c][si] += sample;
                }
            }
        }
        //song.equilibrate();
        song.normalize();
        song.save(sample_rate, filename)
    }
}

pub struct WeightedInstrument {
    instrument: Box<Instrument>,
    weight: f64,
}

impl WeightedInstrument {
    pub fn new(instrument: Box<Instrument>, weight: f64) -> WeightedInstrument {
        WeightedInstrument {instrument, weight}
    }
}

pub struct InstrumentalLine {
    instruments: Vec<WeightedInstrument>,
    notes: Vec<TimedNote>,
    post_processes: Vec<fn(Wave, f64) -> Wave>,
    weight: f64,
}

impl InstrumentalLine {
    pub fn new(weight: f64) -> InstrumentalLine {
        let instruments = Vec::with_capacity(16);
        let notes = Vec::with_capacity(64);
        let post_processes = Vec::with_capacity(16);
        InstrumentalLine {
            instruments,
            notes,
            post_processes,
            weight,
        }
    }
    pub fn add_instrument(&mut self, instrument: WeightedInstrument) {
        self.instruments.push(instrument);
    }
    pub fn add_note(&mut self, note: f64, time: f64, duration: f64) {
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
    pub fn add_post_procces(&mut self, post_procces_fn: fn(Wave, f64) -> Wave) {
        self.post_processes.push(post_procces_fn);
    }
}
type FrequencyModifier = fn(f64, f64) -> f64;
type AmplitudeModifier = fn(f64, f64, f64) -> f64;

pub struct DefualtInstrument {
    decay: f64,
    generators: Vec<fn(f64) -> f64>,
    weights: Vec<f64>,
    frequency_multipliers: Vec<FrequencyModifier>,
    frequency_adders: Vec<FrequencyModifier>,
    amplitude_multipliers: Vec<AmplitudeModifier>,
    amplitude_adders: Vec<AmplitudeModifier>,
}

impl DefualtInstrument {
    pub fn new(decay: f64) -> DefualtInstrument {
        let generators = Vec::with_capacity(16);
        let weights = Vec::with_capacity(16);
        let frequency_multipliers = Vec::with_capacity(16);
        let frequency_adders = Vec::with_capacity(16);
        let amplitude_multipliers = Vec::with_capacity(16);
        let amplitude_adders = Vec::with_capacity(16);
        DefualtInstrument {
            decay,
            generators,
            weights,
            frequency_multipliers,
            frequency_adders,
            amplitude_multipliers,
            amplitude_adders,
        }
    }
    pub fn add_generator(&mut self, generator: fn(f64) -> f64, weight: f64) {
        self.generators.push(generator);
        self.weights.push(weight);
    }
    pub fn add_frequency_multiplier(&mut self, frequency_multiplier: FrequencyModifier) {
        self.frequency_multipliers.push(frequency_multiplier);
    }
    pub fn add_frequency_adder(&mut self, frequency_adder: FrequencyModifier) {
        self.frequency_adders.push(frequency_adder);
    }
    pub fn add_amplitude_multiplier(&mut self, amplitude_multiplier: AmplitudeModifier) {
        self.amplitude_multipliers.push(amplitude_multiplier);
    }
    pub fn add_amplitude_adder(&mut self, amplitude_adder: AmplitudeModifier) {
        self.amplitude_adders.push(amplitude_adder);
    }
}

impl Instrument for DefualtInstrument {
    fn generate(&self, sample_index: usize, sample_rate: f64, note: TimedNote) -> f64 {
        let time = (sample_index as f64) / sample_rate;
        let mut frequency = note.to_frequency();
        for frequency_multiplier in &self.frequency_multipliers {
            frequency *= frequency_multiplier(time, frequency);
        }
        let freq_time = time * frequency;
        let t = (freq_time) % 1.0;
        let mut sample = 0.0;
        let mut sum = 0.0;
        for i in 0..self.generators.len() {
            let weight = self.weights[i];
            sample += generate(self.generators[i], t);
            sum += weight;
            sample *= weight;
        }
        sample /= sum;
        for amplitude_multiplier in &self.amplitude_multipliers {
            sample *= amplitude_multiplier(time, frequency, sample);
        }
        for amplitude_adder in &self.amplitude_adders {
            sample += amplitude_adder(time, frequency, sample);
        }
        sample /= (1.0 + self.decay * time);
        sample
    }
}

fn generate(generator: fn(f64) -> f64, mut time: f64) -> f64 {
    time *= 2.0;
    let mut sample = 0.0;
    let mut sum = 0.0;
    if time < 1.0 {
        sample -= generator(1.0 - time);
    } else {
        sample += generator(time - 1.0);
    }
    sample
}

pub trait Instrument {
    fn generate(&self, sample_index: usize, sample_rate: f64, note: TimedNote) -> f64;
}

#[derive(Copy, Clone)]
pub struct TimedNote {
    note: f64,
    time: f64,
    duration: f64,
}

impl TimedNote {
    pub fn new(note: f64, time: f64, duration: f64) -> TimedNote {
        TimedNote {
            note,
            time,
            duration,
        }
    }
    pub fn find_end_time(&self) -> f64 {
        self.time + self.duration
    }
    pub fn to_frequency(&self) -> f64 {
        440.0 * 2f64.powf(self.note / 12.0)
    }
}

#[derive(Clone)]
pub struct Wave {
    channels: [Vec<f64>; 2],
    frames_count: usize,
}

impl Wave {
    fn new(frames_count: usize) -> Wave {
        let mut left_channel = Vec::with_capacity(frames_count);
        let mut right_channel = Vec::with_capacity(frames_count);
        for _i in 0..frames_count {
            left_channel.push(0.0);
            right_channel.push(0.0);
        }
        Wave {
            channels: [left_channel, right_channel],
            frames_count,
        }
    }
    fn save(&self, sample_rate: f64, filename: &str) {
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: sample_rate as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(filename, spec).unwrap();
        for i in 0..self.frames_count {
            for c in 0..self.channels.len() {
                writer.write_sample(self.channels[c][i] as f32).unwrap();
            }
        }
    }
    /*
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
    */
    fn normalize(&mut self) {
        let mut max = 0.0;
        for c in 0..self.channels.len() {
            for i in 0..self.frames_count {
                if max < self.channels[c][i].abs() {
                    max = self.channels[c][i].abs()
                }
            }
        }

        if max > 0.0 {
            for c in 0..self.channels.len() {
                for i in 0..self.frames_count {
                    self.channels[c][i] /= max;
                }
            }
        }
    }
    /*
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
        let frames_count = clone.frames_count + starting_index;
        self.samples = Vec::with_capacity(frames_count);
        for i in 0..frames_count {
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
    */
    fn write_note(
        &mut self,
        line: &InstrumentalLine,
        note: TimedNote,
        sample_rate: f64,
        amplitude: f64,
    ) {
        let note_frames_count: usize = (note.duration * sample_rate) as usize;
        let starting_index = (note.time * sample_rate) as usize;
        let mut sum = 0.0;
        for c in 0..self.channels.len() {
            for i in 0..note_frames_count {
                for weighted_instrument in &line.instruments {
                    sum += weighted_instrument.weight;
                    let sample = weighted_instrument
                        .instrument
                        .generate(i, sample_rate, note);
                    self.channels[c][starting_index + i] +=
                        sample * amplitude * weighted_instrument.weight;
                }
            }
        }
        if sum > 0.0 {
            for c in 0..self.channels.len() {
                for i in 0..note_frames_count {
                    self.channels[c][starting_index + i] /= sum;
                }
            }
        }
    }
}
