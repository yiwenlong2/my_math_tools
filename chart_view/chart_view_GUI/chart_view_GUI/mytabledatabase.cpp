
#include "mytabledatabase.h"



mytabledatabase::mytabledatabase(QWidget *parent) : QTableWidget(parent)
{
    this->setColumnCount(10);
    this->setRowCount(100);

}

bool mytabledatabase::event(QEvent *event){
    return true;
}

