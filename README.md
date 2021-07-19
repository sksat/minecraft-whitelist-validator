# minecraft-whitelist-validator
Minecraft whitelist.json validator

```sh
# bash
docker run --rm -it -v "$(pwd)"/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json

# fish
docker run --rm -it -v (pwd)/whitelist.json:/app/whitelist.json sksat/minecraft-whitelist-validator /app/minecraft-whitelist-validator whitelist.json
```
