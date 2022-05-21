QT       += core gui
QT += core5compat
QT += charts
QT += svg
greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++11

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    controlchartview.cpp \
    file_io.cpp \
    main.cpp \
    mainwindow.cpp \
    mytabledatabase.cpp

HEADERS += \
    controlchartview.h \
    file_io.h \
    mainwindow.h \
    mytabledatabase.h

FORMS += \
    controlchartview.ui \
    mainwindow.ui

TRANSLATIONS += \
    chart_view_GUI_zh_CN.ts
CONFIG += lrelease
CONFIG += embed_translations

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target






win32:CONFIG(release, debug|release): LIBS += -L$$PWD/../../chart_view_c_api/target/release/ -lchart_view_c_api.dll
else:win32:CONFIG(debug, debug|release): LIBS += -L$$PWD/../../chart_view_c_api/target/debug/ -lchart_view_c_api.dll

INCLUDEPATH += $$PWD/../../chart_view_c_api/target/debug
DEPENDPATH += $$PWD/../../chart_view_c_api/target/debug

RESOURCES += \
    piclib.qrc

RC_ICONS = statistics.ico
