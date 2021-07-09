#include "instrument.h"
#include <QFile>
#include <QIODevice>

Instrument::Instrument(QJSEngine &engine, QString fileName, QObject *parent) : QObject(parent)
{
    QFile file(fileName);
    file.open(QIODevice::ReadOnly);
    module = engine.importModule(fileName);
}

