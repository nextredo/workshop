# Fixing DNS on OpenSUSE
- Using Cloudflare Warp and Tailscale screws up the DNS sometimes
- Commands to fix it are below

## Cmds
```bash
sudo tailscale down
warp-cli disconnect

# Check current DNS resolver options
cat /etc/resolv.conf

# Force regeneration of the file (which is overwritten by warp & tailscale anyways)
sudo netconfig update -f
```
