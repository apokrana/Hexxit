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

    property var hexRows: []

    Connections {
        target: Backend

        function onFileLoadedStatus(success) {
            loaded = success
            loading = false
            if (success) {
                fileCount =  Backend.getFileCount()
                Backend.getHexData(currentFile, 0, 0)
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

        function onHexData(rows) {
            hexRows = rows
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
                onTriggered: Backend.copyRaw(currentFile);
            }
            Action {
                text: "Copy As Vector (C++)"
                onTriggered: Backend.copyAsVecCpp(currentFile);
            }
            Action {
                text: "Copy As Vector (Rust)"
                onTriggered: Backend.copyAsVecRs(currentFile);
            }

            Action {
                text: "Select All"
                shortcut: StandardKey.SelectAll
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

                    BusyIndicator {
                        anchors.centerIn: parent
                        running: loading
                        visible: loading
                    }

                    ListView {
                        anchors.fill: parent
                        visible: loaded
                        clip: true
                        model: hexRows
                        cacheBuffer: 2000

                        delegate: Text {
                            width: ListView.view.width
                            text: modelData

                            font.family: "Courier New"
                            font.pixelSize: 13

                            color: "#d4d4d4"
                            leftPadding: 8
                        }
                    }
                }
            }
        }
    }
}