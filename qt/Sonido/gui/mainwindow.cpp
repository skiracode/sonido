#include "gui/mainwindow.h"
#include <QScreen>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    resize(screen()->geometry().size() * 3.0/4.0);
    move(screen()->geometry().center() - frameGeometry().center());
    pianoRoll = new PianoRoll();
    this->setCentralWidget(pianoRoll);
}

MainWindow::~MainWindow()
{

}

