use base64::Engine;
use bytes::BufMut;
use cassandra_cpp::{Session, CassResult, Row, List, CassCollection, Map};
use prost::Message;
use raptorq::{Decoder, Encoder, EncoderBuilder, EncodingPacket, ObjectTransmissionInformation};
use sha2::{Sha256, Digest};
use zenoh::key_expr::KeyExpr;
use zenoh::sample::Sample;
use zenoh::{info, selector};
use zenoh::value::Value;
use std::collections::{BTreeMap, HashSet};
use std::fmt::format;
use std::{fmt::Error, collections::HashMap};
use log::{info, error, warn};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicBool;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::{MTU, PIECE_SIZE, REPLICATION_FACTOR};
use crate::protobuf::{MsgDevice, MsgFileInfo, MsgFileId, MsgLiveness, MsgMetrics, MsgPeerId, MsgPermissions, MsgUser, MsgUsername};
use crate::query::{insert_into_table_with_num, update_table};
use crate::utils::calculate_hash;
use crate::{FILE_STATUS_TO_DISTRIBUTE, FILE_STORAGE_FOLDER, KEY_EXPR_CHUNK_GET, PENALTY_CORRUPTION, PENALTY_NOT_FOUND};

use super::query::{insert_into_table, get_from_table, delete_from_table};

use uuid::Uuid;
use crate::database::{write_into_db, get_from_db};
use prost::bytes::Bytes;

pub async fn user_signup(db_session: Session, selector: String, user: MsgUser) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, user);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", user);
        Err(Value::from(user.username))
    } else {
        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("username", user.username);
        msg_map.insert("name", user.name);
        msg_map.insert("surname", user.surname);
        msg_map.insert("password", user.password);
        msg_map.insert("email", user.email);
        msg_map.insert("salt", user.salt);
        msg_map.insert("registration_date", user.registration_date);
        let query = insert_into_table("user","user", msg_map, "");
        info!("Query: {}", query);
        write_into_db(db_session, selector, query, "").await
    };
    Ok(reply.unwrap())
}

pub async fn user_login(db_session: Session, selector: String, user: MsgUsername) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, user);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", user);
        Err(Value::from(user.username))
    } else {
        let query = get_from_table(
            "user",
            "user",
            "*",
            format!("username='{}'", user.username)); 
        info!("Query: {}", query);

        let stmt_get_from_table: CassResult = get_from_db(db_session, query).await;

        let reply: String = match stmt_get_from_table.first_row() {
            Some(row) => {
                info!("User {} found.", user.username);
                let pwd: &str = &Row::get_column_by_name(&row, "password").unwrap().to_string();
                let salt: &str = &Row::get_column_by_name(&row, "salt").unwrap().to_string();
                format!("password:{},salt:{}", pwd, salt)
            },
            None => {
                info!("User {} not found.", user.username);
                format!("")
            },
        };
        info!("Reply user login: {}", reply);
        info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(reply.as_str()));
        Ok(Sample::new(KeyExpr::new(selector).unwrap(), reply.as_str()))
    };
    Ok(reply.unwrap())
}

pub async fn peer_signup(db_session: Session, selector: String, device: MsgDevice) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, device);
    let peer_id = Uuid::new_v4().to_string();

    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", device);
        Err(device.username.into())
    } else {
        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("username", device.username);
        msg_map.insert("device_name", device.device_name);
        msg_map.insert("disk_size", device.disk_size);
        msg_map.insert("mount_point", device.mount_point.clone());
        msg_map.insert("country", device.country.clone());
        msg_map.insert("registration_date", device.registration_date);
        msg_map.insert("peer_id", peer_id.clone());
        let disk_size = msg_map["disk_size"].to_string();
        info!("Arguments map: {:?}", msg_map);
        let query = insert_into_table("peer","peer", msg_map, "IF NOT EXISTS");
        let res = write_into_db(db_session.clone(), selector.clone(), query, "IF NOT EXISTS").await;
        match res.clone().unwrap().value.to_string().as_str() {
            "true" => {
                let mut msg_map_selection: HashMap<&str, String> = HashMap::new();
                msg_map_selection.insert("peer_id", peer_id.clone());
                msg_map_selection.insert("last_liveness", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
                msg_map_selection.insert("ranking", String::from("1"));
                msg_map_selection.insert("country", device.country);
                msg_map_selection.insert("mount_point", device.mount_point);
                msg_map_selection.insert("pieces", String::from("{}"));

                let mut msg_map_selection2: HashMap<&str, String> = HashMap::new();
                msg_map_selection2.insert("disk_available", String::from(disk_size));

                info!("Arguments map: {:?}", msg_map_selection);
                info!("Arguments map2: {:?}", msg_map_selection2);
                let query = insert_into_table_with_num("peer","selection", msg_map_selection.clone(), msg_map_selection2, "");
                write_into_db(db_session, selector.clone(), query, "").await
            },
            _ => {
                warn!("New device not inserted.");
                res
            }
        }
    };
    let result = match reply.clone().unwrap().value.to_string() == "true" {
        true => Ok(Sample::new(KeyExpr::new(selector).unwrap(), peer_id)),
        false => reply,
    };
    Ok(result.unwrap())
}


