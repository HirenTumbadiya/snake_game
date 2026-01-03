; NSIS installer script for Snake Game
; Place snake_game.exe next to this script before building (build script will copy it)

!define PRODUCT_NAME "Snake Game"
!define PRODUCT_VERSION "0.1.0"

OutFile "../../snake_game-installer-${PRODUCT_VERSION}.exe"
InstallDir "$PROGRAMFILES\\${PRODUCT_NAME}"
RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"
    SetOutPath "$INSTDIR"
    File "snake_game.exe"

    ; Create shortcuts
    CreateDirectory "$SMPROGRAMS\\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\\${PRODUCT_NAME}\\${PRODUCT_NAME}.lnk" "$INSTDIR\\snake_game.exe"
    CreateShortCut "$DESKTOP\\${PRODUCT_NAME}.lnk" "$INSTDIR\\snake_game.exe"

    ; Write uninstall information
    WriteUninstaller "$INSTDIR\\uninstall.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\\snake_game.exe"
    Delete "$INSTDIR\\uninstall.exe"
    Delete "$SMPROGRAMS\\${PRODUCT_NAME}\\${PRODUCT_NAME}.lnk"
    RMDir "$SMPROGRAMS\\${PRODUCT_NAME}"
    Delete "$DESKTOP\\${PRODUCT_NAME}.lnk"
    RMDir "$INSTDIR"
SectionEnd
