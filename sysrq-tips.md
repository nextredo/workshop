# Tips
- Use dedicated `SysRq` if present
- Use `PrtScn` if not

## FS Interaction
### Query Capabilities
```bash
# Get bitmask in decimal
cat /proc/sys/kernel/sysrq

# Get bitmask in hex
printf '0x%x\n' $(cat /proc/sys/kernel/sysrq)

# Get bitmask in binary
# (more complex since printf doesn't have a binary format specifier)
cat /proc/sys/kernel/sysrq | python -c 'import sys; [print(format(int(line), "b")) for line in sys.stdin]'
```

### Send Commands Via Filesystem
```bash
sudo sh -c 'echo 's' > /proc/sysrq-trigger'
```

## Triggering Magic SysRq Key
### Lenovo ThinkPad P52s
- Press & hold left alt
- Press & hold printscreen
- Press s
- Emergency sync triggered
- Release

### HP EliteBook 640 (HSN-Q38C-4)
- Press & hold left alt
- Press & hold fn
- Press printscreen
- Release fn + printscreen, keep holding alt
- Press s
- Emergency sync triggered
- Release

## Common Operations
### Links Go Here
- See [the wikipedia page][sysrq_wiki]

### REISUB

[sysrq_wiki]: https://en.wikipedia.org/wiki/Magic_SysRq_key
