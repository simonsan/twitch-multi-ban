# Multi Ban for Twitch

Command line tool to ban alt accounts on Twitch

## Setup twitch authentication

You will need to set the following environment variables:

- TWITCH_SEND_TOKEN: can be generated at [twitchapps.com/tmi](https://twitchapps.com/tmi).
- TWITCH_SEND_NAME: is your twitch name
- TWITCH_SEND_CHANNEL: the twitch channel to join

## Setup ban list

Ban list schema:

```yaml
--- # entry
- date: 11/20/2031
  comment: Update to username
  content:
    - username123
    - username456
```

You will need to set the following environment variables:

- TWITCH_BAN_LIST: URL to RAW file of account list

## Download

You can download binary files here: <https://github.com/simonsan/twitch-multi-ban/releases>

## Run

```bash
twitch-multi-ban <reason>
```