pub async fn peer_get(db_session: Session, selector: String, user: MsgUsername) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, user.username);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", user.username);
        Err(user.username.clone().into())
    } else {
        let query = get_from_table(
            "peer",
            "peer",
            "*",
            format!("username='{}'", user.username)); 
        info!("Query: {}", query);

        let stmt_get_from_table: CassResult = get_from_db(db_session, query).await;

        let mut results: Vec<HashMap<&str, String>> = Vec::new();
        for row in stmt_get_from_table.iter() {
            let mut record: HashMap<&str, String> = HashMap::new();
            record.insert("peer_id", row.get_column_by_name("peer_id").map(|v|v.to_string()).unwrap());
            record.insert("device_name", row.get_column_by_name("device_name").map(|v|v.to_string()).unwrap());
            record.insert("disk_size", row.get_column_by_name("disk_size").map(|v|v.to_string()).unwrap());
            record.insert("mount_point", row.get_column_by_name("mount_point").map(|v|v.to_string()).unwrap());
            record.insert("registration_date", row.get_column_by_name("registration_date").map(|v|v.to_string()).unwrap());
            results.push(record);
        }
        let data: String = format!("{:?}", results);
        info!("Reply get peers: {}", data);
        info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(data.as_str()));
        Ok(Sample::new(KeyExpr::new(selector).unwrap(), data.as_str()))
    };
    Ok(reply.unwrap())
}

