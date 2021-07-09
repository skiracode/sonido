#ifndef NOTEITEM_H
#define NOTEITEM_H

#include <QGraphicsWidget>

class NoteItem : public QGraphicsWidget
{
private:
    qreal width;
    qreal penWidth;
public:
    NoteItem(qreal width);
    QRectF boundingRect() const override;
    void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget) override;
};

#endif // NOTEITEM_H
