use log::info;
use zenoh::config::Config;
use zenoh::prelude::sync::SyncResolve;
use	zenoh::prelude::config::WhatAmI;

extern crate storage;

fn main() {
	let	mut	config = Config::default();
    let _ = config.set_mode(Some(WhatAmI::Router));

    info!("Open zenoh session");
    let session = zenoh::open(config).res().unwrap();

    //let mut sub_new_users = session.subscribe("/storage/user/new/**").await.unwrap();
		
    //while let Some(sample) = sub_new_users.receiver().next().await {
    //    println!("('{}': '{}')",	sample.key_expr.as_str(), String::from_utf8_lossy(&sample.value.payload.contiguous()));
    //}
   
    /*for (index, share) in shares.iter().enumerate() {
        let share_expr = format!("share/{}{}", index, normalized_expr);

        println!("Putting share {} of '{}'. ", index, share_expr);
        let share_as_bytes: Vec<u8> = share.try_into().unwrap();
        session.put(&share_expr, share_as_bytes).res().unwrap();
    }*/

    session.close().res().unwrap();
}

/*fn connect_db() -> DBCommon<SingleThreaded, DBWithThreadModeInner> {
    let mut options = rocksdb::Options::default();
    options.set_error_if_exists(false);
    options.create_if_missing(true);
    options.create_missing_column_families(true);

    let path: &str = "./tmp";

    // list existing ColumnFamilies in the given path. returns Err when no DB exists.
    let cfs = rocksdb::DB::list_cf(&options, path).unwrap_or(vec![]);
    let my_column_family_exists = cfs
        .iter().find(|cf| cf == &"my_column_family").is_none();

    // open a DB with specifying ColumnFamilies
    let instance = rocksdb::DB::open_cf(&options, path, cfs).unwrap();

    if my_column_family_exists {
        // create a new ColumnFamily
        let options = rocksdb::Options::default();
        instance.create_cf("my_column_family", &options).unwrap();
    }

    instance // rocksdb::DB instance is available
}*/

