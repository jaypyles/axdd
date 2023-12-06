# Summary

A simple tool to be used with systemd to create a daemon which recognizes display switching.

## Systemctl Configuration

axdd.service

```bash
[Unit]
Description=axdd
After=network.target

[Service]
ExecStart=/home/user/axdd
Restart=no

[Install]
Alias=axdd.service
WantedBy=multi-user.target
```

`systemctl --user daemon-reload`
`systemctl --user enable axdd.service`
`systemctl --user start axdd.service`
