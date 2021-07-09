#include "engine.h"
#include <QDebug>
#include <QFile>
#include <QTextStream>

Engine::Engine(QObject *parent) : QJSEngine(parent)
{
    QString sonidoFile = ":/sonido/core.js";
    interface = new Interface(this);
    QJSValue interfaceValue = newQObject(interface);
    globalObject().setProperty("interface", interfaceValue);
    sonido = importModule(sonidoFile);
    if (sonido.isError()) {
        qInfo() << "Error ->" << sonido.toString();
    }
}
