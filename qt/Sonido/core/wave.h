#ifndef WAVE_H
#define WAVE_H

#include <QObject>
#include <QVector>

class Wave : public QObject
{
    Q_OBJECT
private:
    QVector<qreal> samples;
public:
    explicit Wave(int size, QObject *parent = nullptr);
    void normalize();
    void echo(qreal sampleRate, qreal echoTime, qreal echoFactor);
    void save(qreal sampleRate, QString fileName);
signals:

};

#endif // WAVE_H
