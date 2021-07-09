#include "timednote.h"
#include <QtMath>

TimedNote::TimedNote(qreal note, qreal time, qreal duration, QObject *parent) : QObject(parent)
{
    m_note = note;
    m_duration = duration;
    m_time = time;
}

qreal TimedNote::toFrequency()
{
    return 440.0 * qPow(2.0, m_note / 12.0);
}

qreal TimedNote::getEndTime()
{
    return m_time + m_duration;
}
