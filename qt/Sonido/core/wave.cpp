#include "wave.h"
#include <QtMath>
#include <sndfile.h>

Wave::Wave(int size, QObject *parent) : QObject(parent)
{
    samples = QVector<qreal>(size, 0.0);
}

void Wave::normalize()
{
    qreal max = 0.0;
    for (int i = 0; i < samples.size(); i++) {
        double abs = qAbs(samples[i]);
        if (max < abs) {
            max = abs;
        }
    }
    if (max > 0.0) {
        for (int i = 0; i < samples.size(); i++) {
            samples[i] /= max;
        }
    }
}

void Wave::echo(qreal sampleRate, qreal echoTime, qreal echoFactor)
{
    int startingIndex = echoTime * sampleRate;
    int samplesCount = samples.size() + startingIndex;
    QVector<qreal> moved = QVector<qreal>(std::move(samples));
    samples = QVector<qreal>(samplesCount, 0.0);
    for (int i = 0; i < moved.size(); i++) {
        samples[i] = moved[i];
    }
    for (int i = 0; i < moved.size(); i++) {
        samples[i + startingIndex] += moved[i] * echoFactor;
    }
}

void Wave::save(qreal sampleRate, QString fileName)
{
    SF_INFO sfinfo;
    sfinfo.channels = 2;
    sfinfo.samplerate = sampleRate;
    sfinfo.format = SF_FORMAT_WAV | SF_FORMAT_DOUBLE;
    SNDFILE *file = sf_open(fileName.toLocal8Bit().data(), SFM_WRITE, &sfinfo);
    for (int i = 0; i < samples.size(); i++) {
        double sampleArray[2];
        sampleArray[0] = sampleArray[1] = samples[i];
        sf_writef_double(file, sampleArray, 1);
    }
    sf_close(file);
}

