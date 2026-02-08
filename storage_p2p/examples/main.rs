use std::io::Error;
use storage::Storage;
use zenoh::{config::Config, prelude::r#async::AsyncResolve};

#[tokio::main]
async fn main() -> Result<(), Error> {
   // let _ = env_logger::try_init();
 
    let mut storage = Storage::new().await;
    let _ = storage.initialize() .await;
    let _ = storage.execute().await;

    storage.close().await;
    
   /*let contact_point = "127.0.0 .1";

    let mut cluster = Cluster::default();
    cluster.set_contact_points(contact_point).unwrap();
    cluster.set_load_balance_round_robin();
    let session = cluster.connect().await?;

    let query = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class':'NetworkTopologyStrategy', 'replication_factor':'2'}};", 
        "user"
    );
    let result = session
        //.execute("SELECT keyspace_name FROM system_schema.keyspaces;")
        .execute(query)
        .await?;
    println!("{}", result);

    for row in result.iter() {
        let col: String = row.get_by_name("keyspace_name")?;
        println!("ks name = {}", col);
    }
*/
    Ok(())
}