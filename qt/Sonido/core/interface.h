#ifndef INTERFACE_H
#define INTERFACE_H

#include <QObject>
#include <QJSValue>

class Interface : public QObject
{
    Q_OBJECT
public:
    explicit Interface(QObject *parent = nullptr);
public slots:
    void print(QJSValue value);
    void render(QJSValue wave, QJSValue fileName);
signals:

};

#endif // INTERFACE_H
