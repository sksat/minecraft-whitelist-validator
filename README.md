# minecraft-whitelist-validator
Minecraft whitelist.json validator

![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
[![CI](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/ci.yml/badge.svg)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/ci.yml)
[![Build Image](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-image.yml/badge.svg?branch=main)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-image.yml)
[![Image Size](https://img.shields.io/docker/image-size/sksat/minecraft-whitelist-validator/main)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-image.yml)
[![Build single-binary](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-musl.yml/badge.svg)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-musl.yml)

## GitHub Actions

- Example workflow

```yaml
on:
  push:
    paths:
      - 'whitelist.json'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: sksat/minecraft-whitelist-validator@v0.2.0
```

## Inputs

| Name   | Required | Description                        | Type   | Default        |
| -------| :------: | -----------------------------------| ------ | -------------- |
| `json` | ✓        | Minecraft whitelist.json file path | string | whitelist.json |

## Docker

- Example oneliner

```sh
# bash
docker run --rm -it -v "$(pwd)"/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json

# fish
docker run --rm -it -v (pwd)/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json
```
