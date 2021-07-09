#ifndef INSTRUMENTALLINE_H
#define INSTRUMENTALLINE_H

#include <QObject>

class InstrumentalLine : public QObject
{
    Q_OBJECT
public:
    explicit InstrumentalLine(QObject *parent = nullptr);

signals:

};

#endif // INSTRUMENTALLINE_H
