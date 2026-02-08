use std::{collections::{BTreeMap, HashMap, HashSet}, sync::Arc};

use cassandra_cpp::Session;
use log::{error, info, warn};
use raptorq::Encoder;
use zenoh::{prelude::r#async::AsyncResolve, query::{ConsolidationMode, QueryTarget}};
use std::io::Write;

use crate::{database::{get_from_db, write_into_db}, protobuf::{MsgChunk, MsgUsername}, query::{delete_from_table, get_from_table, insert_into_table_with_num, update_table}, utils::calculate_hash};
use super::constants::*;

pub struct Distribution {
    file_id: String,
    network_session: Arc<zenoh::Session>,
    db_session: cassandra_cpp::Session,
    //chunk_peer: BTreeMap<String, HashMap<String, String>> // piece_id: chunk_id: (peer_id, checksum)
}

impl Distribution {
    pub fn new(id: String, network: Arc<zenoh::Session>, db: Session) -> Self {

        Distribution {
            file_id: id,
            network_session: network,
            db_session: db,
            //chunk_peer: piece_chunk_peer,
        }
    }

    pub async fn send_chunks(self, bytes: Vec<u8>, chunk_peer: BTreeMap<String, HashMap<String, String>>) -> bool {
        let mut pieces: Vec<&[u8]> = bytes.chunks(PIECE_SIZE as usize).collect();
        let mut encoder: Encoder;
        let mut i = 0;

        println!("Chunk peer: {:?}", chunk_peer);
        for (piece_id, mut map_chunk_peer) in chunk_peer {
            info!("Preparing piece_id: {}", piece_id);
            let piece = pieces.remove(0);
            encoder = Encoder::with_defaults(piece, MTU);
            info!("Encoder config: {:?}", encoder.get_config());

            let repair_packets_per_block = map_chunk_peer.len();
            let packets: Vec<Vec<u8>> = encoder
                .get_encoded_packets(repair_packets_per_block.try_into().unwrap())
                .iter()
                .map(|packet| packet.serialize())
                .collect();
            info!("Piece id {}: Total chunks to send: {:?} packets - {:?} KB", i, packets.len(), packets.len()*MTU as usize);
            i = i + 1;
            let mut packet_count = 0;
            let mut chunk_hash_map: HashMap<String, String> = HashMap::new();
            for (chunk, peer) in map_chunk_peer.clone() {
                info!("Send to peer {} a chunk {}.", peer, chunk);
                let selector = format!("{}{}", KEY_EXPR_FILE_DISTRIBUTION, peer);
                info!("Selector: {}", selector);
                
               let msg_chunk = MsgChunk {
                    file_id: self.file_id.clone(),
                    piece_id: piece_id.clone(),
                    chunk_id: chunk.clone(),
                    chunk_bytes: packets[packet_count].clone(),
                };
                packet_count = packet_count + 1;
                let hash = calculate_hash(msg_chunk.chunk_bytes.as_slice());
                info!("Hash: {}", hash);
                chunk_hash_map.insert(chunk.clone(), hash);

                //match upload_file_nodes(network_session, selector, msg_chunk) {
                match upload_file_locally(msg_chunk.clone()) {
                    true => {
                        info!("Chunk distributed successfully.");
                        let query_get_peer_selection = get_from_table(
                            "peer",
                            "selection",
                            "*",
                            format!("peer_id = '{}'", peer));
                        println!("Query: {}", query_get_peer_selection);
                        info!("Query: {}", query_get_peer_selection);
                        let stmt_chunk_peer = get_from_db(self.db_session.clone(), query_get_peer_selection).await;
                        let disk_available = stmt_chunk_peer.first_row().unwrap().get_column_by_name("disk_available").unwrap().to_string();
                        let ranking = stmt_chunk_peer.first_row().unwrap().get_column_by_name("ranking").unwrap().to_string();
                        let country = stmt_chunk_peer.first_row().unwrap().get_column_by_name("country").unwrap().to_string();
                        let last_liveness = stmt_chunk_peer.first_row().unwrap().get_column_by_name("last_liveness").unwrap().to_string();
                        let mount_point = stmt_chunk_peer.first_row().unwrap().get_column_by_name("mount_point").unwrap().to_string();
                        let pieces = stmt_chunk_peer.first_row().unwrap().get_column_by_name("pieces").unwrap().to_string();

                        println!("Pieces: {}", pieces);
                        let mut map_pieces: HashMap<String, String> = match serde_json::from_str(pieces.as_str()).unwrap() {
                            serde_json::Value::Object(obj) => {
                                obj.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned())).collect()
                            }
                            _ => HashMap::new(),
                        };

                        map_pieces.entry(self.file_id.clone())
                            .and_modify(|v| {
                                print!("v: {}", v);
                                *v = format!("{}, {}_{}", v, piece_id, chunk);
                            })
                            .or_insert(format!("{}_{}", piece_id, chunk));
                        
                        let disk_available_new = disk_available.parse::<i32>().unwrap() - msg_chunk.chunk_bytes.len() as i32;
                        println!("Disk available: {}", disk_available_new);

                        let query_insert_peer_selection = delete_from_table(
                            "peer",
                            "selection",
                            format!("peer_id = '{}' and ranking = '{}' and disk_available = {}", peer, ranking, disk_available)
                        );
                        println!("Query: {}", query_insert_peer_selection);
                        info!("Query: {}", query_insert_peer_selection);
                        let _ = write_into_db(self.db_session.clone(), "not_a_selector/not_a_selector".to_string(), query_insert_peer_selection, "").await;
             
                        let mut msg_map: HashMap<&str, String> = HashMap::new();
                        msg_map.insert("peer_id", peer);
                        msg_map.insert("ranking", ranking);
                        msg_map.insert("country", country);
                        msg_map.insert("last_liveness", last_liveness);
                        msg_map.insert("mount_point", mount_point);
                        msg_map.insert("pieces", serde_json::to_string(&map_pieces).unwrap());

                        let mut msg_map2: HashMap<&str, String> = HashMap::new();
                        msg_map2.insert("disk_available", disk_available_new.to_string());

                        let query_insert_peer_selection = insert_into_table_with_num(
                            "peer",
                            "selection",
                            msg_map,
                            msg_map2,
                            ""
                        );
                        println!("Query: {}", query_insert_peer_selection);
                        let _ = write_into_db(self.db_session.clone(), "no_selector".to_string(), query_insert_peer_selection, "").await;

                        info!("Updated the disk_available.");
                    },
                    false => {
                        map_chunk_peer.remove(&chunk);
                        warn!("Chunk not distributed. Map chunk peer to be updated: {:?}.", map_chunk_peer);

                        todo!("Find another place");

                    }, 
                };
            }

            println!("Chunk hash map: {:?}", chunk_hash_map);
            let chunk_hash = chunk_hash_map
                .iter()
                .map(|(key, value)| format!("\"{}\":{:?}", key, value))
                .collect::<Vec<String>>()
                .join(", ");
       
            println!("Chunk hash: {}", chunk_hash);
            let query_update_piece = update_table(
                "file",
                "piece",
                format!("chunk_hash = '{{{}}}', transfer_length = '{}', symbol_size = '{}', source_blocks = '{}', sub_blocks = '{}', symbol_alignment = '{}', chunk_peer = '{:?}'", 
                    chunk_hash, 
                    encoder.get_config().transfer_length(), 
                    encoder.get_config().symbol_size(), 
                    encoder.get_config().source_blocks(),
                    encoder.get_config().sub_blocks(),
                    encoder.get_config().symbol_alignment(), 
                    map_chunk_peer.clone()),
                format!("file_id = '{}' and piece_order = '{}'", self.file_id, piece_id)
            );
            let _ = write_into_db(self.db_session.clone(), "not_a_selector/not_a_selector".to_string(), query_update_piece.clone(), "").await;
        }
        true
    }

    /*fn set_chunk_peer(mut self, new_peer_list: Vec<String>) {
        self.peer_list = new_peer_list;
    }

    fn get_summary(self) -> String {
        format!("File: {}, Size: {}, Pieces: {:?}", self.file_name, self.file_size, self.pieces
        )
    }*/

    pub async fn delete_file(self, peers_with_chunks: HashSet<String>) {
        for peer in peers_with_chunks {     
            println!("Contacting peer {} for deletion of file {:?}", peer, self.file_id.clone());
            delete_file_locally(self.file_id.clone());

            // 2. Delete chunks from peer.selection table, pieces column
            // 3. Recalculate the disk_availability in peer.selection table
            let query_get_peer_selection = get_from_table(
                "peer",
                "selection",
                "*",
                format!("peer_id = '{}'", peer));
            println!("Query: {}", query_get_peer_selection);
            info!("Query: {}", query_get_peer_selection);
            let stmt_chunk_peer = get_from_db(self.db_session.clone(), query_get_peer_selection).await;
            let ranking = stmt_chunk_peer.first_row().unwrap().get_column_by_name("ranking").unwrap().to_string();
            let disk_available = stmt_chunk_peer.first_row().unwrap().get_column_by_name("disk_available").unwrap().to_string();
            let country = stmt_chunk_peer.first_row().unwrap().get_column_by_name("country").unwrap().to_string();
            let mount_point = stmt_chunk_peer.first_row().unwrap().get_column_by_name("mount_point").unwrap().to_string();
            let pieces = stmt_chunk_peer.first_row().unwrap().get_column_by_name("pieces").unwrap().to_string();
            let mut map_pieces: HashMap<String, String> = match serde_json::from_str(pieces.as_str()).unwrap() {
                serde_json::Value::Object(obj) => {
                    obj.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_owned())).collect()
                }
                _ => HashMap::new(),
            };
            let count = map_pieces[&self.file_id].split(",").count();
            println!("Count chunks: {}", count);
            let new_disk_available = disk_available.parse::<i32>().unwrap() + count as i32 * MTU as i32;
            map_pieces.remove(&self.file_id);
            let pieces_str = serde_json::to_string(&map_pieces).unwrap();
            
            let query_delete_peer_selection = delete_from_table(
                "peer",
                "selection",
                format!("ranking = '{}' and disk_available = {} and peer_id = '{}'", ranking, disk_available, peer)
            );
            println!("Query: {}", query_delete_peer_selection);
            info!("Query: {}", query_delete_peer_selection);
            let _ = write_into_db(self.db_session.clone(), "not_a_selector".to_string(), query_delete_peer_selection, "").await;
            
            let mut msg_map: HashMap<&str, String> = HashMap::new();
            msg_map.insert("peer_id", peer.clone());
            msg_map.insert("ranking", ranking);
            msg_map.insert("country", country);
            msg_map.insert("mount_point", mount_point);
            msg_map.insert("pieces", pieces_str);

            let mut msg_map2: HashMap<&str, String> = HashMap::new();
            msg_map2.insert("disk_available", new_disk_available.to_string());

            let query_insert_peer_selection = insert_into_table_with_num(
                "peer",
                "selection",
                msg_map,
                msg_map2,
                ""
            );
            let _ = write_into_db(self.db_session.clone(), "not_a_selector".to_string(), query_insert_peer_selection, "").await;
            println!("Updated peer.selection table for peer {} with disk_availability and pieces", peer);
        }
    }
}

