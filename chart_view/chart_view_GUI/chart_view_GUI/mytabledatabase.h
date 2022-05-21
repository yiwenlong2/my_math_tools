
#ifndef MYTABLEDATABASE_H
#define MYTABLEDATABASE_H

#include <QWidget>
#include <QTableWidget>

class mytabledatabase : public QTableWidget
{
    Q_OBJECT
public:
    explicit mytabledatabase(QWidget *parent = nullptr);
    bool event(QEvent *evnt);

signals:

};

#endif // MYTABLEDATABASE_H
