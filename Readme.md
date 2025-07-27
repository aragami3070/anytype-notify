# Anytype Notify
Уведомления о новых объектах созданных в workspace в Anytype.

## Зависимости
- Anytype
- Docker-compose
- Аккаунт в matrix.org (или self-hosted matrix)
- [Socat](https://github.com/3ndG4me/socat)
- Systemd (либо перепишите script/socat.sh на что-то другое, для автоматического запуска socat)

## Установка
```bash
git clone git@github.com:aragami3070/anytype-notify.git
cd anytype-notify
```


## Запуск
- Заполните необходимые переменные в файле .env, пример можно посмотреть в [.env.example](https://github.com/aragami3070/anytype-notify/blob/master/.env.example)
- Укажите как часто нужно проверять новые объекты в Anytype в файле config.toml
- Укажите как типы объектов нужно проверять в Anytype в файле config.toml
- Поднимите контейнер из docker-compose.yaml

```bash
sudo docker-compose up --no-start 
```
- Запустите контейнер

```bash
sudo docker start anytype-notifier
```
