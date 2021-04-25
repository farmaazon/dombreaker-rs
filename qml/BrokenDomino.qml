import QtQuick 2.15

Domino {
    id: brokenDomino
    PropertyAnimation on opacity {
        from: 1.0
        to: 0.0
        duration: 1500.0
        easing.type: Easing.InSine

//        Component.onDestroyed: console.log("DESTRYO");
//        Component.onDestruction: console.log("DESTRYO2", domino);

        onRunningChanged: {
            if (!running) {
                console.log("FINISHED", this, parent, running);
                brokenDomino.destroy()
                //parent.visible = false
            }
        }
    }

    Component.onDestruction: {
        console.log("DESTRYO3", this);
        console.trace()
    }

    //onDestroyed: {
//        console.log("DESTRYO5);
//        console.trace()
//    }
}
