#ifndef CONTROLCHARTVIEW_H
#define CONTROLCHARTVIEW_H

#include <QWidget>
#include <QDialog>
#include <QtCharts>
#include <QBrush>
#include <QSvgGenerator>
#include "file_io.h"

namespace Ui {
class ControlChartView;
}

class ControlChartView : public QDialog
{
    Q_OBJECT

public:

    explicit ControlChartView(QWidget *parent = nullptr);
    void addControlChartToLayout(control_chart cc,QString title);
    ~ControlChartView();

private:
    Ui::ControlChartView *ui;
};

#endif // CONTROLCHARTVIEW_H
