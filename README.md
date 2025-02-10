# RustyReaderBot
Simple independent (and without database) Telegram Bot for RSS feeds.<br>
This is a simple [tbot](https://crates.io/crates/tbot) for sending update notifications on Telegram channel from differents RSS feeds.<br>

# Setup
1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

2. Install dependencies
```bash
sudo apt install -y build-essential librust-openssl-dev
```

3. Download project
```bash
cd /var/www/
git clone https://github.com/RevolioClockberg/RustyReaderBot.git
``` 

4. Create needed files
```bash
sudo mkdir /var/log/RustyReaderBot && sudo touch /var/log/RustyReaderBot/errors.log && sudo touch /var/log/RustyReaderBot/debug.log
cd RustyReaderBot
vim files/list.json
```
```json
[
  {
    "name": "Test",
    "url": "https://www.badurl.com/totesterrors.rss",
    "last_post": "test"
  }
]
```

5. Build project
```bash
cargo build --release
```

6. Setup service
```bash
sudo vim /lib/systemd/system/telegrambot.service
```
```txt
[Unit]
Description=RustyReaderBot - Bot Telegram

Wants=network.target
After=syslog.target network-online.target
 
[Service]
Type=simple
ExecStart=/var/www/RustyReaderBot/target/release/RustyReaderBot
Restart=always
RestartSec=10
TimeoutStartSec=5
KillMode=process
Environment="RUSTY_DEBUG=FALSE"
Environment="RUSTY_BOT_LOGS=/var/log/RustyReaderBot"
Environment="TELEGRAM_BOT_TOKEN=<YOUR-BOT-TOKEN>"

[Install]
WantedBy=multi-user.target
```
```bash
sudo systemctl daemon-reload
```

7. Run
```bash
sudo systemctl start telegrambot
```


# Todo
* Delete hardcoded path.
* Command to add/delete/modify RSS feeds.
* Command to reinitialize dates and receive all last posts already send on Channel.
* Manage multiple channels.
* Make it run in docker container.

# Based on
* [easy-rss](https://docs.rs/easy_rss/1.0.1/easy_rss/index.html)
* [tbot](https://crates.io/crates/tbot)
