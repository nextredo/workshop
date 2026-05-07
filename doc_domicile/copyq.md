# Clipboard Manager
## [Installation][install-ln]
```bash
# Easiest to install via flatpak
# https://copyq.readthedocs.io/en/latest/installation.html
flatpak install --user --from https://flathub.org/repo/appstream/com.github.hluk.copyq.flatpakref
flatpak run com.github.hluk.copyq

# Add an autostart file for copyq via gnome-tweaks
```

## [Getting it working properly][working-ln]
```bash
# Run with this command
env QT_QPA_PLATFORM=xcb copyq

# Update the autoexec file (assumes flatpak install)
vim ~/.config/autostart/com.github.hluk.copyq.desktop

# Set contents of the file to the following (assumes flatpak install)
# The important part is the environment variable at the start
# Exec=env QT_QPA_PLATFORM=xcb /usr/bin/flatpak run --branch=stable --arch=x86_64 --command=copyq com.github.hluk.copyq --start-server show
```

<!-- Links -->
[working-ln]: https://copyq.readthedocs.io/en/latest/known-issues.html
[install-ln]: https://copyq.readthedocs.io/en/latest/installation.html
