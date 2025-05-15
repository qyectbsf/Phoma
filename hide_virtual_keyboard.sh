#! /bin/bash -xe

qdbus6 org.kde.KWin /VirtualKeyboard org.freedesktop.DBus.Properties.Set "org.kde.kwin.VirtualKeyboard" "active" false
