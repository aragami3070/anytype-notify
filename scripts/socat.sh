#!/bin/bash

# Проверяем наличие socat
if ! [ -x "$(command -v socat)" ]; then
  echo 'Error: socat is not installed.' >&2
  exit 1
fi
