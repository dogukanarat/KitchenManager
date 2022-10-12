#ifndef INCLUDED_MAINWINDOW_H
#define INCLUDED_MAINWINDOW_H

#include <QtWidgets/QMainWindow>

namespace Ui
{
    class MainWindow;
}

class KmMainWindow : public QMainWindow
{
    Q_OBJECT

public:
    KmMainWindow(QWidget *parent = 0);
    ~KmMainWindow();

private:
    Ui::MainWindow *m_ui;
};

#endif
