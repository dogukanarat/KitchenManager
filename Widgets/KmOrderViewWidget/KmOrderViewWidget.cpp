#include "KmOrderViewWidget/KmOrderViewWidget.h"
#include "ui_KmOrderViewWidget.h"

KmOrderViewWidget::KmOrderViewWidget(QWidget *parent)
    : QFrame{parent}
    , m_ui{new Ui::KmOrderViewWidget{}}
{
    m_ui->setupUi(this);

    QPalette pal = QPalette();

    pal.setColor(QPalette::Window, QColor(123, 129, 50));

    this->setAutoFillBackground(TRUE); 
    this->setPalette(pal);

    QFont idFont = m_ui->labelOrderId->font();
    idFont.setPointSize(32);
    idFont.setBold(true);
    m_ui->labelOrderId->setFont(idFont);

    QFont statusFont = m_ui->labelOrderStatus->font();
    statusFont.setPointSize(24);
    statusFont.setBold(true);
    m_ui->labelOrderId->setFont(statusFont);

    // QPainterPath path;
    // path.addRoundedRect(this->rect(), 5.0, 5.0);
    // QRegion mask = QRegion(path.toFillPolygon().toPolygon());
    // this->setMask(mask);
    
    // this->setStyleSheet(".QFrame{background-color: red; border: 5px solid black; border-radius: 100px;}");
}

KmOrderViewWidget::~KmOrderViewWidget()
{
}

void KmOrderViewWidget::setOrderId(const QString &orderId)
{
    m_ui->labelOrderId->setText(orderId);
}

void KmOrderViewWidget::setOrderStatus(OrderStatus orderStatus)
{
    m_orderStatus = orderStatus;

    QMap<OrderStatus, QString> statusMap =
    {
        {ORDER_STATUS_PREPARING, "Hazirlaniyor"},
        {ORDER_STATUS_READY, "Hazir"}
    };
    
    m_ui->labelOrderStatus->setText(statusMap[m_orderStatus]);
}