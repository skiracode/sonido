QT       += core gui qml
greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++11

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    core/engine.cpp \
    core/instrument.cpp \
    core/instrumentalline.cpp \
    core/interface.cpp \
    core/songdescripton.cpp \
    core/timednote.cpp \
    core/wave.cpp \
    gui/dummy.cpp \
    gui/mainwindow.cpp \
    gui/noteitem.cpp \
    gui/pianoroll.cpp \
    main.cpp \

HEADERS += \
    core/engine.h \
    core/instrument.h \
    core/instrumentalline.h \
    core/interface.h \
    core/songdescripton.h \
    core/timednote.h \
    core/wave.h \
    gui/dummy.h \
    gui/mainwindow.h \
    gui/noteitem.h \
    gui/pianoroll.h \

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

unix:!macx: LIBS += -lsndfile

RESOURCES += \
    resources.qrc
