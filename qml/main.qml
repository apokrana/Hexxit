import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Dialogs
import QtQuick.Layouts

import Hexxit

ApplicationWindow {
    visible: true
    width: 960
    height: 540
    title: "Hexxit"

    Material.theme: Material.Dark
    Material.accent: Material.Orange

    property bool loaded: false
    property bool loading: false
    property var fileInfo: ({})

    property int currentFile: 0
    property int fileCount: 0

    // Hex table
    property int hexTotalRows: 0
    property int currentRowOffset: 0
    property string hexText: ""

    function reloadHex() {
        var rowHeight = 17
        var firstRow = Math.max(0, Math.floor(hexScroll.contentY / rowHeight))
        var visibleRows = Math.ceil(hexScroll.height / rowHeight) + 50
        currentRowOffset = firstRow
        Backend.getHexData(currentFile, firstRow, visibleRows)
    }

    Connections {
        target: Backend

        function onFileLoadedStatus(success) {
            loaded = success
            loading = false
            if (success) {
                fileCount = Backend.getFileCount()
                hexTotalRows = Math.ceil(Backend.getFileSize(currentFile) / 16)
                currentRowOffset = 0
                reloadHex()
            }
        }

        function onFileLoadStart(started)  {
            loading = started
        }

        function onFileInfo(name, arch, size, magic) {
            fileInfo = {
                "name": name,
                "size": size,
                "magic": magic,
                "arch": arch
            }
        }

        function onHexData(text) {
            hexText = text
        }
    }

    Timer {
        interval: 100
        running: true
        repeat: true
        onTriggered: Backend.pollResults()
    }

    FileDialog {
        id: fileDialog
        title: "Select a file"
        onAccepted: {
            Backend.loadFile(fileDialog.selectedFile)
        }
    }

    DropArea {
        anchors.fill: parent
        onDropped: (drop) => {
            if (drop.hasUrls) {
                Backend.loadFile(drop.urls[0])
            }
        }
    }
    
    menuBar: MenuBar {
        height: 26

        delegate: MenuBarItem {
            implicitHeight: 26
            implicitWidth: 50

            contentItem: Text {
                text: parent.text
                font.pixelSize: 12
                color: "white"
                verticalAlignment: Text.AlignVCenter
            }
        }

        Menu {
            title: "File"

            delegate: MenuItem {
                implicitHeight: 24
                implicitWidth: 140

                contentItem: Text {
                    text: parent.text
                    font.pixelSize: 12
                    color: "white"
                    verticalAlignment: Text.AlignVCenter
                }
            }

            Action {
                text: "Load File..."
                shortcut: StandardKey.Open
                onTriggered: fileDialog.open()
            }

            Action {
                text: "Exit"
                shortcut: StandardKey.Quit
                onTriggered: Qt.quit()
            }
        }

        Menu {
            title: "Edit"

            delegate: MenuItem {
                implicitHeight: 24
                implicitWidth: 140

                contentItem: Text {
                    text: parent.text
                    font.pixelSize: 12
                    color: "white"
                    verticalAlignment: Text.AlignVCenter
                }
            }

            Action {
                text: "Copy"
                shortcut: StandardKey.Copy
                onTriggered: Backend.copyHexData("raw", currentFile, hexView.selectionStart, hexView.selectionEnd, currentRowOffset);
                
            }
            Action {
                text: "Copy As Vector (C++)"
                onTriggered: Backend.copyHexData("cpp", currentFile, hexView.selectionStart, hexView.selectionEnd, currentRowOffset);
            }
            Action {
                text: "Copy As Vector (Rust)"
                onTriggered: Backend.copyHexData("rust", currentFile, hexView.selectionStart, hexView.selectionEnd, currentRowOffset);
            }

            Action {
                text: "Select All"
                shortcut: StandardKey.SelectAll
                onTriggered: hexText.SelectAll()
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0

            // Left sidebar
            Rectangle {
                Layout.preferredWidth: 200
                Layout.fillHeight: true
                color: Material.background

                Column {
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 10

                    Text {
                        text: fileInfo.name
                            ? "Name: " + fileInfo.name
                            : "Name:"
                        color: "white"
                    }

                    Text {
                        text: {
                            fileInfo.arch 
                            ? "Arch:  " + fileInfo.arch
                            : "Arch:"
                        }
                        color: "white"
                    }

                    Text {
                        text: fileInfo.size
                            ? "Size: " + fileInfo.size + " bytes"
                            : "Size:"
                        color: "white"
                    }

                    Text {
                        text: fileInfo.magic
                            ? "Magic: 0x" + fileInfo.magic.toString(16).toUpperCase()
                            : "Magic:"
                        color: "white"
                    }
                }
            }

            // Hex viewer and tabs
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 0

                TabBar {
                    id: tabs
                    Layout.fillWidth: true

                    Repeater {
                        model: fileCount

                        TabButton {
                            text: Backend.getFileName(index)

                            onClicked: {
                                currentFile = index
                                Backend.getHexData(index, 0, 0)
                            }
                        }
                        
                    }
                }

                // Hex area
                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: Qt.lighter(Material.background, 1.1)
                    clip: true

                    BusyIndicator {
                        anchors.centerIn: parent
                        running: loading
                        visible: loading
                    }

                    Flickable {
                        id: hexScroll
                        anchors.fill: parent
                        visible: loaded
                        clip: true
                        contentWidth: width
                        contentHeight: hexTotalRows * 17

                        ScrollBar.vertical: ScrollBar {}
                        ScrollBar.horizontal: ScrollBar { policy: ScrollBar.AlwaysOff }

                        onContentYChanged: {
                            var rowHeight = 17
                            var firstRow = Math.max(0, Math.floor(contentY / rowHeight))
                            if (firstRow !== currentRowOffset) {
                                currentRowOffset = firstRow
                                var visibleRows = Math.ceil(hexScroll.height / rowHeight) + 50
                                Backend.getHexData(currentFile, firstRow, visibleRows)
                            }
                        }
                    
                        TextEdit {
                            id: hexView
                            y: currentRowOffset * 17
                            width: hexScroll.width
                            text: hexText
                            readOnly: true
                            selectByMouse: true
                            textFormat: TextEdit.PlainText
                            selectionColor: Material.accent
                            selectedTextColor: "#000000"
                            font.family: "Courier New"
                            font.pixelSize: 13
                            color: "#d4d4d4"
                            leftPadding: 8
                            topPadding: 4
                            wrapMode: TextEdit.NoWrap
                        }
                    }
                }
            }
        }
    }
}