pub async fn file_list(db_session: Session, selector: String, user: MsgUsername) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, user);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", user);
        Err(user.username.into())
    } else {
        let mut query = get_from_table(
            "file",
            "file",
            "*",
            format!("username='{}'", user.username)); 
        info!("Query: {}", query);

        let stmt_get_my_file: CassResult = get_from_db(db_session.clone(), query).await;
        let mut my_files: Vec<String> = Vec::new();
        for row in stmt_get_my_file.iter() {
            let file_info = MsgFileInfo {
                file_id: row.get_column_by_name("file_id").map(|v|v.to_string()).unwrap(),
                file_name: row.get_column_by_name("file_name").map(|v|v.to_string()).unwrap(),
                file_size: row.get_column_by_name("file_size").map(|v|v.to_string()).unwrap(),
                file_size_compressed: row.get_column_by_name("file_size_compressed").map(|v|v.to_string()).unwrap(),
                file_type: row.get_column_by_name("file_type").map(|v|v.to_string()).unwrap(),
                file_status: row.get_column_by_name("file_status").map(|v|v.to_string()).unwrap(),
                upload_date: row.get_column_by_name("upload_date").map(|v|v.to_string()).unwrap(),
                owner: "Y".to_string(),
                write: "Y".to_string(),
                username: user.username.clone(),
                file_bytes: Vec::new(),
            };
            let mut bytes: Vec<u8> = Vec::new();
            prost::Message::encode(&file_info, &mut bytes).unwrap();
            let serialized = Engine::encode(
                &base64::engine::general_purpose::STANDARD, 
                bytes.clone());
            my_files.push(serialized);
        }

        query = get_from_table(
            "file",
            "permission",
            "*",
            format!("username='{}'", user.username.clone())); 
        info!("Query: {}", query);
        let stmt_get_permission: CassResult = get_from_db(db_session.clone(), query).await;
        let mut shared_file_permission: HashMap<String, MsgFileInfo> = HashMap::new();
        let mut shared_file_ids: Vec<String> = Vec::new();
        for row in stmt_get_permission.iter() {
            let file_info = MsgFileInfo {
                owner: row.get_column_by_name("owner").map(|v|v.to_string()).unwrap(),
                write: row.get_column_by_name("write").map(|v|v.to_string()).unwrap(),
                file_id: "".to_string(),
                file_name: "".to_string(),
                file_size: "".to_string(),
                file_size_compressed: "".to_string(),
                file_type: "".to_string(),
                file_status: "".to_string(),
                upload_date: "".to_string(),
                file_bytes: Vec::new(),
                username: user.username.clone(),
            };
            let id = row.get_column_by_name("file_id").map(|v|v.to_string()).unwrap();
            shared_file_ids.push(id.clone());
            shared_file_permission.insert(id, file_info);
        }
        info!("Shared Files: {:?}", shared_file_ids);

        let mut shared_files: Vec<String> = Vec::new();
        if shared_file_ids.len() <= 0 {
            info!("No shared files found.");
        } else { 
            query = get_from_table(
                "file",
                "file",
                "*",
                format!("file_id in ('{}')", shared_file_ids.join("','"))); 
            info!("Query: {}", query);
            let stmt_get_shared_files: CassResult = get_from_db(db_session, query).await;
            info!("Permission table: {:?}", shared_file_permission);
            for row in stmt_get_shared_files.iter() {
                let id: String = row.get_column_by_name("file_id").map(|v|v.to_string()).unwrap();
                info!("ID: {}", id);
                if let Some(msg_file_info) = shared_file_permission.get(&id){
                    let file_info = MsgFileInfo {
                        file_id: row.get_column_by_name("file_id").map(|v|v.to_string()).unwrap(),
                        file_name: row.get_column_by_name("file_name").map(|v|v.to_string()).unwrap(),
                        file_size: row.get_column_by_name("file_size").map(|v|v.to_string()).unwrap(),
                        file_size_compressed: row.get_column_by_name("file_size_compressed").map(|v|v.to_string()).unwrap(),
                        file_type: row.get_column_by_name("file_type").map(|v|v.to_string()).unwrap(),
                        file_status: row.get_column_by_name("file_status").map(|v|v.to_string()).unwrap(),
                        upload_date: row.get_column_by_name("upload_date").map(|v|v.to_string()).unwrap(),
                        owner: msg_file_info.owner.to_string(),
                        write: msg_file_info.write.to_string(),
                        username: user.username.clone(),
                        file_bytes: Vec::new(),
                    };
                    let mut bytes: Vec<u8> = Vec::new();
                    prost::Message::encode(&file_info, &mut bytes).unwrap();
                    let serialized = Engine::encode(
                        &base64::engine::general_purpose::STANDARD, 
                        bytes.clone());
                    shared_files.push(serialized);
                };
            }
        }
        let data: String = format!("{:?}", vec![my_files, shared_files]);
        info!("Reply get file list: {:?}", data);
        info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(data.as_str()));
        Ok(Sample::new(KeyExpr::new(selector).unwrap(), data.as_str()))
    };
    Ok(reply.unwrap())
    
}

