//! # Storage Crate
//! 
//! # 'storage' is a collection of utilities to run the server side of the storage system

mod signals;
mod query;
mod services;
mod constants;
mod database;
mod distribution;
mod protobuf;
mod utils;
mod rating;

use base64::Engine;
use database::create_db_schema;

use constants::*;

use crate::rating::cron_job;
use crate::{database::write_into_db, query::update_table};
use std::str::FromStr;

use signals::signals_handling;
use signal_hook::consts::{SIGINT, SIGALRM, SIGQUIT, SIGTERM};

use cassandra_cpp::*;
use cassandra_cpp::Error;

use zenoh::config::{EndPoint, ListenConfig, ZenohId};
use zenoh::key_expr::KeyExpr;
use zenoh::queryable::{Query, Queryable};
use zenoh::{config::Config, prelude::r#async::AsyncResolve};
use	zenoh::prelude::config::WhatAmI;
use zenoh::*;
use zenoh::subscriber::Subscriber;
use zenoh::sample::Sample;

use flume::Receiver;
use futures::select;

use std::collections::HashMap;
use std::{env, fs};
use std::sync::Arc;
use std::{prelude::v1::Result};

use log::{debug, error, log_enabled, info, Level, warn};

use crate::protobuf::{MsgDevice, MsgFileInfo, MsgFileId, MsgLiveness, MsgMetrics, MsgPeerId, MsgPermissions, MsgUser, MsgUsername};
use crate::distribution::Distribution;

use bytes::Bytes;
use prost::{bytes, Message};

pub struct Storage <'a> {
    db_session: cassandra_cpp::Session,
    network_session: Arc<zenoh::Session>,
    sub_user_signup: Queryable<'a, Receiver<Query>>,
    sub_user_login: Queryable<'a, Receiver<Query>>,
    sub_peer_signup: Queryable<'a, Receiver<Query>>,
    sub_peer_get: Queryable<'a, Receiver<Query>>,
    sub_file_list: Queryable<'a, Receiver<Query>>,
    sub_file_upload: Queryable<'a, Receiver<Query>>,
    //sub_file_upload_cloud: Queryable<'a, Receiver<Query>>,
    sub_file_get: Queryable<'a, Receiver<Query>>,
    sub_file_delete: Queryable<'a, Receiver<Query>>,
    sub_metrics_put: Queryable<'a, Receiver<Query>>,
    sub_metrics_get: Queryable<'a, Receiver<Query>>,
    sub_liveness_put: Subscriber<'a, Receiver<Sample>>,
    sub_permission_put: Queryable<'a, Receiver<Query>>,
}

