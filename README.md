# Factorio <-> Telegram Bridge

This bridge uses RCON to forward messages from Telegram to Factorio and Factorio's console log file to forward messages from Factorio to Telegram

## Usage

```
Usage: cracktorio-bot [OPTIONS] -t <TELEGRAM_TOKEN> -c <TELEGRAM_CHAT_ID> -p <FACTORIO_RCON_PASSWORD> <-l <FACTORIO_LOG_FILE>|-n <FACTORIO_KUBE_NAMESPACE>>
```

Below is a full list of options and environment variables
```
Option                        Env var                 Description
-s, --state-file-path         STATE_FILE_PATH         Path of state file that contains data shared between restarts
-t, --telegram-token          TELEGRAM_TOKEN          Telegram bot token
-c, --telegram-chat-id        TELEGRAM_CHAT_ID        Telegram chat id
-h, --factorio-rcon-host      FACTORIO_RCON_HOST      Factorio RCON host
-p, --factorio-rcon-password  FACTORIO_RCON_PASSWORD  Factorio RCON password
-l, --factorio-log-file       FACTORIO_LOG_FILE       Factorio console log file
-n, --factorio-kube-namespace FACTORIO_KUBE_NAMESPACE Factorio Kubernetes Namespace
-L, --factorio-kube-labels    FACTORIO_KUBE_LABELS    Factorio Kubernetes pod label filter
-e, --experimental            FACTORIO_EXPERIMENTAL   Use Experimental factorio version
    --fff                     FACTORIO_FRIDAY_FACTS   If set, the bridge will check for new Factorio Friday Facts and send them to the chat
-h, --help                                            Print help
```
