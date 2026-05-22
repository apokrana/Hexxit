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

    property var hexRows: []

    Connections {
        target: Backend

        function onFileLoadedStatus(success) {
            loaded = success
            loading = false
            if (success) {
                Backend.getHexData(0, 0, 0)
            }
        }

        function onFileLoadStart(started)  {
            loading = started
        }

        function onFileInfo(name, size, magic) {
            fileInfo = {
                "name": name,
                "size": size,
                "magic": magic
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

    RowLayout {
        anchors.fill: parent

        Rectangle {
            Layout.preferredWidth: 200
            Layout.fillHeight: true
            color: Material.background

            Column {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 10

                Text {
                    text: "File Info"
                    font.bold: true
                    color: "white"
                }

                Text {
                    text: fileInfo.name ? "Name: " + fileInfo.name : ""
                    color: "white"
                }

                Text {
                    text: fileInfo.size ? "Size: " + fileInfo.size + " bytes" : ""
                    color: "white"
                }

                Text {
                    text: fileInfo.magic
                        ? "Magic: 0x" + fileInfo.magic.toString(16).toUpperCase()
                        : ""
                    color: "white"
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: Qt.lighter(Material.background, 1.1)

            BusyIndicator {
                id: loadingIndicator
                anchors.centerIn: parent
                
                running: loading 
                visible: loading
            }

            Button {
                anchors.centerIn: parent
                text: "Load File"
                visible: !loading && !loaded
                onClicked: fileDialog.open()
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