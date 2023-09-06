# `notifier`

> *A simple API to send messages to your Dashboard*

## Routes

+ `GET` - `/` - Get the current general message
+ `GET` - `/<channel>` - Get the current message in the channel
+ `GET` - `/health` - Get the health of the service
+ `POST` - `/` - Send a message to the general channel
+ `POST` - `/<channel>` - Send a message to a specific channel

## Message Format

```jsonc
{
  "message": "Hello World!",
  "title": "Hello",
  "type": "info" // can be info, success, warning, or error
}
```
