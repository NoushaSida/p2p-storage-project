use cassandra_cpp::{AsRustType, CassResult};
use cron::Schedule;
use chrono::Utc;
use std::{collections::HashMap, str::FromStr};

use crate::{constants, database::{get_from_db, write_into_db}, query::{get_from_table, insert_into_table, insert_into_table_with_num}};
use constants::CRON_JOB_RATING;

pub async fn cron_job(session: &cassandra_cpp::Session) {
    let schedule = Schedule::from_str(CRON_JOB_RATING).unwrap();
    for datetime in schedule.upcoming(Utc) {
        println!("-> fired cron job: {}", datetime);
        update_ratings(session).await;
    }
}

async fn update_ratings(db_session: &cassandra_cpp::Session) {
    /*let query_table_piece_get = get_from_table(
        "peer",
        "performance",
        "*",
        "group by peer_id".to_string());
    println!("Query: {}", query_table_piece_get);
    let stmt_get_from_piece: CassResult = get_from_db(db_session.clone(), query_table_piece_get).await;
    println!("Statement {:?}", stmt_get_from_piece.iter());
    for row in stmt_get_from_piece.iter() {
        let peer_id = row.get_column_by_name("peer_id").map(|v|v.to_string()).unwrap();
        let uptime_start = row.get_column_by_name("uptime_start").map(|v|v.to_string()).unwrap();
        let disk_read = row.get_column_by_name("disk_read").map(|v|v.to_string()).unwrap();
        let disk_write = row.get_column_by_name("disk_write").map(|v|v.to_string()).unwrap();
        let throughput = row.get_column_by_name("throughput").map(|v|v.to_string()).unwrap();
        let uptime_end = row.get_column_by_name("uptime_end").map(|v|v.to_string()).unwrap();

        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("peer_id", peer_id);
        msg_map.insert("uptime_start", uptime_start);
        msg_map.insert("disk_read", disk_read);
        msg_map.insert("disk_write", disk_write);
        msg_map.insert("throughput", throughput);
        msg_map.insert("uptime_end", uptime_end);
        let query_insert_peer_perf_history = insert_into_table(
            "peer",
            "performance_history",
            msg_map,
            ""
        );
        println!("Query: {}", query_insert_peer_perf_history);
        let _ = write_into_db(db_session.clone(), "no_selector".to_string(), query_insert_peer_perf_history, "").await;
    
        

    }*/


    todo!("update ratings");
}
