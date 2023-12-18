# RustyReaderBot
Simple independent (without database) Telegram Bot for RSS feeds.<br>
This simple bot is made for send update notifications on Telegram channel from differents RSS feeds.<br>

# Setup
## Deamon setup
1. [Install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

2. Install dependencies
```bash
sudo apt install build-essential
sudo apt install librust-openssl-dev
```

3. Download project
```bash
git clone https://github.com/RevolioClockberg/RustyReaderBot.git
``` 

4. Build project
```bash
cd 
cargo build --release
```

# Todo
* Command to add/delete/modify RSS feeds.
* Command to reinitialize dates and receive all last posts already send on Channel.
* Manage multiple channels.
* Docker.

# Based on
* [easy-rss](https://docs.rs/easy_rss/1.0.1/easy_rss/index.html)
* [tbot](https://crates.io/crates/tbot)