pub async fn file_upload(db_session: Session, selector: String, username: String, file_name: String, file_size: String, file_size_c: String, file_type: String, upload_date: String, file_bytes: Vec<u8>) 
    -> (Result<Sample, zenoh::prelude::Value>, String, i32, BTreeMap<String, HashMap<String, String>>) {
    info!(">> [Queryable ] Received Query '{}' with values:\nusername '{}',\nfile_name '{}',\nfile_size '{}',\nfile_type '{}',\nupload_date '{}'", selector, username, file_name, file_size, file_type, upload_date);
    let mut file_id: String = "".to_string();
    let mut size = 0;
    let mut piece_chunk_peer: BTreeMap<String, HashMap<String, String>> = BTreeMap::new();

    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: \nselector '{}', \nusername '{}',\nfile_name '{}',\nfile_size '{}',\nfile_size_c '{}',\nfile_type '{}',\nupload_date '{}'", selector, username, file_name, file_size, file_size_c, file_type, upload_date);
        Err(Value::from(file_name))
    } else {
        let size_file = file_size.parse::<i32>().unwrap();
        let size_file_c = file_size_c.parse::<i32>().unwrap();
        size = match size_file_c < size_file {
            true => {
                info!("Using the compressed size {} instead of the actual size {}", size_file_c, size_file);
                size_file_c
            },
            false => {
                info!("Using the actual size {} instead of the compressed size {}", size_file, size_file_c);
                size_file
            },
        };

        let peers: Vec<String> = get_peers(db_session.clone(), size).await;
        if peers.len() <= 0 {
            warn!("No peer available.");
            Err(Value::from(file_name))
        } else {
        
            // File arguments
            let mut msg_map_file: HashMap<&str, String> = HashMap::new();
            msg_map_file.insert("file_name", file_name.clone());
            msg_map_file.insert("file_type", file_type);
            msg_map_file.insert("username", username.clone());
            msg_map_file.insert("upload_date", upload_date);
            msg_map_file.insert("file_size", file_size.clone());
            msg_map_file.insert("file_size_compressed", file_size_c.clone());
            msg_map_file.insert("file_status", FILE_STATUS_TO_DISTRIBUTE.to_string());
            file_id = Uuid::new_v4().to_string();
            msg_map_file.insert("file_id", file_id.clone());
            info!("Arguments map file: {:?}", msg_map_file);

            // Insert in table file
            let query_file = insert_into_table("file","file", msg_map_file.clone(), "IF NOT EXISTS");
            let res = write_into_db(db_session.clone(), selector.clone(), query_file, "IF NOT EXISTS").await;
            match res.clone().unwrap().value.to_string().as_str() {
                "true" => {
                    // Piece arguments
                    let mut msg_map_piece: HashMap<&str, String> = HashMap::new();
                    msg_map_piece.insert("file_id", file_id.to_string());
                    let piece_num = 1 + size/PIECE_SIZE;
                    info!("Piece num: {}", piece_num);
                    let mut chunk_peer: String = "".to_string();
                    for order in 0..piece_num {
                        msg_map_piece.insert("piece_order", order.to_string());
                        let piece_size = if size > PIECE_SIZE {
                            size = size - PIECE_SIZE;
                            PIECE_SIZE
                        } else {
                            size
                        };
                        msg_map_piece.insert("piece_size", piece_size.to_string());
                        let chunk_num = 1 + piece_size/MTU as i32;
                        msg_map_piece.insert("chunk_num", chunk_num.to_string());
                        let total_chunk: i32 = chunk_num*REPLICATION_FACTOR as i32/100;
                        msg_map_piece.insert("replication_num", total_chunk.to_string());
                        info!("Size: {}, Chunk num: {}, Total_chunks: {}", piece_size, chunk_num, total_chunk);
                        let chunk_peer_map = chunk_peer_selection(peers.clone(), chunk_num + total_chunk).await;
                        chunk_peer = if chunk_peer_map.is_empty() {
                                break;
                            } else {
                                chunk_peer_map
                                    .iter()
                                    .map(|(key, value)| format!("{{\"{}\":\"{:?}\"}}", key, value))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            };
                        msg_map_piece.insert("chunk_peer", chunk_peer.clone());
                        info!("Arguments map piece: {:?}", msg_map_file);

                        piece_chunk_peer.insert(order.to_string(), chunk_peer_map);

                        // Insert in table piece
                        let query = insert_into_table("file","piece", msg_map_piece.clone(), "");
                        info!("Query: {}", query);
                        let _ = write_into_db(db_session.clone(), selector.clone(), query, "").await;
                    }

                    let result_piece: String = if chunk_peer == "" {
                        let query_file = 
                            delete_from_table(
                                "file",
                                "file",
                                format!("file_id='{}' AND username='{}'", file_id, username));
                        info!("Query: {}", query_file);
                        let _stmt_delete_file = db_session.statement(query_file).execute().await;

                        let query_piece = delete_from_table("file", "piece", format!("file_id='{}'", file_id));
                        info!("Query: {}", query_piece);
                        let _stmt_delete_piece = db_session.statement(query_piece).execute().await;
                        "false".to_string()
                    } else {
                        file_id.clone()
                    };
                    Ok(Sample::new(KeyExpr::new(selector).unwrap(), result_piece))
                },
                _ => {
                    warn!("File already present.");
                    res
                }
            }
        }       
    };
    (reply, file_id, size, piece_chunk_peer)
}

