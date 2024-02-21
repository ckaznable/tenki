#!/bin/bash

if [ "$(id -u)" -eq 0 ]; then
  TARGET="/usr/local/bin/tenki"
else
  if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
    TARGET="$HOME/.local/bin/tenki"
  elif [[ ":$PATH:" == *":$HOME/bin:"* ]]; then
    TARGET="$HOME/bin/tenki"
  else
    echo "This operation requires root privileges. Please run this script as root or use sudo to obtain the necessary permissions."
    exit 1
  fi
fi

if [ -n "$1" ] && [ "$1" = "uninstall" ] && [ -f "$TARGET" ]; then
	rm $TARGET
else
  cp -r target/release/tenki $TARGET
  chmod +x $TARGET
  echo "The installation was successful. The application is now installed in $TARGET"
fi

