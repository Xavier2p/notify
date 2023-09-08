<div align="center">
  <h1>
    <code>notify</code>
  </h1>
  <div>
    <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/xavier2p/notify?style=for-the-badge&logo=rust&color=orange">
  </div>
  <div>
    <img src="https://img.shields.io/github/license/xavier2p/notify?logo=github&style=for-the-badge" />
    <a href="https://xavier2p.github.io/notify">
      <img src="https://img.shields.io/website?down_color=critical&down_message=DOWN&label=Documentation&logo=github&style=for-the-badge&up_color=success&up_message=UP&url=https%3A%2F%2Fxavier2p.github.io%2Fnotify" />
    </a>
  </div>
</div>

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
  "style": "info" // can be `info`, `success`, `warning` or `danger`
}
```

## Deployment

### Local build

```bash
# Clone repo
git clone https://github.com/Xavier2p/notify.git && cd notify

# Build the image
docker build -t notifier .

# Run the image
docker run -d --rm -p 8080:8000 --name notifier notifier
```

### From GHCR

```bash
docker run -d --rm -p 8080:8000 --name notifier ghcr.io/xavier2p/notify:latest
```

## License - MIT

> Copyright (c) 2023 Xavier2p

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
