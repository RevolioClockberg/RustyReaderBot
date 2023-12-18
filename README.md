# RustyReaderBot
Simple independent (without database) Telegram Bot for RSS feeds.<br>
This simple bot is made for send update notifications on Telegram channel from differents RSS feeds.<br>

# Setup
1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

2. Install dependencies
```bash
sudo apt install build-essential
sudo apt install librust-openssl-dev
```

3. Download project
```bash
cd /var/www/
git clone https://github.com/RevolioClockberg/RustyReaderBot.git
``` 

4. Create needed files
```bash
cd RustyReaderBot
mkdir files && touch files/logs.txt && touch files/list.json
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
* Make it run inn docker container.

# Based on
* [easy-rss](https://docs.rs/easy_rss/1.0.1/easy_rss/index.html)
* [tbot](https://crates.io/crates/tbot)