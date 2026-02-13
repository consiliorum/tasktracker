use rusqlite::{Connection, params};
use std::path::PathBuf;

use crate::models::{Priority, Task};

fn db_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".tasktracker.db");
    path
}

pub fn open() -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path())?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            description TEXT,
            priority    TEXT NOT NULL DEFAULT 'medium',
            due_date    TEXT,
            done        INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT NOT NULL
        );",
    )?;
    Ok(conn)
}

pub fn add_task(
    conn: &Connection,
    title: &str,
    description: Option<&str>,
    priority: &Priority,
    due_date: Option<&str>,
) -> rusqlite::Result<i64> {
    let now = chrono::Local::now().to_rfc3339();
    conn.execute(
        "INSERT INTO tasks (title, description, priority, due_date, done, created_at)
         VALUES (?1, ?2, ?3, ?4, 0, ?5)",
        params![title, description, priority.to_string(), due_date, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_tasks(conn: &Connection, include_done: bool) -> rusqlite::Result<Vec<Task>> {
    let sql = if include_done {
        "SELECT id, title, description, priority, due_date, done, created_at FROM tasks ORDER BY id"
    } else {
        "SELECT id, title, description, priority, due_date, done, created_at FROM tasks WHERE done = 0 ORDER BY id"
    };
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([], |row| {
        let priority_str: String = row.get(3)?;
        let done_int: i32 = row.get(5)?;
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            priority: priority_str.parse().unwrap_or(Priority::Medium),
            due_date: row.get(4)?,
            done: done_int != 0,
            created_at: row.get(6)?,
        })
    })?;
    rows.collect()
}

pub fn mark_done(conn: &Connection, id: i64) -> rusqlite::Result<bool> {
    let changed = conn.execute("UPDATE tasks SET done = 1 WHERE id = ?1", params![id])?;
    Ok(changed > 0)
}

pub fn remove_task(conn: &Connection, id: i64) -> rusqlite::Result<bool> {
    let changed = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(changed > 0)
}

pub fn edit_task(
    conn: &Connection,
    id: i64,
    title: Option<&str>,
    description: Option<&str>,
    priority: Option<&Priority>,
    due_date: Option<&str>,
) -> rusqlite::Result<bool> {
    let mut sets = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(t) = title {
        sets.push("title = ?");
        values.push(Box::new(t.to_string()));
    }
    if let Some(d) = description {
        sets.push("description = ?");
        values.push(Box::new(d.to_string()));
    }
    if let Some(p) = priority {
        sets.push("priority = ?");
        values.push(Box::new(p.to_string()));
    }
    if let Some(d) = due_date {
        sets.push("due_date = ?");
        values.push(Box::new(d.to_string()));
    }

    if sets.is_empty() {
        return Ok(false);
    }

    let sql = format!("UPDATE tasks SET {} WHERE id = ?", sets.join(", "));
    values.push(Box::new(id));

    let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let changed = conn.execute(&sql, params.as_slice())?;
    Ok(changed > 0)
}
