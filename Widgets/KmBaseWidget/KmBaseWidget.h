#ifndef INCLUDED_KMBASEWIDGET_H
#define INCLUDED_KMBASEWIDGET_H

#include "QtWidgets/QWidget"
#include "KmCommon/KmCommon.h"

class KmBaseWidget : public QWidget
{
    Q_OBJECT

public:
    KmBaseWidget(QWidget *parent = 0);
    ~KmBaseWidget();

private:
    KInt32 test;
};

#endif // INCLUDED_KMBASEWIDGET_H
