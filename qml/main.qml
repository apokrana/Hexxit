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

    Connections {
        target: Backend
        
        onFileLoadedStatus: (success) => {
            if (success) {
                console.log("File load success")
                loadButton.visible = false
            } else {
                console.log("File load failed")
                loadButton.visible = true
            }
        }

        onFileLoadStart: (started) => {
            if (started) {
                console.log("File load start")
                loadButton.visible = false
            } else {
                console.log("File load stop")
                loadButton.visible = true
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

    Button {
        id: loadButton
        anchors.centerIn: parent
        text: "Load File"
        onClicked: fileDialog.open()
        visible: true
    }

    
    
}