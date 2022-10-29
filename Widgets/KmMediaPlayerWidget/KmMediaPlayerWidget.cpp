#include "KmMediaPlayerWidget/KmMediaPlayerWidget.h"
#include "ui_KmMediaPlayerWidget.h"
#include "QtWidgets/QFileDialog"

KmMediaPlayerWidget::KmMediaPlayerWidget(QWidget *parent)
    : KmBaseWidget{parent}
    , m_ui{new Ui::KmMediaPlayerWidget{}}
{
    m_ui->setupUi(this);

    // // set widget background color as greeen
    // QPalette pal = QPalette();
    
    // pal.setColor(QPalette::Window, QColor(123, 100, 50));

    // this->setAutoFillBackground(TRUE); 
    // this->setPalette(pal);

    // set spacing 0
    this->layout()->setContentsMargins(0, 0, 0, 0);

    init();

    QString fileName = QFileDialog::getOpenFileName(this, "Open a File", "", "Video File (*.avi, *.mpg, *.mp4)");

    m_mediaPlayer->stop();
    m_mediaPlayer->setMedia(QUrl::fromLocalFile(fileName));
    m_mediaPlayer->play();
}

KmMediaPlayerWidget::~KmMediaPlayerWidget()
{
}

void KmMediaPlayerWidget::init()
{
    m_videoWidget = new QVideoWidget();
    m_mediaPlayer = new QMediaPlayer();

    m_mediaPlayer->setVideoOutput(m_videoWidget);

    initLayout();
}

void KmMediaPlayerWidget::initLayout()
{
    m_ui->mainLayout->addWidget(m_videoWidget);
}