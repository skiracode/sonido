#include "gui/noteitem.h"
#include <QPainter>
#include <QBrush>
#include <QDebug>

NoteItem::NoteItem(qreal width)
{
    penWidth = 1.0 / 16.0;
    this->width = width;
}

QRectF NoteItem::boundingRect() const
{
    return QRectF(penWidth, penWidth, this->width - penWidth, 1.0 - penWidth);
}

void NoteItem::paint(QPainter *painter, const QStyleOptionGraphicsItem *, QWidget *)
{
    QBrush brush = this->palette().windowText();
    //qDebug() << brush;
    //painter->setBrush(brush);
    QRectF rect(0, 0, this->width, 1.0);
    painter->fillRect(rect, brush);
}
