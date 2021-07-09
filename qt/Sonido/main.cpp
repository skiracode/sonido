#include "gui/mainwindow.h"
#include "core/engine.h"

#include <QApplication>
#include <QDebug>


int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    qInfo() << "From the outside...";
    Engine engine;
    return a.exec();
}
