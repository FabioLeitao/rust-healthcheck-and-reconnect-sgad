extern crate oracle;

use oracle::Connection;
use std::env;
use std::process;

fn require_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        eprintln!(
            "Missing environment variable {name}. See README.md and .env.example."
        );
        process::exit(1);
    })
}

fn oracle_password() -> String {
    env::var("ORACLE_PASSWORD").unwrap_or_else(|_| {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!(
                "Set ORACLE_PASSWORD or pass the database password as the first CLI argument."
            );
            process::exit(1);
        }
        args[1].clone()
    })
}

fn cosetting_row_id() -> u32 {
    let raw = env::var("COSETTING_ROW_ID").unwrap_or_else(|_| "681".to_string());
    raw.parse::<u32>().unwrap_or_else(|_| {
        eprintln!("COSETTING_ROW_ID must be a positive integer.");
        process::exit(1);
    })
}

fn main() {
    let user = require_env("ORACLE_USER");
    let connect = require_env("ORACLE_CONNECT_STRING");
    let db_passwd = oracle_password();
    let row_id = cosetting_row_id();

    let conn =
        Connection::connect(&user, &db_passwd, &connect).expect("Oracle connection failed");

    let select_sql = format!(
        "SELECT ID, KEY, VALUE, UPDATED FROM CORE.cosetting WHERE id = {}",
        row_id
    );
    let rows = conn.query(&select_sql, &[]).expect("query failed");
    for r in rows {
        let row = r.expect("row");
        let id: String = row.get("ID").expect("ID");
        let key: String = row.get("KEY").expect("KEY");
        let value: String = row.get("VALUE").expect("VALUE");
        let updated: String = row.get("UPDATED").expect("UPDATED");
        if value == "TRUE" {
            println!("{} {} {} {}", id, key, value, updated);
        } else {
            println!("{} {} {} {}", id, key, value, updated);
            let update_sql = format!(
                "UPDATE CORE.cosetting SET VALUE = 'TRUE' WHERE id = {}",
                row_id
            );
            conn.execute(&update_sql, &[]).expect("update failed");
            conn.commit().expect("commit failed");
        }
    }
}
