use cassandra_cpp::{Session, CassResult};
use log::{debug, error, log_enabled, info, Level, warn};
use zenoh::{sample::Sample, key_expr::KeyExpr};
use super::query::{create_table, create_keyspace, create_index};

pub async fn create_db_schema(session: &cassandra_cpp::Session) {
    // Create keyspaces
    info!("Creating Cassandra keyspaces...");
    let _statement_key_user = session.statement(create_keyspace("user")).execute().await;
    let _statement_key_file = session.statement(create_keyspace("file")).execute().await;
    let _statement_key_peer = session.statement(create_keyspace("peer")).execute().await;
    
    // Create tables
    info!("Creating Cassandra tables...");
    let _statement_table_user = session.
        statement(create_table(
            "user",
            "user",
            "username text, name text, surname text, password text, email text, registration_date text, salt text",
            "username"))
        .execute().await;

    let _statement_table_permission = session.
        statement(create_table(
            "file",
            "permission",
            "username text, file_id text, owner text, write text",
            "username, file_id"))
        .execute().await;

    let _statement_table_file = session.
        statement(create_table(
            "file",
            "file",
            "file_id text, username text, file_name text, file_size text, file_size_compressed text, file_type text, file_status text, upload_date text",
            "username, file_id"))
        .execute().await;

    let _statement_table_piece = session.
        statement(create_table(
            "file",
            "piece",
            "file_id text, piece_order text, piece_size text, chunk_num text, replication_num text, chunk_peer text, chunk_hash text, transfer_length text, symbol_size text, source_blocks text, sub_blocks text, symbol_alignment text",
            "file_id, piece_order"))
        .execute().await;

    let _statement_table_peer = session.
        statement( create_table(
            "peer",
            "peer",
            "peer_id text, username text, device_name text, country text, disk_size text, mount_point text, registration_date text",
            "username, device_name"))
        .execute().await;

    let _statement_table_peer_selection = session.
        statement( create_table(
            "peer",
            "selection",
            "peer_id text, disk_available bigint, last_liveness text, ranking text, country text, mount_point text, pieces text",
            "ranking, disk_available, peer_id"))
        .execute().await;

    let _statement_table_peer_penalty = session.
        statement( create_table(
            "peer",
            "restriction",
            "peer_id text, penalty text, last_penalty text, ban text, last_ban text",
            "peer_id"))
        .execute().await;

    let _statement_table_peer_performance = session.
        statement(create_table(
            "peer",
            "performance",
            "peer_id text, uptime_start text, uptime_end text, disk_read text, disk_write text, throughput text",
            "peer_id, uptime_start"))
        .execute().await;

    let _statement_table_peer_performance_history = session.
        statement(create_table(
            "peer",
            "performance_history",
            "peer_id text, uptime_start text, uptime_end text, disk_read text, disk_write text, throughput text",
            "peer_id, uptime_start"))
        .execute().await;

    let _statement_table_peer_liveness = session.
        statement(create_table(
            "peer",
            "liveness",
            "peer_id text, timestamp text, available text",
            "peer_id"))
        .execute().await;

    // Create indexes
    info!("Creating Cassandra indexes...");
    //let _statement_index_user = session.
    //    statement(create_index("user", "user", "password")).execute().await;
   // let _statement_index_permission = session.
   //     statement(create_index("file", "permission", "username")).execute().await;
    //let _statement_index_file = session.
    //    statement(create_index("file", "file", "username")).execute().await;
    //let _statement_index_file = session.
    //    statement(create_index("file", "file", "file_id")).execute().await;
    //let _statement_index_peer = session.
    //    statement(create_index("peer", "peer", "username")).execute().await;
    let _statement_index_peer_selection_liveness = session.
        statement(create_index("peer", "selection", "last_liveness")).execute().await;
    //let _statement_index_peer_selection_ranking = session.
    //    statement(create_index("peer", "selection", "ranking")).execute().await;
    let _statement_index_peer_id = session.
        statement(create_index("peer", "selection", "peer_id")).execute().await;
    let _statement_index_country = session.
        statement(create_index("peer", "selection", "country")).execute().await;


    info!("Created Cassandra schema.");
}

pub async fn write_into_db(db_session: Session, selector: String, query: String, not_exists: &str) -> Result<Sample, zenoh::prelude::Value> {
    info!("Query: {}", query);
    let res: String = match db_session.
            statement(query)
            .execute().await {
        Ok(row) => {
            match row.first_row() {
                Some(a) => a.get_column_by_name("[applied]").unwrap().to_string(),
                None => {
                    if not_exists == "" {
                        info!("Writing to DB: {}...", selector);
                        String::from("true")
                    } else {
                        warn!("Not writing anything.");
                        String::from("false")
                    }
                },
            }
        },
        Err(_) => {
            info!("Failed to insert.");
            String::from("false")
        },
    };
    info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(res.as_str()));
    info!("Selector: {}", selector);
    info!("KeyExpr: {}", KeyExpr::new(selector.clone()).unwrap());
    Ok(Sample::new(KeyExpr::new(selector).unwrap(), res))
}

pub async fn get_from_db(db_session: Session, query: String) -> CassResult{
    let stmt_insert_into_table: CassResult = db_session.
        statement(query)
        .execute()
        .await
        .expect("Failed to insert user.");
    info!("stmt_insert_into_table {:?}", stmt_insert_into_table);
    stmt_insert_into_table
}