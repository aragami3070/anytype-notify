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

# Настройки сервиса для автозапуска socat при запуске системы
service="
[Unit]\n
Description=Socat tunnel from 0.0.0.0:20390 to localhost:31009\n
After=network.target\n
[Service]\n
ExecStart=/usr/bin/socat TCP-LISTEN:20390,reuseaddr,fork TCP4:localhost:31009\n
Restart=always\n
User=root\n
Group=root\n
[Install]\n
WantedBy=multi-user.target\n
"

# Записываем настройки сервиса
sudo -i echo -e "$service" > /etc/systemd/system/tunnel.service

# Перезапускаем сервис
sudo systemctl daemon-reload
sudo systemctl enable --now tunnel.service
