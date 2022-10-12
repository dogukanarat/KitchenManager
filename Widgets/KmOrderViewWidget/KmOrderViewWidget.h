#ifndef INCLUDED_KMORDERWIDGET_H
#define INCLUDED_KMORDERWIDGET_H

#include "QtWidgets/QWidget"
#include "QtWidgets/QFrame"
#include "KmCommon/KmCommon.h"
#include "KmBaseWidget/KmBaseWidget.h"

namespace Ui
{
    class KmOrderViewWidget;
}

class KmOrderViewWidget : public QFrame
{
    Q_OBJECT

public:
    typedef enum
    {
        ORDER_STATUS_PREPARING = 0,
        ORDER_STATUS_READY,
    } OrderStatus;

    KmOrderViewWidget(QWidget *parent = 0);
    virtual ~KmOrderViewWidget();

public slots:
    void setOrderId(const QString& orderId);
    void setOrderStatus(OrderStatus orderStatus);

private:
    Ui::KmOrderViewWidget* m_ui;
    OrderStatus m_orderStatus;
};

#endif // INCLUDED_KMORDERWIDGET_H
