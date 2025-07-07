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

### 📦 1. Build the App (One Time)

Make sure you have Rust installed.  
Then run this in the root of the project:

```bash
cargo build --release
```

This creates the app binary at:

```bash
./target/release/iNuke
```

---

### 💥 2. Run the Export

To extract your messages, you need two things:

1. The path to your `chat.db` file  
2. The path where you want the CSV exported

Here’s the full command:

```bash
./target/release/iNuke /full/path/to/chat.db /full/path/to/output.csv
```

🔧 **Example for macOS:**

```bash
./target/release/iNuke /Users/yourname/Documents/chat/chat.db /Users/yourname/Documents/messages_full.csv
```

⚠️ **Make sure:**
- Your `.db` path is correct (no typos, full path only — no `~`)
- The file exists and is not locked
- You have permission to read/write in the destination folder

---

### 💻 Pro Tips

- You can test with copies of the database if needed
- If you're on Windows, use `\\` double-backslashes for paths
- Want to verify the output? Open the CSV with Excel, Numbers, or any text editor

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

## ⚖️ License

This tool is **free for personal use only**.  
See [LICENSE](./LICENSE) for full terms — no commercial forking, selling, bundling, or integration allowed without written consent.

---

Built with ☠️ by BuiltByWill.  
If this repo disappears, you were too slow.