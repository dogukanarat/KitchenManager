#include "KmApiCommunication/KmApiCommunication.h"
#include "QDebug"
#include <QtSql/QSqlQuery>
#include <QtWidgets/QMessageBox>
#include <QtSql/QSqlError>

KmApiCommunication::KmApiCommunication(QObject *parent)
    : QObject(parent)
{

}

KmApiCommunication::~KmApiCommunication()
{
    
}