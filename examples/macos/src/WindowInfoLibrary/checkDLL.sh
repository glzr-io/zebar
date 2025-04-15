#!/bin/bash
x86_64-w64-mingw32-objdump -p bin/Release/net8.0-windows/win-x64/publish/WindowInfoLibrary.dll | grep GetActiveWindowTopLevelMenuItems
