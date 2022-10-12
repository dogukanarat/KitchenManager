#ifndef INCLUDED_KMMEDIAPLAYERWIDGET_H
#define INCLUDED_KMMEDIAPLAYERWIDGET_H

#include "QtWidgets/QWidget"
#include "KmCommon/KmCommon.h"
#include "KmBaseWidget/KmBaseWidget.h"
#include "QtMultimedia/QMediaPlayer"
#include "QtMultimediaWidgets/QVideoWidget"

namespace Ui
{
    class KmMediaPlayerWidget;
}

class KmMediaPlayerWidget : public KmBaseWidget
{
    Q_OBJECT

public:
    KmMediaPlayerWidget(QWidget *parent = 0);
    virtual ~KmMediaPlayerWidget();

protected:
    void init();
    void initLayout();


private:
    Ui::KmMediaPlayerWidget *m_ui;
    QMediaPlayer *m_mediaPlayer;
    QVideoWidget *m_videoWidget;
};

#endif // INCLUDED_KMMEDIAPLAYERWIDGET_H
