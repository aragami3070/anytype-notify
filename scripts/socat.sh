#!/bin/bash

# Проверяем наличие socat
if ! [ -x "$(command -v socat)" ]; then
  echo 'Error: socat is not installed.' >&2
  exit 1
fi

# Создаем файл с настройками для сервиса
if [ ! -f "/etc/systemd/system/tunnel.service" ]; then
	sudo touch /etc/systemd/system/tunnel.service
fi
