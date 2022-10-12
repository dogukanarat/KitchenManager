#ifndef INCLUDED_KMSESSIONWIDGET_H
#define INCLUDED_KMSESSIONWIDGET_H

#include "QtWidgets/QWidget"
#include "QtCore/QMap"
#include "QtWidgets/QMainWindow"
#include "QtWidgets/QGridLayout"
#include "KmCommon/KmCommon.h"
#include "KmBaseWidget/KmBaseWidget.h"
#include "KmApiCommunication/KmApiCommunication.h"
#include "KmOrderViewWidget/KmOrderViewWidget.h"

namespace Ui
{
    class KmSessionWidget;
}

class KmSessionWidget : public KmBaseWidget
{
    Q_OBJECT

public:
    KmSessionWidget(QWidget *parent = 0);
    virtual ~KmSessionWidget();

    void initializeGrid(QGridLayout* layout, KUInt32 parentId);

private:
    Ui::KmSessionWidget* m_ui;
    QMap<KUInt32, KmOrderViewWidget*> m_orderViewMap;
};
#endif // INCLUDED_KMSESSIONWIDGET_H
