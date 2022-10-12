#include "KmSessionWidget/KmSessionWidget.h"
#include "ui_KmSessionWidget.h"
#include "QtWidgets/QLineEdit"
#include "QtWidgets/QLabel"
#include "QtWidgets/QMessageBox"

KmSessionWidget::KmSessionWidget(QWidget *parent)
    : KmBaseWidget(parent), m_ui{new Ui::KmSessionWidget{}}
{
    m_ui->setupUi(this);

    initializeGrid(m_ui->gridLayout12, 12);
    initializeGrid(m_ui->gridLayout21, 21);
    initializeGrid(m_ui->gridLayout22, 22);

    for (auto itOrderView = m_orderViewMap.begin();
         itOrderView != m_orderViewMap.end(); 
         ++itOrderView)
    {
        auto widget = itOrderView.value();
        auto id = itOrderView.key();

        emit widget->setOrderId(QString::number(id));
        emit widget->setOrderStatus(KmOrderViewWidget::ORDER_STATUS_PREPARING);
    }
}

KmSessionWidget::~KmSessionWidget()
{
}

void KmSessionWidget::initializeGrid(QGridLayout *layout, KUInt32 parentId)
{
    m_orderViewMap[parentId * 100 + 0] = new KmOrderViewWidget{this};
    m_orderViewMap[parentId * 100 + 1] = new KmOrderViewWidget{this};
    m_orderViewMap[parentId * 100 + 10] = new KmOrderViewWidget{this};
    m_orderViewMap[parentId * 100 + 11] = new KmOrderViewWidget{this};

    layout->addWidget(m_orderViewMap[parentId * 100 + 0], 0, 0);
    layout->addWidget(m_orderViewMap[parentId * 100 + 1], 0, 1);
    layout->addWidget(m_orderViewMap[parentId * 100 + 10], 1, 0);
    layout->addWidget(m_orderViewMap[parentId * 100 + 11], 1, 1);
}
