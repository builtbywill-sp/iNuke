# 🧨 iNuke

iNuke is a terminal-first iMessage extractor for serious users.  
Pulls *everything* from `chat.db`, offline, in seconds. No cloud. No license key. No bullshit.
![downloads](https://img.shields.io/github/downloads/builtbywill-sp/iNuke/total)
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
- You can pipe output directly into another script or CLI tool for batch processing

---

### 🍎 macOS Gotchas

Because Apple hates unsigned binaries, you might see a scary “can’t verify” message.

To fix it:
```bash
chmod +x ~/Downloads/iNuke-mac
xattr -d com.apple.quarantine ~/Downloads/iNuke-mac
```

Then run like this:
```bash
~/Downloads/iNuke-mac ~/Downloads/chat.db 1234567890 ~/Downloads/messages.csv
```

Or, if you’re inside the Downloads folder:
```bash
cd ~/Downloads
./iNuke-mac chat.db 1234567890 messages.csv
```

✅ Make sure:
- Your `chat.db` file is in the same folder
- You include the target phone number (or leave it out to dump all messages)
- The output path is writable (Desktop, Downloads, etc.)

---

## 📤 Output Format

| ROWID   | message_date         | is_from_me | text                       | date_raw       |
|---------|----------------------|------------|----------------------------|----------------|
| 1234345 | 2024-04-19 17:02:33  | 1          | Yo can you send the file? | 98765432109876 |
| 4567890 | 2024-04-20 08:14:12  | 0          | Yeah one sec               | 87654321098765 |

---

## 🧪 Tested On

- macOS Sonoma
- SQLite 3.45+
- `chat.db` pulled from `/private/var/mobile/Library/SMS/chat.db`
- Works with .db files pulled using iExplorer, iMazing, or full iOS backups
- Tested on Intel and Apple Silicon Macs
- Compatible with Terminal, iTerm2, and VS Code terminal

---

## ⚖️ License

This tool is **free for personal use only**.  
See [LICENSE](./LICENSE) for full terms — no commercial forking, selling, bundling, or integration allowed without written consent.

---

Built with ☠️ by BuiltByWill.  
If this repo disappears, you were too slow.

---
🧠 Need help or want to contribute? File an issue or fork the repo. BuiltByWill is watching.