async fn get_peers(db_session: Session, file_size: i32) -> Vec<String> {
    let peer_number: i32 = (1 + file_size/MTU as i32) * 3;
    let query = get_from_table(
        "peer",
        "selection",
        "*",
        format!("ranking in ('0','1','2','3','4','5','6','7','8','9','10') and disk_available > {} LIMIT {}", file_size, peer_number));
    println!("Query: {}", query);
    info!("Query: {}", query);
    let stmt_chunk_peer = get_from_db(db_session.clone(), query).await;
    info!("Statement {}", stmt_chunk_peer);
    let mut peers: Vec<String> = Vec::new();
    if stmt_chunk_peer.iter().count() < 1 {
        warn!("No suitable peer found");
    } else {
        for row in stmt_chunk_peer.iter() {
            let peer_id = row.get_column_by_name("peer_id").unwrap().to_string() ;
            peers.push(peer_id);
        }
        println!("Found peers: {:?}", peers);
    }
    peers
}

async fn chunk_peer_selection(peers: Vec<String>, chunks: i32) -> HashMap<String, String> {
    info!("Chunks: {}, Peers: {:?}", chunks, peers);
    let mut chunk_peer: HashMap<String, String> = HashMap::new();
    for chunk_num  in 0..chunks {
        let peer: &String = &peers[chunk_num as usize % peers.len()];
        chunk_peer.insert(format!("chunk{}", chunk_num), peer.to_string());
    }
    chunk_peer
}

pub async fn file_get(db_session: Session, selector: String, data: MsgFileId) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.username))
    } else {
        let query = get_from_table(
            "file",
            "piece",
            "*",
            format!("file_id='{}' ORDER BY piece_order", data.file_id)); 
        info!("Query: {}", query);
        let stmt_get_from_piece: CassResult = get_from_db(db_session.clone(), query).await;
        let mut file_bytes: Vec<u8> = Vec::new();

        let mut penalties: HashMap<String, i8> = HashMap::new();
        for row_piece in stmt_get_from_piece.iter() {
            let piece_order = row_piece.get_column_by_name("piece_order").map(|v|v.to_string()).unwrap();
            let chunk_num = row_piece.get_column_by_name("chunk_num").map(|v|v.to_string()).unwrap();
            let replication_num = row_piece.get_column_by_name("replication_num").map(|v|v.to_string()).unwrap();
            let chunk_peer = row_piece.get_column_by_name("chunk_peer").map(|v|v.to_string()).unwrap();
            let chunk_hash = row_piece.get_column_by_name("chunk_hash").map(|v|v.to_string()).unwrap();
            let piece_size = row_piece.get_column_by_name("piece_size").map(|v|v.to_string()).unwrap();
            let transfer_length = row_piece.get_column_by_name("transfer_length").map(|v|v.to_string()).unwrap();
            let symbol_size = row_piece.get_column_by_name("symbol_size").map(|v|v.to_string()).unwrap();
            let source_blocks = row_piece.get_column_by_name("source_blocks").map(|v|v.to_string()).unwrap();
            let sub_blocks = row_piece.get_column_by_name("sub_blocks").map(|v|v.to_string()).unwrap();
            let symbol_alignment = row_piece.get_column_by_name("symbol_alignment").map(|v|v.to_string()).unwrap();
            
            info!("Piece_order: {}, Chunk_num: {}, Rep_num: {}, chunk_peer: {}, chunk_hash: {}, piece size: {}, transfer_length: {}, symbol_size: {}, source_blocks: {}, sub_blocks: {}, symbol_alignment: {}", 
                piece_order, chunk_num, replication_num, chunk_peer, chunk_hash, piece_size, transfer_length, symbol_size, source_blocks, sub_blocks, symbol_alignment);
 
            let map_chunk_peer: HashMap<String, String> = match serde_json::from_str(chunk_peer.as_str()).unwrap() {
                serde_json::Value::Object(obj) => {
                    obj.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned())).collect()
                }
                _ => HashMap::new(),
            };
            let map_chunk_hash: HashMap<String, String> = match serde_json::from_str(chunk_hash.as_str()).unwrap() {
                serde_json::Value::Object(obj) => {
                    obj.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned())).collect()
                }
                _ => HashMap::new(),
            };
            let mut sorted_chunks: Vec<_> = map_chunk_peer.keys().cloned().collect();
            sorted_chunks.sort();
            info!("Sorted chunks {:?}", sorted_chunks);

            let mut packets: Vec<Vec<u8>> = Vec::new();
            for chunk in sorted_chunks {
                let chunk_bytes = download_file_locally(data.file_id.clone(), piece_order.clone(), chunk.as_str());
                if chunk_peer.is_empty() {
                    error!("No chunk found.");
                    let peer = map_chunk_peer[&chunk].as_str();
                    info!("Penalty for peer {}", peer);
                    penalties.entry(peer.to_string())
                        .and_modify(|v| {*v += PENALTY_NOT_FOUND;})
                        .or_insert(PENALTY_NOT_FOUND);
                    println!("Penalties: {:?}", penalties);
                } else if calculate_hash(&chunk_bytes) == map_chunk_hash[&chunk] {
                    info!("File hash does match. File ok.");
                    packets.push(chunk_bytes);
                } else {
                    error!("File hash does not match. File may be corrupted.");
                    let peer = map_chunk_peer[&chunk].as_str();
                    info!("Penalty for peer {}", peer);
                    println!("Penalty for peer {}", peer);
                    penalties.entry(peer.to_string())
                        .and_modify(|v| {*v += PENALTY_CORRUPTION;})
                        .or_insert(PENALTY_CORRUPTION);
                    println!("Penalties: {:?}", penalties);
                };
            }

            let config: ObjectTransmissionInformation = ObjectTransmissionInformation::new(
                transfer_length.parse::<u64>().unwrap(), 
                symbol_size.parse::<u16>().unwrap(),
                 source_blocks.parse::<u8>().unwrap(), 
                 sub_blocks.parse::<u16>().unwrap(), 
                 symbol_alignment.parse::<u8>().unwrap()
            );
            let mut decoder = Decoder::new(config);

            let mut piece_bytes = None;
            while !packets.is_empty() {
                piece_bytes = decoder.decode(EncodingPacket::deserialize(&packets.pop().unwrap()));
                if piece_bytes.is_some() {
                    break;
                }
            }
            file_bytes.extend(piece_bytes.unwrap());

            penalties_update(db_session.clone(), penalties.clone()).await;
        }

        let query = get_from_table(
            "file",
            "file",
            "*",
            format!("username='{}' and file_id='{}'", data.username, data.file_id)); 
        info!("Query: {}", query);

        let stmt_get_from_file: CassResult = get_from_db(db_session, query).await;
        let row_file = stmt_get_from_file.first_row().unwrap();

        let message_file = MsgFileInfo {
            username: data.username,
            file_name: row_file.get_column_by_name("file_name").map(|v|v.to_string()).unwrap(),
            file_size: row_file.get_column_by_name("file_size").map(|v|v.to_string()).unwrap(),
            file_size_compressed: row_file.get_column_by_name("file_size_compressed").map(|v|v.to_string()).unwrap(),
            file_type: row_file.get_column_by_name("file_type").map(|v|v.to_string()).unwrap(),
            file_status: row_file.get_column_by_name("file_status").map(|v|v.to_string()).unwrap(),
            upload_date: row_file.get_column_by_name("upload_date").map(|v|v.to_string()).unwrap(),
            file_bytes: file_bytes,
            file_id: data.file_id,
            owner: "".to_string(),
            write: "".to_string(),
        };
        
        let mut bytes: Vec<u8> = Vec::new();
        prost::Message::encode(&message_file, &mut bytes).unwrap();
        let serialized = Engine::encode(
            &base64::engine::general_purpose::STANDARD, 
            bytes.clone()
        );
        info!("Results: {:?}", serialized);
        info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(serialized.clone()));
        Ok(Sample::new(KeyExpr::new(selector).unwrap(), serialized))
    };
    Ok(reply.unwrap())
}

