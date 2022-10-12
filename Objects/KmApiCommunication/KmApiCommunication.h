#ifndef INCLUDED_KMAPICOMMUNICATION_H
#define INCLUDED_KMAPICOMMUNICATION_H

#include "QtCore/QObject"
#include "QtSql/QSqlDatabase"
#include "KmCommon/KmCommon.h"

class KmApiCommunication : public QObject
{
    Q_OBJECT

public:
    KmApiCommunication(QObject *parent = 0);
    ~KmApiCommunication();

private:
};

#endif // INCLUDED_KMAPICOMMUNICATION_H
