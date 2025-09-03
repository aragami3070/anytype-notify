# Anytype Notify
Change language: [English](./README.md)

Уведомления о новых объектах созданных в workspace в Anytype.
<details>
	<summary>Содержание</summary>

* [Зависимости](#Зависимости)
* [Установка](#Установка)
* [Запуск](#Запуск)
* [Внести свой вклад](#Внести-свой-вклад)
* [Лицензия](#Лицензия)
</details>

## Зависимости
- Anytype
- Docker или docker-compose
- Аккаунт в matrix.org (или self-hosted matrix)
- Аккаунт в [matrix.org](https://matrix.org) (или self-hosted matrix)
- [Socat](https://github.com/3ndG4me/socat)
- Systemd (либо перепишите [socat.sh](./scripts/socat.sh) на что-то другое, для автоматического запуска socat)

## Установка
```sh
git clone git@github.com:aragami3070/anytype-notify.git
cd anytype-notify
```


## Запуск
- Создайте .env файл:
```sh
cp .env.example .env
```
- Заполните .env вашими данными
- Укажите как часто нужно проверять новые объекты в Anytype и какой тип имеют обекты с сопоставлением Anytype ID к Matrix ID в [config.toml](./config.toml)
- Поднимите контейнер из docker-compose.yaml
```sh
sudo docker-compose up --no-start 
```
- Запустите контейнер
```sh
sudo docker start anytype-notifier
```

## Внести свой вклад

Смотрите [CONTRIBUTING.md](CONTRIBUTING.md).

## Лицензия

[License](License)
