
#include "file_io.h"



file_io::file_io()
{

}

void file_io::CSVFileRead(QString *filename){
    QFile file(*filename);
    if(!file.open(QIODevice::ReadOnly | QIODevice::Text))
    {
        qDebug()<<"Open CSV file failed!";
        return;
    }
    QStringList list;
    list.clear();
    QTextStream in(&file);
    for(int i=0; !in.atEnd();i++)
    {
        QString fileLine=in.readLine();
        list=fileLine.split(",",Qt::SkipEmptyParts);
        for(int j=0;j<list.length();j++)
        {
            qDebug()<<list.at(j);
        }
    }
    file.close();
}
//将RUST Lib生成的CSV进行读取;
//读取csv文件,并将其转换为QStringList;
QStringList file_io::ReadCSVtoQSL(const QString *filename){
    QFile file(*filename);
    QStringList list;
    list.clear();
    if(!file.open(QIODevice::ReadOnly | QIODevice::Text))
    {
        return list;

    }

    QTextStream in(&file);
    for(int i=0; !in.atEnd();i++)
    {
        QString fileLine=in.readLine();
        list<<fileLine;
    }
    file.close();
    return list;
}
//将CSV文件导入到表格中
void file_io::ImportCSVtoTable(QTableWidget *tablewidget,QString *filename){


    tablewidget->clear();

    QStringList list_from_csv=ReadCSVtoQSL(filename);
    if(list_from_csv.isEmpty()){
        //qDebug()<<"List is empty";
        return;
    }
    int real_rows_count=list_from_csv.length();
    int real_columns_count=list_from_csv.at(0).split(',').length();
    tablewidget->setRowCount(real_rows_count);
    tablewidget->setColumnCount(real_columns_count);


    for (int i=0;i<real_rows_count;i++){

        QStringList *temp_list=new QStringList(list_from_csv.at(i).split(','));
        for(int j=0;j<real_columns_count;j++){
            QTableWidgetItem *item=new QTableWidgetItem();
            item->setText(temp_list->at(j));
            tablewidget->setItem(i,j,item);
            //qDebug()<<item->text();
        }
    }
}



//CSV write 模板
void file_io::CSVFileWrite(QString *filename){
    QFile file(*filename);
    QStringList lines;
    lines<<"class, name, age, score\n"<<"3,xiaodong,12,87\n"<<"3,lihong,12,95\n";

    if (file.open(QIODevice::WriteOnly))
    {
        for(int i=0;i<lines.size();i++)
        {
            file.write(lines[i].toStdString().c_str());
        }

        file.close();

    }

}
//将表格写入到CSV,文件保存在database,文件名为tableInput;
bool file_io::TableWriteToCSV(QTableWidget *tablewidget){

    int columns_count=tablewidget->columnCount();
    int rows_count=tablewidget->rowCount();
    QString comma=",";
    QString LF="\n";
    //qDebug()<<rows_count;
    //qDebug()<<columns_count;


    QString file_database="./database/tableInput.csv";
    QFile file(file_database);

    if (file.exists()) {
        file.remove();
    }

    if (TableItemCheck(tablewidget)){
       if (file.open(QIODevice::WriteOnly))
        {


            //qDebug()<<"The table is"<<real_rows_count<<"*"<<real_columns_count;
            for (int i=0;i<rows_count;i++){
                for (int j=0;j<columns_count;j++){


                    file.write(tablewidget->item(i,j)->text().toStdString().c_str());
                    //qDebug()<<"Writed the text:"<<tablewidget->item(i,j)->text();
                    if(j==columns_count-1){

                    }
                    else {
                        file.write(comma.toStdString().c_str());
                    }
                }
                file.write(LF.toStdString().c_str());
            }
        }
        file.close();
        return true;
    }
    else{
        return false;
        //qDebug()<<"TableItemCheck=False";
    }


}
//检查表格内数据是否有空值;
bool file_io::TableItemCheck(QTableWidget *tablewidget){
    //qDebug()<<"Enter the TableItemCheck";

    int columns_count=tablewidget->columnCount();
    int rows_count=tablewidget->rowCount();

    //qDebug()<<"The orgin count of rows,column is: "<<rows_count<<","<<columns_count;

    int real_columns_count=columns_count;
    int real_rows_count=rows_count;

    QString emtyp_index="第";
     //qDebug()<<"The table real size is"<<real_rows_count<<"*"<<real_columns_count;

     for (int i=0;i<real_rows_count;i++){
         for (int j=0;j<real_columns_count;j++){
             if (tablewidget->item(i,j)==NULL)
             {
                 //qDebug()<<i<<","<<j<<"is NULL";
                 QString i_string=QString::number(i+1);
                 QString j_string=QString::number(j+1);
                 emtyp_index.append(i_string);
                 emtyp_index.append("行,");
                 emtyp_index.append(j_string);
                 emtyp_index.append("列,是空值");

                 QMessageBox::critical(tablewidget,"空值警告",emtyp_index);
                 return false;
             }
             else if (tablewidget->item(i,j)->text()==""){
                 //qDebug()<<i<<","<<j<<"is Empty";
                 QString i_string=QString::number(i+1);
                 QString j_string=QString::number(j+1);
                 emtyp_index.append(i_string);
                 emtyp_index.append("行,");
                 emtyp_index.append(j_string);
                 emtyp_index.append("列,是空值");

                 QMessageBox::critical(tablewidget,"空值警告",emtyp_index);
                 return false;
             }
         }
     }

     return true;
}



control_chart::control_chart() {

}
//从
control_chart control_chart::from_qsl(QStringList qsl){

    QStringList qsl_0=qsl.at(0).split(",",Qt::SkipEmptyParts);
    QStringList qsl_1=qsl.at(1).split(",",Qt::SkipEmptyParts);
/*    for (int i=0;i<qsl_0.length();i++){
        qDebug()<<qsl_0.at(i);
    }

*/
    double *data=new double[qsl_1.length()];
    for (int i=0;i<qsl_1.length();i++){
       // qDebug()<<qsl_1.at(i);
        data[i]=qsl_1.at(i).toDouble();
    }

    control_chart cc=control_chart();
    cc.ucl=qsl_0.at(0).toDouble();
    cc.lcl=qsl_0.at(1).toDouble();
    cc.ml=qsl_0.at(2).toDouble();
    cc.length=qsl_1.length();
    cc.data=data;


    return cc;

}