async fn penalties_update(db_session: Session, mut penalties: HashMap<String, i8>) {
    if !penalties.is_empty() {
        let peer_list = penalties.keys().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        let query = get_from_table(
            "peer",
            "restriction",
            "*",
            format!("peer_id in ({})", peer_list));
        info!("Query: {}", query);

        let stmt_get_peer_penalty: CassResult = get_from_db(db_session.clone(), query).await;
        if stmt_get_peer_penalty.iter().count() < 1 {
            error!("No peer found among these {} for penalty updates.", peer_list);
        } else {
            for row in stmt_get_peer_penalty.iter() {
                let peer_id = row.get_column_by_name("peer_id").map(|v|v.to_string()).unwrap();
                let penalty_past= row.get_column_by_name("penalty").map(|v|v.to_string()).unwrap();
                
                let total_penalties = penalties[&peer_id] + penalty_past.parse::<i8>().unwrap();
                let query = update_table(
                    "peer",
                    "restriction",
                    format!("penalty = '{}'", total_penalties),
                    format!("peer_id = '{}'", peer_id));
                println!("Query: {}", query);
                let _ = write_into_db(db_session.clone(), "no_selector".to_string(), query, "").await;
                println!("Updated penalty for peer {} from {} to {}", peer_id, penalty_past, total_penalties);
                penalties.remove(&peer_id);
            }
            for (peer_id, penalty) in penalties.iter() {
                let mut msg_map: HashMap<&str, String> = HashMap::new();
                msg_map.insert("peer_id", peer_id.to_string());
                msg_map.insert("penalty", penalty.to_string());
                msg_map.insert("last_penalty", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
                let query = insert_into_table(
                    "peer",
                    "restriction",
                    msg_map,
                    ""
                );
                println!("Query: {}", query);
                let _ = write_into_db(db_session.clone(), "no_selector".to_string(), query, "").await;
                println!("Updated penalty for peer {} from 0 to {}", peer_id, penalty);
            }           
            info!("Updated penalties for peers {:?}", penalties.keys());
        }
    }
}

fn download_file_locally(file_id: String, piece_id: String, chunk_id: &str) -> Vec<u8> {
    let file_name = format!("{}_{}_{}", file_id, piece_id, chunk_id);
    let path = format!("{}/{}", FILE_STORAGE_FOLDER, file_name);
    info!("Downloading file with path: {}", path);
    let file_content = match std::fs::read(path) {
        Ok(v) => {
            info!("File retrieved successfully.");
            v
        },
        Err(e) => {
            error!("File not found. Error: {}", e);
            Vec::new()
        },
    };
    file_content
}

fn download_file_from_peer(file_id: String, piece_id: String, chunk_id: String, peer: String) -> Vec<u8> {
    let selector = format!("{}/{}", KEY_EXPR_CHUNK_GET, peer);
    let file_name = format!("{}_{}_{}", file_id, piece_id, chunk_id);
    let path = format!("{}/{}", FILE_STORAGE_FOLDER, file_name);
    let file_contents = match std::fs::read(path) {
        Ok(v) => {
            info!("File retrieved successfully.");
            v
        },
        Err(e) => {
            error!("File not found. Error: {}", e);
            Vec::new()
        },
    };
    file_contents
}


pub async fn file_delete(db_session: Session, selector: String, data: MsgFileId) 
    -> (Result<Sample, zenoh::prelude::Value>, HashSet<String>) {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let mut peers_with_chunks: HashSet<String> = HashSet::new();
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.file_id))
    } else {
        let query_table_file_delete = delete_from_table(
            "file",
            "file",
            format!("username='{}' and file_id='{}'", data.username, data.file_id));
        println!("Query: {}", query_table_file_delete);
        let _ = write_into_db(db_session.clone(), selector.clone(), query_table_file_delete, "").await;
        
        let query_table_piece_get = get_from_table(
            "file",
            "piece",
            "chunk_peer",
            format!("file_id='{}'", data.file_id));
        println!("Query: {}", query_table_piece_get);
        let stmt_get_from_piece: CassResult = get_from_db(db_session.clone(), query_table_piece_get).await;
        println!("Statement {:?}", stmt_get_from_piece.iter());
        for row in stmt_get_from_piece.iter() {
            let chunk_peer = row.get_column_by_name("chunk_peer").map(|v|v.to_string()).unwrap();
            println!("Chunk peer: {:?}", chunk_peer);
            let map_chunk_peer: HashMap<String, String> = match serde_json::from_str(chunk_peer.as_str()).unwrap() {
                serde_json::Value::Object(obj) => {
                    obj.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned())).collect()
                }
                _ => HashMap::new(),
            };
            let peers: HashSet<String> = map_chunk_peer.values().cloned().collect();
            peers_with_chunks.extend(peers);
        }
        println!("Deletion of file: {}. List of peers to contact for deletion: {:?}", data.file_id, peers_with_chunks);

        let query_table_piece_delete = delete_from_table(
            "file",
            "piece",
            format!("file_id='{}'", data.file_id));
        println!("Query: {}", query_table_piece_delete);
        write_into_db(db_session, selector.clone(), query_table_piece_delete, "").await
    };
    (reply, peers_with_chunks)
}