fn delete_file_locally(file_id: String) {
    let entries = std::fs::read_dir(FILE_STORAGE_FOLDER).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if let Some(file_name) = file_path.file_name() {
            if let Some(name) = file_name.to_str() {
                if name.starts_with(&format!("{}_", file_id)) {
                    std::fs::remove_file(&file_path).unwrap();
                    println!("File '{:?}' deleted successfully.", file_path.display());
                }
            }
        }
    }
 
    /*match std::fs::remove_file(path.clone()) {
        Ok(_) => println!("File '{}' deleted successfully.", path),
        Err(err) => eprintln!("Error deleting file '{}': {}", path, err),
    }*/
}

fn upload_file_locally(msg_chunk: MsgChunk) -> bool {
    let file_name = format!("{}_{}_{}", msg_chunk.file_id, msg_chunk.piece_id, msg_chunk.chunk_id);
    let path = format!("{}/{}", FILE_STORAGE_FOLDER, file_name);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path).unwrap();
    let res = match file.write_all(&msg_chunk.chunk_bytes){
        Ok(_) => {
            info!("File saved successfully at: {}", path);
            true
        },
        Err(_) => {
            warn!("File not saved at: {}", path);
            false
        },
    };
    res
}

async fn upload_file_nodes(network_session: Arc<zenoh::Session>, selector: String, msg_chunk: MsgChunk) {
    let mut encoded_msg = vec![];
    prost::Message::encode(&msg_chunk, &mut encoded_msg).unwrap();

    let	replies = network_session
        .get(selector)
        .with_value(encoded_msg)
        .consolidation(ConsolidationMode::None)
        .target(QueryTarget::BestMatching)
        .res_async()
        .await
        .unwrap();

    while let Ok(reply) = replies.recv_async().await {
        match reply.sample {
            Ok(sample) => info!(">> Received ('{}': '{}')", sample.key_expr.as_str(), sample.value, ),
            Err(err) => error!(">> Received (ERROR: '{}')", String::try_from(&err).unwrap()),
        }
    }
}

/*
let file_size: i32 = msg_map["file_size"].parse().unwrap();
let piece_num_int = file_size/PIECE_SIZE;
let piece_num = &((file_size/PIECE_SIZE).to_string())[..];
msg_map.insert("piece_num", piece_num);
let mut pieces: Vec<String> = Vec::new();
for _i in 1..=piece_num_int {
    let id = &Uuid::new_v4();
    pieces.push(id.to_string());
}
let pieces_str: String = format!("{:?}", pieces);*/

//msg_map.insert("piece_num", piece_num);
//msg_map.insert("piece_list", pieces_str.as_str());
