# Linux Command Line Tips & Tricks

## Shortcuts

>From [this page][1]<br>

- `Ctrl + A` - Move to the start of the line.
- `Ctrl + E` - Move to the end of the line.
- `Ctrl + U` - Delete from the cursor to the start of the line.
- `Ctrl + K` - Delete from the cursor to the end of the line.
- `Ctrl + W` - Delete the word before the cursor.
- `Ctrl + L` - Clear the terminal screen.
- `Ctrl + C` - Stop the current process/command.
- `Ctrl + D` - Log out or exit the terminal.
- `Ctrl + Z` - Pause the current process (can be resumed).
- `Ctrl + R` - Search command history (backward search).
- `Up Arrow` - Show the previous command (from the command history).
- `Down Arrow` - Show the next command (from the command history).
- `!!` - Repeat the last command.
- `!n` - Repeat the nth command from history.
- `Tab` - Auto-complete commands, files, or directories.
- `Tab` twice - List all possible completions.
- `Ctrl + Shift + C` - Copy the selected text or command.
- `Ctrl + Shift + V` - Paste copied text or command.
- `Ctrl + Shift + N` - Open a new terminal window.
- `Ctrl + Shift + T` - Open a new tab in the terminal.
- `Ctrl + Tab` or `Ctrl + PageDown` - Switch between terminal tabs.

## `chmod` and Linux File Permissions

- You may have seen things along the lines of `chmod 600 <file>`
- This octal string at the end is the permissions
- The higher the number, the more permissions that entity can access the file with
  - e.g. `600`
    - File's owner has read / write access
    - File's group has no access at all
    - Others have no access at all

<!-- Links -->
[1]: https://itsfoss.com/linux-terminal-shortcuts/
[2]: https://www.stationx.net/linux-file-permissions-cheat-sheet/
[3]: https://linuxcommand.org/lc3_learning_the_shell.php
