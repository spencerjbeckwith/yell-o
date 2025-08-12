#!/bin/bash
set -e
systemctl stop yell-o.service
systemctl disable yell-o.service
systemctl stop pulseaudio.service
systemctl disable pulseaudio.service
deluser --system "yell-o" 2>dev/null || true