pub async fn metrics_put(db_session: Session, selector: String, data: MsgMetrics) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.peer_id))
    } else {
        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("peer_id", data.peer_id);
        msg_map.insert("uptime_start", data.uptime_start);
        msg_map.insert("uptime_end", data.uptime_end);
        msg_map.insert("disk_read", data.disk_read);
        msg_map.insert("disk_write", data.disk_write);
        msg_map.insert("throughput", data.throughput);
                
        let query = insert_into_table(
            "peer",
            "performance",
            msg_map,
            "");
        info!("Query: {}", query);
        write_into_db(db_session, selector, query, "").await
    };
    Ok(reply.unwrap())
}

pub async fn metrics_get(db_session: Session, selector: String, data: MsgPeerId) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.peer_id))
    } else {
        let peers: Vec<&str> = data.peer_id.split(|c| c == ',').collect();
        let query: String = get_from_table (
            "peer",
            "performance",
            "*",
            format!("peer_id in ({})", peers.iter().map(|&id| format!("'{}'", id)).collect::<Vec<_>>().join(", ")));
        info!("Query: {}", query);
        let stmt_get_from_performance: CassResult = get_from_db(db_session, query).await;
        let mut metrics_vec: Vec<String> = Vec::new();
        for row in stmt_get_from_performance.iter() {
            let metric: MsgMetrics = MsgMetrics {
                peer_id: row.get_column_by_name("peer_id").map(|v|v.to_string()).unwrap(),
                uptime_start: row.get_column_by_name("uptime_start").map(|v|v.to_string()).unwrap(),
                uptime_end: row.get_column_by_name("uptime_end").map(|v|v.to_string()).unwrap(),
                disk_read: row.get_column_by_name("disk_read").map(|v|v.to_string()).unwrap(),
                disk_write: row.get_column_by_name("disk_write").map(|v|v.to_string()).unwrap(),
                throughput: row.get_column_by_name("throughput").map(|v|v.to_string()).unwrap(),
            };
            let mut bytes: Vec<u8> = Vec::new();
            prost::Message::encode(&metric, &mut bytes).unwrap();
            let serialized = Engine::encode(
                &base64::engine::general_purpose::STANDARD, 
                bytes.clone());
            metrics_vec.push(serialized);
        }
        let ret = format!("{:?}", metrics_vec);
        info!("Reply get metrics: {:?}", ret);
        info!(">> [Queryable ] Responding ('{}': '{}')", selector, zenoh::prelude::Value::from(ret.as_str()));
        Ok(Sample::new(KeyExpr::new(selector).unwrap(), ret.as_str()))
    };
    Ok(reply.unwrap())
}

