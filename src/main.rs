extern crate oracle;

use oracle::Connection;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let db_passwd = &args[1];
    let conn = Connection::connect("tosp", db_passwd, "10.129.48.68:1521/tosprd").unwrap();
    {
        /*
        let (version, banner) = conn.server_version().unwrap();
        println!("Oracle Version: {}", version);
        println!("--- Version Banner ---");
        println!("{}", banner);
        println!("---------------|---------------|---------------|");
        */
        let select_sql = "SELECT ID, KEY, VALUE, UPDATED FROM CORE.cosetting WHERE id = 681";
        let rows = conn.query(select_sql, &[]).unwrap();
        for r in rows {
            let row = r.unwrap();
            let id: String = row.get("ID").unwrap();
            let key: String = row.get("KEY").unwrap();
            let value: String = row.get("VALUE").unwrap();
            let updated: String = row.get("UPDATED").unwrap();
            if value == "TRUE" {
                println!("{} {} {} {}", id, key, value, updated);
            } else {
                println!("{} {} {} {}", id, key, value, updated);
                let update_sql = "UPDATE CORE.cosetting SET VALUE = 'TRUE' WHERE id = 681";
                conn.execute(update_sql, &[]).unwrap();
                conn.commit().unwrap();
            }
        }
    }
}
