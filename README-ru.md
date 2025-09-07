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
- Anytype версии не ранее v0.46.6
- Docker и docker-compose
- Аккаунт в [matrix.org](https://matrix.org) (или self-hosted matrix)
- [Socat](https://github.com/3ndG4me/socat)
- Systemd (либо перепишите [socat.sh](./scripts/socat.sh) на что-то другое, для автоматического запуска socat)

## Первые шаги
### Настройка Anytype
Для работы сервиса уведомлений необходим сервер или иное устройство с постоянно работающим приложением Anytype. При этом на текущий момент Anytype (v0.47.6) не может работать без графического окружения, поэтому на сервере придётся установить GUI. Советую выбрать наиболее легковесное (Mate, XFCE, или иное). 

Установите клиент [Anytype](https://anytype.io). Рекомендую сделать это через AppImage:
```sh
curl -L -o Anytype.AppImage "https://download.anytype.io/Anytype-x86_64.AppImage"
chmod +x Anytype.AppImage
```
После установки Anytype может неправильно работать при отсутствии следующих пакетов: `gnome-keyring libsecret-1-dev`. 

Затем нужно создать для этого сервиса отдельный аккаунт (например с именем Notifier) в вашем спейсе Anytype с ролью Viewer, добавить Anytype в автозапуск и включить автоматический вход в систему при старте сервера (иначе Anytype API может не заработать автоматически после включения). Также не забудьте при создании аккаунта в self-hosted спейсе выбрать правильный режим работы Anytype, а также указать ваш файл конфигурации для [Self-hosted](https://doc.anytype.io/anytype-docs/advanced/data-and-security/self-hosting/self-hosted). 

Также в спейсе Anytype потребуется создать следующие типы объектов:
- "Task"
    - Объекты с названием и описанием задачи, который должен иметь следующие поля:
        - "Notify" с типом Checkbox
        - "Proposed by" с типом Object и в Limit Objects Types выставленным типом "Space member"  
        - "Assignee" с типом Object и в Limit Objects Types выставленным типом "Space member"
            Пример: ![[./examples/Task-Properties.png]]
        - "Due date" с типом Date
- "Matrix Member"
    - Объекты для сопоставления Space member, указанного в полях "Proposed by" и "Assignee" с Matrix ID этого пользователя, который должен иметь следующие поля:
        - "Anytype ID" с типом Object и в Limit Objects Types выставленным типом "Space member"
        - "Matrix ID" с типом Text (в формате @username:matrix.org или @username:your-server.domain)
    - Для корректного отображения желательно создать такие сопоставления для каждого пользователя в вашем спейсе Anytype, у которого имеется Matrix аккаунт

### Настройка Matrix
Создать бота в вашем Matrix сервере (или [matrix.org](https://matrix.org)). Ботом в этом случае является самый обычный аккаунт.

Добавить бота в комнату, в которую будут присылаться уведомления. 

### Установка
После выполнения вышеперечисленных действий установите сам сервис:
```sh
git clone git@github.com:aragami3070/anytype-notify.git
cd anytype-notify
```

### Настройка
- Создайте .env файл в корне проекта:
```sh
cp .env.example .env
```
- Заполните .env вашими данными:
    - ANYTYPE_URL - URL вашего спейса Anytype
        - Если этот сервис находится на одном сервере с запущенным клиентом Anytype, то `anytype_ip` будет `localhost`
        - `space_id` можно узнать в самом Anytype, зайдя в настройки спейса -> General -> More -> Space information -> Space ID 
    - ANYTYPE_TOKEN - ключ доступа для Anytype API
        - Создаётся в Anytype в настройках аккаунта Notifier-а -> API Keys -> Create new
    - MATRIX_SERVER - сервер Matrix, на котором создан аккаунт бота
    - MATRIX_USER - имя аккаунта, используемого в роли бота
    - MATRIX_PASSWORD - пароль от аккаунта бота
    - MATRIX_ROOM_ID - ID комнаты, куда был добавлен бот, и куда он будет отправлять уведомления
        - Можно найти в Matrix в Room Settings -> Advanced -> Internal room ID

- При надобности измените настройки в файле [config.toml](./config.toml). 
    - Как часто нужно проверять новые объекты в Anytype (в минутах)
    - Какой тип имеют объекты с сопоставлением Anytype ID к Matrix ID (если он отличается от "Matrix Member")
    - С какой частотой присылать повторные напоминания о задаче, если её никто на себя не взял, то есть поле Assignee пусто (в днях)

### Запуск
Запустите службу socat для проксирования порта Anytype API, чтобы контейнер мог к нему обращаться:
```sh
chmod +x scripts/socat.sh
sudo ./scripts/socat.sh
```
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
