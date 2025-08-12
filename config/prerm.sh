#!/bin/bash
set -e
systemctl stop yell-o.service
systemctl disable yell-o.service
deluser --system "yell-o" 2>dev/null || true