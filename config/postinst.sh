#!/bin/bash
set -e
if ! id "yell-o" &>/dev/null; then
    adduser --system --group --no-create-hom --disabled-login "yell-o"
fi
usermod -aG audio "yell-o"
systemctl daemon-reexec
systemctl daemon-reload

systemctl enable pulseaudio.service
systemctl start pulseaudio.service

systemctl enable yell-o.service
systemctl start yell-o.service