
#ifndef FILE_IO_H
#define FILE_IO_H

#include <QObject>
#include <QFile>
#include <QDebug>
#include <QTableWidget>
#include <QMessageBox>


class file_io
{
public:
    file_io();

    QString *filename=new QString;
    static void CSVFileRead(QString *filename);
    static void CSVFileWrite(QString *filename);
    static bool TableItemCheck(QTableWidget *tablewidget);
    static bool TableWriteToCSV(QTableWidget *tablewidget);
    static QStringList ReadCSVtoQSL(const QString *filename);
    static void ImportCSVtoTable(QTableWidget *tablewidget,QString *filename);


};

class control_chart
{
public:
    control_chart();
    double ucl;
    double lcl;
    double ml;
    uint32_t length;
    double *data;
    static control_chart from_qsl(QStringList qsl);
};

#endif // FILE_IO_H
