# Factorio <-> Telegram Bridge

This bridge uses RCON to forward messages from Telegram to Factorio and Factorio's console log file to forward messages from Factorio to Telegram

## Usage
To use the program, you need to set some required options/environment variables:
```
Option                        Env var                 Description
-t, --telegram-token          TELEGRAM_TOKEN          Telegram bot token
-c, --telegram-chat-id        TELEGRAM_CHAT_ID        Telegram chat id
-h, --factorio-rcon-host      FACTORIO_RCON_HOST      Factorio RCON host
-p, --factorio-rcon-password  FACTORIO_RCON_PASSWORD  Factorio RCON password
-l, --factorio-log-file       FACTORIO_LOG_FILE       Factorio console log file
```

### Factorio Friday Facts 

When `--fff file_path` option is set, the bridge will check for new FFFs and send them to both chats. The file contains the latest title of the Factorio Friday Facts post in order to send the info about the FFF only once. 

```
Option  Env var     Description
--fff   FFF         Factorio Friday Facts Cache File. If set, the bridge will check for new FFFs and send them to the chat [env: FFF=]
```
