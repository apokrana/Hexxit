import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Dialogs
import QtQuick.Layouts

import Hexxit

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Hexxit"

    Material.theme: Material.Dark
    Material.accent: Material.Orange

    property bool loaded: false
    property bool loading: false
    property var fileInfo: ({})

    Connections {
        target: Backend

        function onFileLoadedStatus(success) {
            loaded = success
            loading = false
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
            color: "#222222"

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
            color: "#333333"

            Button {
                anchors.centerIn: parent
                text: loading ? "Loading..." : "Load File"
                enabled: !loading
                visible: !loaded
                onClicked: fileDialog.open()
            }
        }
    }
}