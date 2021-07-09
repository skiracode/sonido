#include "gui/pianoroll.h"
#include "noteitem.h"

PianoRoll::PianoRoll(QWidget *parent) : QWidget(parent)
{
    octaves = 8;
    stackedLayout = new QStackedLayout();
    this->setLayout(stackedLayout);
    scene = new QGraphicsScene();
    scene->setSceneRect(0, 0, 16*4, 12*octaves);
    NoteItem *note = new NoteItem(1.0);
    scene->addItem(note);
    note->setPos(2.0, 1.0);
    //scene->addText("Hello, world!");
    view = new QGraphicsView(scene);
    stackedLayout->addWidget(view);
    stackedLayout->setCurrentWidget(view);
    view->centerOn(0, 0);
    view->scale(64, 64);
}
