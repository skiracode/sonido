#ifndef PIANOROLL_H
#define PIANOROLL_H

#include <QWidget>
#include <QGraphicsView>
#include <QGraphicsScene>
#include <QStackedLayout>

class PianoRoll : public QWidget
{
public:
    PianoRoll(QWidget *parent = nullptr);
private:
    quint32 octaves;
    QStackedLayout *stackedLayout;
    QGraphicsView *view;
    QGraphicsScene *scene;
};

#endif // PIANOROLL_H
