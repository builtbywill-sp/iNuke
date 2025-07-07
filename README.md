# 🧨 iNuke

iNuke is a terminal-first iMessage extractor for serious users.  
Pulls *everything* from `chat.db`, offline, in seconds. No cloud. No license key. No bullshit.

---

## ⚙️ Features

- 🔍 Extracts full message history from Apple’s iMessage `chat.db` files
- 💾 Converts messages into clean CSVs for legal, forensic, or personal use
- 🚀 100% offline — nothing leaves your machine
- 🦀 Built with Rust for speed, precision, and safety

---

## 🧨 Usage

### 📦 Build

```bash
cargo build --release
```

### 💥 Run

```bash
./target/release/iNuke /absolute/path/to/chat.db /absolute/path/to/output.csv
```

> Example:

```bash
./target/release/iNuke /Users/builtbybrown/Documents/chat/chat.db /Users/builtbybrown/Documents/messages_full.csv
```

---

## 📤 Output Format

| ROWID | message_date        | is_from_me | text     | date_raw     |
|-------|---------------------|------------|----------|--------------|
| 84016 | 2023-12-09 03:24:43 | 1          | Awwww    | 17207677209  |

---

## 🧪 Tested On

- macOS Sonoma
- SQLite 3.45+
- `chat.db` pulled from `/private/var/mobile/Library/SMS/chat.db`

---

## 📌 Roadmap

- [ ] Attachments
- [ ] Phone number filters
- [ ] Timestamp formatting
- [ ] JSON mode for Airtable import

---

## ⚖️ License

This tool is **free for personal use only**.  
See [LICENSE](./LICENSE) for full terms — no commercial forking, selling, bundling, or integration allowed without written consent.

---

Built with ☠️ by BuiltByWill.  
If this repo disappears, you were too slow.