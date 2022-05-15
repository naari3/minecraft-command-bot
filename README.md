# minecraft-command-bot

A discord bot for Minecraft server.

# requirement

- Minecraft Server
  - RCON is required
- Discord Server
  - A role named `cmd` is required
- `logs/latest.log`
  - for log transfer

# how to use

```yaml
# example: docker-comopse.yaml
version: "3.8"

services:
  mc:
    ports:
      - "25565:25565"
      - "25575:25575"
    volumes:
      - "mc:/data"
    environment:
      EULA: "TRUE"
    image: itzg/minecraft-server
    restart: always
  bot:
    image: ghcr.io/naari3/minecraft-command-bot:latest
    environment:
      RCON_HOST: mc
      RCON_PASSWORD: minecraft
      DISCORD_BOT_TOKEN: your_token
      DISCORD_BOT_PREFIX: \
      MINECRAFT_LOG_PATH: /data/logs/latest.log
      MINECRAFT_LOG_CHANNEL_ID: your_channel_id
    volumes:
      - "mc:/data"

volumes:
  mc:
```

# features

## commands

### cmd

- Execute command.
- Example:
  - `\cmd kill @e`
- Can only be executed by those with the `cmd` role.

### mcuuid

- Convert Minecraft name and UUID to each other.
- Example:
  - `\mcuuid naarisan` will return => `05140bb4-f432-43fe-a5e4-069da2d4fc46`
  - `\mcuuid 05140bb4-f432-43fe-a5e4-069da2d4fc46` will return => `naarisan`
  - `\mcuuid 05140bb4f43243fea5e4069da2d4fc46` will also return => `naarisan`

### say

- Send chat to Minecraft server.
- Example:
  - `\say howdy!`

## log transfer

- Transfer chats in the Minecraft server to a Discord channel.

## add to whitelist via heart reaction

- When a user adds a heart reaction to a user listed message, the user is added to the server's whitelist.
