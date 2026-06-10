# Cool Vim Tips
## Commands
### Bang `!` Operator
1. In normal mode

```
# Run command
:!{command}

# Run command, with % as current filename placeholder
:!{command} %

# Run command, put output into this file
:r !{command}

# Run command, substitute this file's contents with the output of the command
# Good for sorting - e.g. with `sort`
:%!{command}

# Run current line in buffer as a command
# Can also yy (yank line) then :!<ctrl+R then "> to run it
:.w !sh

echo hello
```