impl Storage<'_> {
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        //TODO: add user and password for connection and within Cassandra

        //let tls_ca_certificate_path = env::var("CERT_PATH").expect("CERT_PATH must be set");
        //info!("TLS CA certificate path: {:?}", tls_ca_certificate_path);
        //let cert = fs::read_to_string(tls_ca_certificate_path).expect("Failed to open certificate file");

        let mut ssl = cassandra_cpp::Ssl::default();
        //ssl.add_trusted_cert(cert).unwrap();
        ssl.set_verify_flags(&[cassandra_cpp::SslVerifyFlag::PEER_IDENTITY]);

        let cassandra_host = env::var("CASSANDRA_HOST").expect("CASSANDRA_HOST must be set");
        info!("Cassandra host: {:?}", cassandra_host);
        let mut cluster = Cluster::default();
        cluster.set_contact_points(cassandra_host.as_str()).unwrap();
        cluster.set_load_balance_round_robin();
        //cluster.set_ssl(ssl);

        let db_session = cluster.connect().await.expect("Failed to connect to Database");

        let	mut	config = Config::default(); 
        //let zlisten1 = EndPoint::from_str("tcp/172.22.209.161:8000").unwrap();
        let zenoh_host = env::var("ZENOH_HOST").expect("ZENOH_HOST must be set");
        let endpoint = format!("tcp/{}:7447", zenoh_host);
        info!("Zenoh enpoint: {:?}", endpoint);
        let zlisten = EndPoint::from_str(&endpoint).unwrap();
        let _ = config.set_listen(ListenConfig::new(vec![zlisten]).unwrap());
        let _ = config.set_mode(Some(WhatAmI::Router));
        info!("Open zenoh session");
        let network_session = Arc::new(zenoh::open(config).res().await.unwrap());
        
        let info = network_session.info();
        info!("Router info:");
        info!("zid={:?}", info.zid().res().await);
        info!("router_zid={:?}", info.routers_zid().res().await.collect::<Vec<ZenohId>>());
        info!("peers_zid={:?}", info.peers_zid().res().await.collect::<Vec<ZenohId>>());

        let sub_user_signup = network_session.declare_queryable(KEY_EXPR_USER_SIGNUP).res().await.unwrap();
        let sub_user_login = network_session.declare_queryable(KEY_EXPR_USER_LOGIN).res().await.unwrap();
        let sub_peer_signup = network_session.declare_queryable(KEY_EXPR_PEER_SIGNUP).res().await.unwrap();
        let sub_peer_get = network_session.declare_queryable(KEY_EXPR_PEER_GET).res().await.unwrap();
        let sub_file_list = network_session.declare_queryable(KEY_EXPR_FILE_LIST).res().await.unwrap();
        let sub_file_upload = network_session.declare_queryable(KEY_EXPR_FILE_UPLOAD).res().await.unwrap();
        //let sub_file_upload_cloud = network_session.declare_queryable(KEY_EXPR_FILE_UPLOAD).res().await.unwrap();
        let sub_file_get = network_session.declare_queryable(KEY_EXPR_FILE_GET).res().await.unwrap();
        let sub_file_delete = network_session.declare_queryable(KEY_EXPR_FILE_DELETE).res().await.unwrap();
        let sub_metrics_put = network_session.declare_queryable(KEY_EXPR_METRICS_PUT).res().await.unwrap();
        let sub_metrics_get = network_session.declare_queryable(KEY_EXPR_METRICS_GET).res().await.unwrap();
        let sub_liveness_put = network_session.declare_subscriber(KEY_EXPR_LIVENESS_PUT).res().await.unwrap();
        let sub_permission_put = network_session.declare_queryable(KEY_EXPR_PERMISSION_PUT).res().await.unwrap();
    
        let storage = Storage {
            db_session,
            network_session,
            sub_user_signup,
            sub_user_login,
            sub_peer_signup,
            sub_peer_get,
            sub_file_list,
            sub_file_upload,
            //sub_file_upload_cloud,
            sub_file_get,
            sub_file_delete,
            sub_metrics_put,
            sub_metrics_get,
            sub_liveness_put,
            sub_permission_put,
        };

        //signals_handling(SIGINT, &storage);
        //signals_handling(SIGALRM, &storage);
        //signals_handling(SIGQUIT, &storage);
        //signals_handling(SIGTERM, &storage);

        storage
    }

    pub async fn initialize(&mut self) -> Result<(), Error> {        
        let _ = create_db_schema(&self.db_session).await;

        cron_job(&self.db_session);

        info!("Subscribe to user signup with key {}.", KEY_EXPR_USER_SIGNUP);
        self.sub_user_signup = self.network_session.declare_queryable(KEY_EXPR_USER_SIGNUP).res().await.unwrap();

        info!("Subscribe to user login with key {}.", KEY_EXPR_USER_LOGIN);
        self.sub_user_login = self.network_session.declare_queryable(KEY_EXPR_USER_LOGIN).res().await.unwrap();
        
        info!("Subscribe to peer signup with key {}.", KEY_EXPR_PEER_SIGNUP);
        self.sub_peer_signup = self.network_session.declare_queryable(KEY_EXPR_PEER_SIGNUP).res().await.unwrap();

        info!("Subscribe to peer get with key {}.", KEY_EXPR_PEER_GET);
        self.sub_peer_get = self.network_session.declare_queryable(KEY_EXPR_PEER_GET).res().await.unwrap();
        
        info!("Subscribe to file list with key {}.", KEY_EXPR_FILE_LIST);
        self.sub_file_list = self.network_session.declare_queryable(KEY_EXPR_FILE_LIST).res().await.unwrap();

        info!("Subscribe to file upload with key {}.", KEY_EXPR_FILE_UPLOAD);
        self.sub_file_upload = self.network_session.declare_queryable(KEY_EXPR_FILE_UPLOAD).res().await.unwrap();

        //info!("Subscribe to file upload on the cloud with key {}.", KEY_EXPR_FILE_UPLOAD_CLOUD);
        //self.sub_file_upload_cloud = self.network_session.declare_queryable(KEY_EXPR_FILE_UPLOAD_CLOUD).res().await.unwrap();
        
        info!("Subscribe to file get with key {}.", KEY_EXPR_FILE_GET);
        self.sub_file_get = self.network_session.declare_queryable(KEY_EXPR_FILE_GET).res().await.unwrap();
        
        info!("Subscribe to file delete with key {}.", KEY_EXPR_FILE_DELETE);
        self.sub_file_delete = self.network_session.declare_queryable(KEY_EXPR_FILE_DELETE).res().await.unwrap();
        
        info!("Subscribe to metrics put with key {}.", KEY_EXPR_METRICS_PUT);
        self.sub_metrics_put = self.network_session.declare_queryable(KEY_EXPR_METRICS_PUT).res().await.unwrap();

        info!("Subscribe to metrics get with key {}.", KEY_EXPR_METRICS_GET);
        self.sub_metrics_get = self.network_session.declare_queryable(KEY_EXPR_METRICS_GET).res().await.unwrap();

        info!("Subscribe to liveness put with key {}.", KEY_EXPR_LIVENESS_PUT);
        self.sub_liveness_put = self.network_session.declare_subscriber(KEY_EXPR_LIVENESS_PUT).res().await.unwrap();

        info!("Subscribe to permission put with key {}.", KEY_EXPR_PERMISSION_PUT);
        self.sub_permission_put = self.network_session.declare_queryable(KEY_EXPR_PERMISSION_PUT).res().await.unwrap();

        Ok(())
    }

    pub async fn execute(&self) -> ! {
        loop {
            select!(
                user_signup = self.sub_user_signup.recv_async() => {
                    let user_signup = user_signup.unwrap();
                    match user_signup.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", user_signup.selector()),
                        Some(value) => {
                            let data: MsgUser = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);
                            
                            let reply = services::user_signup(self.get_db_session(), user_signup.selector().to_string(), data).await;
                            user_signup
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                user_login = self.sub_user_login.recv_async() => {
                    let user_login = user_login.unwrap();
                    match user_login.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", user_login.selector()),
                        Some(value) => {    
                            let data: MsgUsername = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::user_login(self.get_db_session(), user_login.selector().to_string(), data).await;
                            user_login
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                peer_signup = self.sub_peer_signup.recv_async() => {
                    let peer_signup = peer_signup.unwrap();
                    match peer_signup.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", peer_signup.selector()),
                        Some(value) => {
                            let data: MsgDevice = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::peer_signup(self.get_db_session(), peer_signup.selector().to_string(), data).await;
                            peer_signup
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                peer_get = self.sub_peer_get.recv_async() => {
                    let peer_get = peer_get.unwrap();
                    match peer_get.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", peer_get.selector()),
                        Some(value) => {
                            let data: MsgUsername = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::peer_get(self.get_db_session(), peer_get.selector().to_string(), data).await;
                            peer_get
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                file_list = self.sub_file_list.recv_async() => {
                    let file_list = file_list.unwrap();
                    match file_list.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", file_list.selector()),
                        Some(value) => {
                            let data: MsgUsername = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::file_list(self.get_db_session(), file_list.selector().to_string(), data).await;
                            file_list
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                file_upload = self.sub_file_upload.recv_async() => {
                    let file_upload = file_upload.unwrap();
                    match file_upload.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", file_upload.selector()),
                        Some(value) => {
                            let req_metadata_bin = Engine::decode(
                                    &base64::engine::general_purpose::STANDARD, 
                                    value.to_string())
                                .unwrap();
                            let data: MsgFileInfo = prost::Message::decode(req_metadata_bin.as_slice()).unwrap();
                            
                            info!("Username: {}", data.username);
                            info!("File Name: {}", data.file_name);
                            info!("File Size: {}", data.file_size);
                            info!("File Size compressed: {}", data.file_size_compressed);
                            info!("File Type: {}", data.file_type);
                            info!("Upload Date: {}", data.upload_date);
                            
                            let (reply, file_id, file_size, piece_chunk_peer) = 
                                services::file_upload(
                                    self.get_db_session(),
                                    file_upload.selector().to_string(),
                                    data.username.clone(),
                                    data.file_name,
                                    data.file_size,
                                    data.file_size_compressed,
                                    data.file_type,
                                    data.upload_date,
                                    data.file_bytes.clone(),
                                ).await;

                            match reply {
                                Ok(val) => {
                                     // Reply to client
                                    file_upload
                                    .reply(Ok(val.clone()))
                                    .res()
                                    .await
                                    .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));

                                    // Distribute chunks to peers
                                    if !val.value.to_string().is_empty() && val.value.to_string() != "false"{
                                        let file_id = val.value.to_string();
                                        info!("File inserted. I'm going to distribute chunks.");
                                        let distribution = Distribution::new(file_id.clone(), self.network_session.clone(), self.db_session.clone());
                                        match distribution.send_chunks(data.file_bytes, piece_chunk_peer).await{
                                            true => {
                                                info!("Chunks distributed.");
                                                let query_update_piece = update_table(
                                                    "file",
                                                    "file",
                                                    format!("file_status = '{}'", FILE_STATUS_READY),
                                                    format!("file_id = '{}' and username = '{}'", file_id, data.username)
                                                );
                                                let _ = write_into_db(self.db_session.clone(), "not_a_selector/not_a_selector".to_string(), query_update_piece.clone(), "").await;
                                            },
                                            _ => error!("Couldn't distribute chunks."),
                                        
                                        };
                                    } else {
                                        warn!("File not inserted. I do not distribute chunks.");
                                    } 
                                },
                                Err(e) => {
                                    error!("Doing nothing. Error during saving the file into DB: {}", e);
                                },
                            };
                        }
                    }
                }
                file_get = self.sub_file_get.recv_async() => {
                    let file_get = file_get.unwrap();
                    match file_get.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", file_get.selector()),
                        Some(value) => {
                            let data: MsgFileId = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::file_get(self.get_db_session(), file_get.selector().to_string(), data).await;
                            file_get
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                file_delete = self.sub_file_delete.recv_async() => {
                    let file_delete = file_delete.unwrap();
                    match file_delete.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", file_delete.selector()),
                        Some(value) => {
                            let data: MsgFileId = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let (reply, peers_with_chunks) = services::file_delete(self.get_db_session(), file_delete.selector().to_string(), data.clone()).await;
                            file_delete
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));

                            let distribution = Distribution::new(data.file_id.clone(), self.network_session.clone(), self.db_session.clone());
                            distribution.delete_file(peers_with_chunks).await;
                        }
                    }
                }
                metrics_put = self.sub_metrics_put.recv_async() => {
                    let metrics_put = metrics_put.unwrap();
                    match metrics_put.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", metrics_put.selector()),
                        Some(value) => {
                            let data: MsgMetrics = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::metrics_put(self.get_db_session(), metrics_put.selector().to_string(), data).await;
                            metrics_put
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                metrics_get = self.sub_metrics_get.recv_async() => {
                    let metrics_get = metrics_get.unwrap();
                    match metrics_get.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", metrics_get.selector()),
                        Some(value) => {
                            let data: MsgPeerId = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::metrics_get(self.get_db_session(), metrics_get.selector().to_string(), data).await;
                            metrics_get
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
                liveness_put = self.sub_liveness_put.recv_async() => {
                    info!("Received liveness put.");
                    let liveness_put = liveness_put.unwrap();
                    let value = liveness_put.value;
                    let selector = liveness_put.key_expr;
                    let data: MsgLiveness = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                    info!("Data input: {:?}", data);
                    match services::liveness_put(self.get_db_session(), selector.to_string(), data.clone()).await{
                        Ok(res) => info!("Updated liveness {} with result {:?}", data.peer_id, res),
                        Err(err) => error!("Error during updating liveness for peer {}: {:?}", data.peer_id, err),
                    };                    
                }
                permission_put = self.sub_permission_put.recv_async() => {
                    let permission_put = permission_put.unwrap();
                    match permission_put.value() {
                        None => warn!(">> [Queryable ] Received Query '{}' with no value", permission_put.selector()),
                        Some(value) => {
                            let data: MsgPermissions = prost::Message::decode(value.to_string().as_bytes()).unwrap();
                            info!("Data input: {:?}", data);

                            let reply = services::permission_put(self.get_db_session(), permission_put.selector().to_string(), data).await;
                            permission_put
                                .reply(reply)
                                .res()
                                .await
                                .unwrap_or_else(|e| error!(">> [Queryable ] Error sending reply: {e}"));
                        }
                    }
                }
            );
        }
    }

    pub async fn close(self) {
        // Cleanup logic goes here
        info!("Unsubscribing from user signup with key {}.", KEY_EXPR_USER_SIGNUP);
        self.network_session.undeclare(self.sub_user_signup).res().await.unwrap();
        self.network_session.undeclare(self.sub_user_login).res().await.unwrap();
        self.network_session.undeclare(self.sub_peer_signup).res().await.unwrap();
        self.network_session.undeclare(self.sub_file_upload).res().await.unwrap();
        self.network_session.undeclare(self.sub_file_get).res().await.unwrap(); 

        info!("Closing the module...");
        Arc::try_unwrap(self.network_session).unwrap().close().res().await.unwrap();
    }

    pub fn get_db_session(&self) -> cassandra_cpp::Session{
        self.db_session.clone()
    }

    pub fn get_network_session(&self) -> Arc<zenoh::Session>{
        self.network_session.clone()
    }
}
