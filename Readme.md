# Anytype Notify
Change language: [Russian](./README-ru.md)

Notifications for new Anytype objects in the workspace via Matrix
<details>
	<summary>Table of Contents</summary>

* [Dependencies](#Dependencies)
* [Installation](#Installation)
* [Usage](#Usage)
* [Contributing](#Contributing)
* [License](#License)
</details>

## Dependencies
- Anytype
- Docker and docker-compose
- Account in [matrix.org](https://matrix.org) (or self-hosted matrix)
- [Socat](https://github.com/3ndG4me/socat)
- Systemd (or write your own realisation of [socat.sh](./scripts/socat.sh) for socat autostart on boot)

## Installation
```sh
git clone git@github.com:aragami3070/anytype-notify.git
cd anytype-notify
```

## Usage
- Create .env file: 
```sh
cp .env.example .env
```
- Fill .env with your actual data
- Specify how often to check the new Anytype objects and what type do the objects with anytype-to-matrix ID mappings have in [config.toml](./config.toml)
- Deploy container with docker-compose.yaml:
```sh
sudo docker-compose up --no-start 
```
- Run container:
```sh
sudo docker start anytype-notifier
```

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[License](License)
