mod file;

use std::{fs, path::Path};
use rusqlite::{Connection, Result as SqlResult};
use serde_json::Value;
use crate::file::get_json_files;

fn main(){
    let conn = Connection::open("data.db").expect("无法打开数据库");

    let data_dir = "./data";
    let json_files = get_json_files(data_dir);
    let json_files = match json_files {
        Ok(files) => files,
        Err(e) => {
            println!("获取JSON文件失败: {:?}", e);
            return;
        }
    };

    for file_path in json_files {
        if let Err(e) = process_json_file(&conn, &file_path) {
            println!("处理文件 {} 失败: {}", file_path, e);
            return;
        }
    };
}


fn process_json_file(conn: &Connection, file_path: &str) -> SqlResult<()> {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("无法读取文件: {}", file_path);
            return Err(rusqlite::Error::InvalidQuery);
        }
    };

    let json: Value = match serde_json::from_str(&content) {
        Ok(json) => json,
        Err(_) => {
            println!("无法解析JSON: {}", file_path);
            return Err(rusqlite::Error::InvalidQuery);
        }
    };

    let file_name = Path::new(file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let table_name = normalize_table_name(file_name);

    conn.execute(
        &format!("CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY AUTOINCREMENT, text TEXT)", table_name),
        [],
    )?;

    let mut stmt = conn.prepare(&format!("INSERT INTO {} (text) VALUES (?1)", table_name))?;

    for item in json.as_array().unwrap() {
        if let Some(text) = item.as_str() {
            stmt.execute([text])?;
        }
    }
    println!("已处理文件: {}，插入表: {}", file_path, table_name);
    Ok(())
}

fn normalize_table_name(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() > 1 && parts.last().unwrap().parse::<i32>().is_ok() {
        parts[..parts.len()-1].join("_")
    } else {
        name.to_string()
    }
}
