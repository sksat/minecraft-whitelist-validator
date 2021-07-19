# minecraft-whitelist-validator
Minecraft whitelist.json validator

[![CI](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/ci.yml/badge.svg)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/ci.yml)
[![Build Image](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-image.yml/badge.svg?branch=main)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-image.yml)
[![Build single-binary](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-musl.yml/badge.svg)](https://github.com/sksat/minecraft-whitelist-validator/actions/workflows/build-musl.yml)

```sh
# bash
docker run --rm -it -v "$(pwd)"/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json

# fish
docker run --rm -it -v (pwd)/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json
```
