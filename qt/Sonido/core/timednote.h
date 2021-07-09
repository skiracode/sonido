#ifndef TIMEDNOTE_H
#define TIMEDNOTE_H

#include <QObject>

class TimedNote : public QObject
{
    Q_OBJECT
    Q_PROPERTY(qreal note MEMBER m_note)
    Q_PROPERTY(qreal time MEMBER m_time)
    Q_PROPERTY(qreal duration MEMBER m_duration)
    Q_PROPERTY(qreal frequency READ toFrequency)
    Q_PROPERTY(qreal endTime READ getEndTime)
private:
    qreal m_note;
    qreal m_time;
    qreal m_duration;
public:
    explicit TimedNote(qreal note, qreal time, qreal duration, QObject *parent = nullptr);
    qreal toFrequency();
    qreal getEndTime();
signals:

};

#endif // TIMEDNOTE_H
