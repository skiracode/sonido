import sys
from typing import Optional
from PySide6.QtWidgets import (QApplication, QWidget, QStackedLayout, QGraphicsScene, QGraphicsView, QMainWindow,
                               QGraphicsWidget, QStyleOptionGraphicsItem)
from PySide6.QtGui import (QPainter)
from PySide6.QtCore import (QRectF)


def test():
    app = QApplication(sys.argv)
    window = MainWindow(app.primaryScreen())
    window.show()
    sys.exit(app.exec_())


class NoteItem(QGraphicsWidget):
    def __init__(self, width):
        super().__init__()
        self.pen_width = 1.0 / 16.0
        self.width = width

    def paint(self, painter: QPainter, option: QStyleOptionGraphicsItem, widget: Optional[QWidget] = ...) -> None:
        brush = self.palette().windowText()
        rect = QRectF(0.0, 0.0, self.width, 1.0)
        painter.fillRect(rect, brush)

    def boundingRect(self) -> QRectF:
        return QRectF(self.pen_width, self.pen_width, self.width - self.pen_width, 1.0 - self.pen_width)


class MainWindow(QMainWindow):
    def __init__(self, screen):
        super().__init__()
        self.screen = screen
        self.resize(self.screen.geometry().size() * 3.0 / 4.0)
        self.move(self.screen.geometry().center() - self.frameGeometry().center())
        self.piano_roll = PianoRoll()
        self.setCentralWidget(self.piano_roll)


class PianoRoll(QWidget):
    def __init__(self):
        super().__init__()
        self.octaves = 8
        self.stacked_layout = QStackedLayout()
        self.setLayout(self.stacked_layout)
        self.scene = QGraphicsScene()
        self.scene.setSceneRect(0.0, 0.0, 16.0 * 8.0, 12.0 * self.octaves)
        note_item = NoteItem(1.0)
        self.scene.addItem(note_item)
        note_item.setPos(2.0, 1.0)
        self.view = QGraphicsView(self.scene)
        self.stacked_layout.addWidget(self.view)
        self.stacked_layout.setCurrentWidget(self.view)
        self.view.centerOn(0.0, 0.0)
        self.view.scale(64.0, 64.0)
