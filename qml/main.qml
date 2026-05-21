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

    FileDialog {
        id: fileDialog
        title: "Select a file"
        onAccepted: {
            Backend.loadFile(fileDialog.selectedFile)
        }
    }
    Button {
        anchors.centerIn: parent
        text: "Load File"
        onClicked: fileDialog.open()
    }
    
    // ListView {
    //     id: processView
    //     anchors.fill: parent

    //     model: Backend.getProcesses()

    //     delegate: Rectangle {
    //         width: parent.width
    //         height: 40
    //         color: index % 2 === 0 ? "#202020" : "#2A2A2A"

    //         Text {
    //             anchors.centerIn: parent
    //             color: "white"
    //             text: modelData
    //         }
    //     }
    // }

    // Button {
    //     anchors.bottom: parent.bottom
    //     anchors.horizontalCenter: parent.horizontalCenter
    //     text: "Refresh"

    //     onClicked: processView.model = Backend.getProcesses()
    // }
}