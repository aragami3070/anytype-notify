#!/bin/bash

# Получение всех своего IP адреса
ip a | grep "$1" | cut -d '/' -f1 | awk -F' ' '{print $2}' > ../assets/my-ip.txt

# Сканирование сети для нахождения всех IP подключенных к данной сети и исключение своего IP
sudo nmap -sP "$1.*" | grep -v "$(cat ../assets/my-ip.txt)" > ../assets/nmap-scan-result.txt

rm ../assets/my-ip.txt
