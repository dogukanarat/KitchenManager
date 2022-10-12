#include "MainWindow.h"
#include <QApplication>
#include <iostream>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    KmMainWindow w;
    w.show();

    return a.exec();
}
