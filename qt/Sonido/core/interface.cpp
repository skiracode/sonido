#include "interface.h"
#include <QDebug>

Interface::Interface(QObject *parent) : QObject(parent)
{

}

void Interface::print(QJSValue value)
{
    qInfo() << "From the print...";
    qInfo() << value.toString();
}

void Interface::render(QJSValue wave, QJSValue fileName)
{

}
