#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "QDebug"
#include "file_io.h"
static uint32_t type_RControlChart=1;
static uint32_t type_XbarControlChart=2;
static uint32_t type_SControlChart=3;
static uint32_t type_IControlChart=4;
static uint32_t type_MRControlChart=5;




MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);


    QString filename="./testdata/csv_qcharttestdata.csv";
    QString filename_write="./testdata/csv_testdata_write.csv";
    QString filename_rc="./testdata/子组_R控制图_凸轮轴长度.csv";
    QString filename_rust_output="./output/control_chart.csv";
    //将表格写入到CSV文件
    connect(ui->pushButton,&QPushButton::clicked,[=](){

        file_io::TableWriteToCSV(ui->tableWidget);
        QMessageBox::information(this,"数据已保存","表格数据已保存在/database/tableInput.csv; \n请重命名或移走,以免被覆盖!");
    });
    //按钮"导入CSV"
    //导入CSV文件
    connect(ui->pushButton_2,&QPushButton::clicked,[=](){
        QString filename=QFileDialog::getOpenFileName(this, tr("打开文件"),"./",tr("CSV file (*.csv)"));

        file_io::ImportCSVtoTable(ui->tableWidget,&filename);
    });  
    //按钮"设定"
    //生成/扩展表格, 用户输入行和列的数量;
    connect(ui->pushButton_3,&QPushButton::clicked,[=](){
        int rows=ui->lineEdit->text().toInt();
        int column=ui->lineEdit_2->text().toInt();

        //在设定表格大小前, 先确定表格是否已经被编辑过; 如果被编辑过, 要提醒用户设定表格大小会清除超出表格的数据;
        //在tablewidget的(0,0)||(0,1)||(1,0) 有元素, 则认为表格被编辑过,有可能有数据;
       if (ui->tableWidget->item(0,0)!=NULL ||ui->tableWidget->item(0,1) !=NULL || ui->tableWidget->item(1,0)!=NULL){
           qDebug()<<"(0,0)||(0,1)||(1,0) is not NULL";
           //如果设定行数或者列数,小于原来的值, 则提醒用户部分数据可能会被清除;
           if(ui->tableWidget->rowCount()>=rows || ui->tableWidget->columnCount()>=column){
               QMessageBox::StandardButton answer=QMessageBox::question(this,"数据清除警告!","该操作可能会清除已有数据,是否继续",(QMessageBox::Yes | QMessageBox::No), QMessageBox::Yes);
               if(answer==QMessageBox::Yes){
                   ui->tableWidget->setRowCount(rows);
                   ui->tableWidget->setColumnCount(column);
               }
               else{
               }
           }
       }
       else{
           qDebug()<<"(0,0) is NULL";
           ui->tableWidget->setRowCount(rows);
           ui->tableWidget->setColumnCount(column);
       }
    });
    //按钮"增加,删除行和列"
    //表格的增删;
    connect(ui->pushButton_4,&QPushButton::clicked,[=](){
        resizeTable(1,1);
    });
    connect(ui->pushButton_5,&QPushButton::clicked,[=](){
        resizeTable(0,1);
    });
    connect(ui->pushButton_7,&QPushButton::clicked,[=](){
        resizeTable(1,0);
    });
    connect(ui->pushButton_8,&QPushButton::clicked,[=](){
        resizeTable(0,0);
    });


    //按钮"生成", 将control chart 数据传入congtrol chart view 类的函数addControlCharttoLayout
    //并展示图片
    connect(ui->pushButton_6,&QPushButton::clicked,[=](){
        //将表格中的数据写入CSV文件,以便RUST Lib调用;
        if (file_io::TableWriteToCSV(ui->tableWidget))
        {
            //获取设置的参数:
        uint32_t column=1;
        uint32_t split_n=5;

        if (ui->radioButton->isChecked()){
            QString column_string=ui->lineEdit_3->text();
            QString split_string=ui->lineEdit_4->text();
            column=column_string.toUInt();
            split_n=split_string.toUInt();
        }
        else{
            return;
        }
        //图表的标题;
        QString rcc_name=QString("R Control Chart");
        QString xbarc_name=QString("Xbar Control Chart");
        QString scc_name=QString("S Control Chart");

        //生成QDailog 展示图表使用;
        ControlChartView *ccv=new ControlChartView(this);

        if (ui->checkBox->isChecked()){
            calculate_control_chart_subgroup(true,type_RControlChart,column,split_n);
            QStringList cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
            control_chart cc=control_chart::from_qsl(cc_list);
            ccv->addControlChartToLayout(cc,rcc_name);
        }

        if (ui->checkBox_2->isChecked()){
            calculate_control_chart_subgroup(true,type_XbarControlChart,column,split_n);
            QStringList xbar_cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
            control_chart xbar_cc=control_chart::from_qsl(xbar_cc_list);
            ccv->addControlChartToLayout(xbar_cc,xbarc_name);
        }
        if (ui->checkBox_3->isChecked()){
            calculate_control_chart_subgroup(true,type_SControlChart,column,split_n);
            QStringList s_cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
            control_chart s_cc=control_chart::from_qsl(s_cc_list);
            ccv->addControlChartToLayout(s_cc,scc_name);
        }


        //ccv->setModal(true);
        ccv->show();
        }
    });

    connect(ui->pushButton_9,&QPushButton::clicked,[=](){
        //将表格中的数据写入CSV文件,以便RUST Lib调用;
        if (file_io::TableWriteToCSV(ui->tableWidget))
        {
            //获取设置的参数:
        uint32_t column=1;
        uint32_t split_n=0;

        if (ui->radioButton_3->isChecked()){
            QString column_string=ui->lineEdit_8->text();
            column=column_string.toUInt();

        }
        else{
            return;
        }
        //图表的标题;
        QString icc_name=QString("I Control Chart");
        QString mrcc_name=QString("MR Control Chart");
        //QString scc_name=QString("S Control Chart");

        //生成QDailog 展示图表使用;
        ControlChartView *ccv=new ControlChartView(this);

        if (ui->checkBox_4->isChecked()){
            calculate_control_chart_individual(true,type_IControlChart,column,split_n,2,1);
            QStringList cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
            control_chart cc=control_chart::from_qsl(cc_list);
            ccv->addControlChartToLayout(cc,icc_name);
        }

        if (ui->checkBox_5->isChecked()){
            calculate_control_chart_individual(true,type_IControlChart,column,split_n,2,1);
            QStringList cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
            control_chart cc=control_chart::from_qsl(cc_list);
            ccv->addControlChartToLayout(cc,mrcc_name);
        }
//        if (ui->checkBox_3->isChecked()){
//            calculate_control_chart_subgroup(true,type_SControlChart,column,split_n);
//            QStringList s_cc_list=file_io::ReadCSVtoQSL(&filename_rust_output);
//            control_chart s_cc=control_chart::from_qsl(s_cc_list);
//            ccv->addControlChartToLayout(s_cc,scc_name);
//        }


        //ccv->setModal(true);
        ccv->show();
        }
    });



}



void MainWindow::resizeTable(bool rs_row,bool addrm){
    int currentrow=ui->tableWidget->currentRow();
    int currentcolumn=ui->tableWidget->currentColumn();
    if (rs_row){
        if(addrm){
            ui->tableWidget->insertRow(currentrow);
        }
        else{
            ui->tableWidget->removeRow(currentrow);
        }
    }
    else{
        if(addrm){
            ui->tableWidget->insertColumn(currentcolumn);
        }
        else{
            ui->tableWidget->removeColumn(currentcolumn);
        }
    }
}

MainWindow::~MainWindow()
{
    delete ui;
}

