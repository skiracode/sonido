#ifndef INSTRUMENT_H
#define INSTRUMENT_H

#include <QObject>
#include <QJSValue>
#include <QJSEngine>

class Instrument : public QObject
{
    Q_OBJECT
private:
    QJSValue module;
public:
    explicit Instrument(QJSEngine &engine, QString fileName, QObject *parent = nullptr);
signals:

};

#endif // INSTRUMENT_H
