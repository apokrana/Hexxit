import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Dialogs

import Hexxit

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Hexxit"
    Material.theme: Material.Dark
    Material.accent: Material.Orange

    property bool loadSuccess: false

    FileDialog {
        id: fileDialog
        title: "Select a file"
        onAccepted: {
            loadSuccess = Backend.loadFile(fileDialog.selectedFile)
        }
    }

    Button {
        id: loadButton
        anchors.centerIn: parent
        text: "Load File"
        onClicked: fileDialog.open()

        visible: !loadSuccess
    }
    
}