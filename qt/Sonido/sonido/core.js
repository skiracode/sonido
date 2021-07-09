export function getSonido() {
    let sonido = {};

    sonido.normalize = (wave) => {
        let max = 0.0;
        for (let i = 0; i < wave.length; i++) {
            const abs = Math.abs(wave[i]);
            if (max < abs) {
                max = abs;
            }
        }
        if (max > 0.0) {
            for (let i = 0; i < wave.length; i++) {
                wave[i] /= max;
            }
        }
    };
    
    sonido.write = (timedNote, wave, line, sampleRate) => {
        let noteFramesCount = timedNote.duration * sampleRate;
        let startingIndex = timedNote.time * sampleRate;
        for (let i = 0; i < noteFramesCount; i++) {
            for (let j = 0; j < line.instruments.length; j++) {
                let instrument = line.instruments[j][0]
                let weight = line.instruments[j][1];
                let frame = instrument.generate(timedNote, i, sampleRate);
                wave[(startingIndex + i) * 2] += frame[0] * weight;
                wave[(startingIndex + i) * 2 + 1] += frame[0] * weight;
            }
        }
        let sum = line.sum();
        for (let i = 0; i < noteFramesCount; i++) {
            wave[(startingIndex + i) * 2] /= sum;
            wave[(startingIndex + i) * 2 + 1] /= sum;
        }
    };
    
    sonido.left = 0;
    
    sonido.right = 1;
    
    sonido.timedNote = (note, time, duration) => {
        let timedNote = { note, time, duration };
        timedNote.endTime = () => {
            return timedNote.time + timedNote.duration;
        };
        timedNote.frequency = () => {
            440.0 * Math.pow(2, timedNote.note / 12.0);
        };
        return timedNote;
    };
    
    sonido.line = (instruments, weight, notes) => {
        let line = { instruments, weight, notes };
        line.endTime = () => {
            let endTime = 0.0;
            for (let i = 0; i < line.notes.length; i++) {
                let lineEnd = line.notes[i].endTime();
                if (endTime < lineEnd) {
                    endTime = lineEnd;
                }
            }
            return endTime;
        };
        line.sum = () => {
            let sum = 0.0;
            for (let i = 0; i < line.instruments.length; i++) {
                sum += line.instruments[i][1];
            }
            return sum;
        };
        return line;
    };
    
    sonido.render = (wave, fileName) => {
        interface.render(wave, fileName);
    }
    
    sonido.wave = (framesCount) => {
        let samples = [];
        for (let i = 0; i < framesCount; i++) {
            samples.push(0.0);
            samples.push(0.0);
        }
        return samples;
    };
    
    sonido.song = (lines) => {
        let song = { lines };
        song.toWave = (sampleRate) => {
            const waves = [];
            for (let i = 0; i < song.lines.length; i++) {
                const line = song.lines[i];
                const endTime = line.endTime();
                const framesCount = endTime * sampleRate;
                const wave = sonido.wave(framesCount);
                for (let j = 0; j < line.notes.length; j++) {
                    sonido.write(line.notes[j], wave, line, sampleRate);
                }
                sonido.normalize(wave);
                waves.push(wave);
            }
            let sum = 0.0;
            let framesCount = 0.0;
            for (let i = 0; i < waves.length; i++) {
                sum += song.lines[i].weight;
                if (framesCount < waves[i].length) {
                    lengframesCountth = waves[i].length;
                }
            }
            framesCount /= 2;
            let songWave = sonido.wave(framesCount);
            for (let i = 0; i < waves.length; i++) {
                for (let j = 0; j < waves[i].length; j++) {
                    let sample = waves[i][j] * song.lines[i].weight;
                    if (sum > 0.0) {
                        sample /= sum;
                    }
                    songWave[j] += sample;
                }
            }
            return songWave;
        };
        return song;
    };
    
    sonido.generate = (generator, time) => {
        time *= 2.0;
        let leftSample, rightSample;
        if (time < 1.0) {
            leftSample = generator(1.0 - time, sonido.left);
            rightSample = generator(1.0 - time, sonido.right);
        } else {
            leftSample = generator(time - 1.0, sonido.left);
            rightSample = generator(time - 1.0, sonido.right);
        }
        return [leftSample, rightSample];
    };
    
    sonido.saw = (time) => {
        return time;
    };
    
    sonido.simpleInstrument = (generator) => {
        let instrument = {};
        instrument.generator = generator;
        instrument.generate = (timedNote, frameIndex, sampleRate) => {
            let time = frameIndex * sampleRate;
            let frequencyTime = time * timedNote.frequency();
            return sonido.generate(instrument.generator, frequencyTime % 1.0);
        };
        return instrument;
    };
    return sonido;
}
