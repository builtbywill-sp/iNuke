use rusqlite::{Connection, Result};
use csv::Writer;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        eprintln!("❌ Usage: ./iNuke <path/to/chat.db> [<TARGET_NUMBER>] <path/to/output.csv>");
        std::process::exit(1);
    }

    let db_path = &args[1];
    let (target_number, output_path) = if args.len() == 4 {
        (Some(format!("%{}%", &args[2])), &args[3])
    } else {
        (None, &args[2])
    };

    println!("🚀 iNuke CLI activated.");
    println!("📂 Reading DB: {}", db_path);
    if let Some(ref _t) = target_number {
        println!("🎯 Target Number: {}", &args[2]);
    }
    println!("💾 Writing CSV to: {}", output_path);

    // Connect to the DB
    let snapshot_path = "/tmp/chat_snapshot.db";
    fs::copy(db_path, snapshot_path)?;
    let conn = Connection::open(snapshot_path)?;

    // Check if the 'message' table exists
    let table_check: Result<String, _> = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='message';",
        [],
        |row| row.get(0),
    );

    if table_check.is_err() {
        eprintln!("❌ Error: 'message' table not found in the database.");
        std::process::exit(1);
    }

    // Force WAL checkpoint flush
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_checkpoint(FULL);")?;

    let query = "
        SELECT
            message.ROWID,
            datetime(message.date / 1000000000 + strftime('%s','2001-01-01'), 'unixepoch') AS message_date,
            message.is_from_me,
            message.text,
            handle.id AS sender
        FROM
            message
        LEFT JOIN handle ON message.handle_id = handle.ROWID
        ORDER BY message.date ASC
    ".to_string();

    // Prepare statement and CSV writer
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    let mut wtr = Writer::from_path(output_path)?;

    // Write CSV header
    wtr.write_record(&[
        "ROWID",
        "message_date",
        "is_from_me",
        "text",
        "sender"
    ])?;

    // Write each row to CSV
    while let Some(row) = rows.next()? {
        let rowid: i64 = row.get::<_, Option<i64>>(0)?.unwrap_or(0);
        let date: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let is_from_me: i64 = row.get::<_, Option<i64>>(2)?.unwrap_or(0);
        let text: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let sender: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();

        wtr.write_record(&[
            rowid.to_string(),
            date,
            is_from_me.to_string(),
            text,
            sender,
        ])?;
    }

    wtr.flush()?;

    println!("✅ Export complete.");
    Ok(())
}