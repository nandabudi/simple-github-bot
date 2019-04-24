# Simple Github Bot
A simple script to listen on github webhook and use telegram bot to deliver the payload.

### Foreword
I created this simple script to help me at my work. Currently this script will report pull request event (more about [github webhook events](https://developer.github.com/webhooks/#events)). But it'll also report every any events regardless. It just the message won't be as good. PR is always welcomed.

## Getting Started
You need to have those followings.
1. [Zeit](https://zeit.co/now) now account.
2. [Telegram bot](https://core.telegram.org/bots) access token.
3. Target chat ID. (related [stack overflow](https) question).
4. A github repository. ([duh](https://github.com/alvinmatias69))

## How To Use
1. Modify `now.json.example` files. Change the token and chat id with your own.
2. Save as `now.json`.
3. Upload to now server
  ```sh
  $ now
  ```
4. On finished, your terminal will print your serverless domain. Keep it.
5. Open your github repository
6. Setup a [webhook](https://developer.github.com/webhooks)
7. Set your serverless domain as payload url.
