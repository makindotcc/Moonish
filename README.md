# Moonish
Force enable immersive dark mode titlebar for whitelisted windows.

## Example program without moonish
![Window with white titlebar](assets%2Fdisabled_preview.png)
## Example program with moonish
![Window with dark, immersive titlebar](assets%2Fenabled_preview.png)

# Usage
- open file explorer and go to ``%localappdata%\Programs`` path:\
  !["%localappdata%\Programs" in windows explorer path field](assets%2Flocalprogramspath.png)
- download latest release from [releases](https://github.com/makindotcc/moonish/releases) and unzip it to
  ``Moonish`` directory
- Go to ``Moonish`` directory and run ``moonish.exe``.
- Add your desired window titles (case-sensitive) to immersive darkmode to ``whitelisted_windows.txt``.
Every line is part of window title that should have our dark immersive titlebar. Example:
``Minecraft`` will enable dark mode for windows with titles: ``Minecraft Launcher``, ``Launcher Minecraft 1.20`` etc.
- [Optional] enable auto launch on startup from icon tray:\
  ![Moonish trayicon context menu showing "Toggle auto start" menu item](assets%2Ftray_autolaunch.png)
- Reload config from tray icon.

# Logo credits
moon.svg, moon.ico - [tabler-icons.io](https://tabler-icons.io/)