pub async fn liveness_put(db_session: Session, selector: String, data: MsgLiveness) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.peer_id))
    } else {
        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("peer_id", data.peer_id.clone());
        let query_insert_liveness = insert_into_table("peer", "liveness", msg_map,"");
        let _ = write_into_db(db_session.clone(), selector.clone(), query_insert_liveness, "").await;

        let query = update_table(
            "peer",
            "selection",
            format!("last_liveness='{}'", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string()),
            format!("peer_id='{}'", data.peer_id));
        write_into_db(db_session, selector, query, "").await
    };
    Ok(reply.unwrap())
}

pub async fn permission_put(db_session: Session, selector: String, data: MsgPermissions) -> Result<Sample, zenoh::prelude::Value> {
    info!(">> [Queryable ] Received Query '{}' with value '{:?}'", selector, data);
    let send_errors = AtomicBool::new(false);
    let reply: Result<Sample, zenoh::prelude::Value> = if send_errors.swap(false, Relaxed) {
        error!(">> [Queryable ] Replying (ERROR: '{:?}')", data);
        Err(Value::from(data.username))
    } else {
        let mut msg_map: HashMap<&str, String> = HashMap::new();
        msg_map.insert("username", data.username.clone());
        msg_map.insert("file_id", data.file_id.clone());
        msg_map.insert("owner", data.owner.clone());
        msg_map.insert("write", data.write.clone());

        let query = insert_into_table(
            "user",
            "permission",
            msg_map,
            "IF NOT EXISTS");
        info!("Query: {}", query);
        let res = write_into_db(db_session.clone(), selector.clone(), query, "IF NOT EXISTS").await;
        match res.clone().unwrap().value.to_string().as_str() {
            "true" => {
                info!("Permission inserted.");
                res
            },
            _ => {
                warn!("Permission already present...updating it.");
                let query = update_table(
                    "user",
                    "permission",
                    format!("owner='{}' and write='{}'", data.owner, data.write),
                    format!("username='{}' and file_id='{}'", data.username, data.file_id));
                write_into_db(db_session, selector.clone(), query, "").await
            }
        }
    };
    Ok(reply.unwrap())
}
