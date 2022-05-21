#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QFileDialog>
#include <QRegExp>
#include <QtCharts>
#include <QBrush>
#include "controlchartview.h"

struct ChartToC {
    double ucl;
    double lcl;
    double bar;
    uint32_t len;
    double *data;
};


#pragma comment(lib,"chart_view_c_api.dll.lib")
//extern "C" ChartToC r_control_chart(bool inrows);
extern "C" void r_control_chart(bool inrows);
extern "C" void get_vec_from_rust_v2(double array[],uint32_t *len);
extern "C" void xbar_control_chart(bool inrows);
extern  "C" void calculate_control_chart_subgroup(bool inrows,uint32_t chart_type,uint32_t data_in_column_number,uint32_t split_n);
extern "C" void calculate_control_chart_individual(bool inrows,uint32_t chart_type, uint32_t data_in_column_number,uint32_t split_n,uint32_t mr_width, uint32_t mr_step);

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    void resizeTable(bool rs_row,bool addrm);
    ~MainWindow();


private:
    Ui::MainWindow *ui;
};



#endif // MAINWINDOW_H
