use log::info;
use std::collections::HashMap;

use crate::constants;

pub fn create_keyspace(key: &str) -> String {
    info!("creating keyspace {}...", key);
    format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class':'{}', 'replication_factor':'{}'}};", 
        key,
        constants::REPLICATION_STRATEGY_DB,
        constants::REPLICATION_FACTOR_DB
    )
}

pub fn create_table(key: &str, table: &str, columns: &str, pk: &str) -> String {
    info!("creating table {} in keyspace {}...", table, key);
    format!(
        "CREATE TABLE IF NOT EXISTS {}.{} ({}, PRIMARY KEY ({}));",
        key,
        table,
        columns,
        pk,
    )
}

pub fn create_index(key: &str, table: &str, column: &str) -> String {
    info!("creating index {} for table {} in keyspace {}...", column, table, key);
    format!(
        "CREATE INDEX ON {}.{}({});",
        key,
        table,
        column
    )
}

pub fn insert_into_table(key: &str, table: &str, msg: HashMap<&str, String>, extra: &str) -> String {
    let mut columns = vec![];
    let mut values = vec![];
    for  (k, v) in msg.iter() {
        columns.push(k.to_string());
        values.push(v.to_string());
    }
    format!(
        "insert into {}.{} ({}) values ({}) {};",
        key,
        table,
        columns.join(", "),
        format!("'{}'", values.join("', '")),
        extra
    )
}

pub fn insert_into_table_with_num(key: &str, table: &str, msg: HashMap<&str, String>, msg2: HashMap<&str, String>, extra: &str) -> String {
    let mut columns = vec![];
    let mut values = vec![];
    for  (k, v) in msg.iter() {
        columns.push(k.to_string());
        values.push(v.to_string());
    }
    let mut columns2 = vec![];
    let mut values2 = vec![];
    for  (k, v) in msg2.iter() {
        columns2.push(k.to_string());
        values2.push(v.to_string());
    }
    format!(
        "insert into {}.{} ({}, {}) values ({}, {}) {};",
        key,
        table,
        columns.join(", "),
        columns2.join(", "),
        format!("'{}'", values.join("', '")),
        format!("{}", values2.join(", ")),
        extra
    )
}

pub fn delete_from_table(key: &str, table: &str, clause: String) -> String {
    format!(
        "delete from {}.{} where {};",
        key,
        table,
        clause
    )
}

pub fn get_from_table(key: &str, table: &str, select: &str, clause: String) -> String {
    if clause.is_empty() {
        format!(
            "SELECT {} FROM {}.{};",
            select,
            key,
            table
        )
    } else {
        format!(
            "SELECT {} FROM {}.{} WHERE {};",
            select,
            key,
            table,
            clause
        )
    }
}

pub fn update_table(key: &str, table: &str, new_value: String, clause: String) -> String {
    format!(
        "update {}.{} set {} where {};",
        key,
        table,
        new_value,
        clause
    )
}
