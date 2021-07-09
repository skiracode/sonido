#ifndef ENGINE_H
#define ENGINE_H

#include <QObject>
#include <QJSValue>
#include <QJSEngine>
#include "core/interface.h"

class Engine : public QJSEngine
{
    Q_OBJECT
private:
    QJSValue sonido;
    Interface *interface;
public:
    explicit Engine(QObject *parent = nullptr);

signals:

};

#endif // ENGINE_H
