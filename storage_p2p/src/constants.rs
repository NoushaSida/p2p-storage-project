// Queriable
pub const KEY_EXPR_USER_SIGNUP: &str = "storage/user/signup/**";
pub const KEY_EXPR_USER_LOGIN: &str = "storage/user/login/**";
pub const KEY_EXPR_PEER_SIGNUP: &str = "storage/peer/signup/**";
pub const KEY_EXPR_PEER_GET: &str = "storage/peer/get/**";
pub const KEY_EXPR_FILE_LIST: &str = "storage/file/list/**";
pub const KEY_EXPR_FILE_UPLOAD: &str = "storage/file/upload/**";
pub const KEY_EXPR_FILE_UPLOAD_CLOUD: &str = "storage/file/upload_cloud/**";
pub const KEY_EXPR_FILE_GET: &str = "storage/file/get/**";
pub const KEY_EXPR_FILE_DELETE: &str = "storage/file/delete/**";
pub const KEY_EXPR_METRICS_PUT: &str = "storage/metrics/put/**";
pub const KEY_EXPR_METRICS_GET: &str = "storage/metrics/get/**";
pub const KEY_EXPR_PERMISSION_PUT: &str = "storage/permission/put/**";

// Subscriber
pub const KEY_EXPR_LIVENESS_PUT: &str = "storage/liveness/**";

// Publisher
pub const KEY_EXPR_FILE_DISTRIBUTION: &str = "storage/file/distribution/";
pub const KEY_EXPR_CHUNK_GET: &str = "storage/chunk/get/";

pub const PIECE_SIZE: i32 = 100_000;
//pub const CHUNK_SIZE: i32 = 1400;
pub const REPLICATION_FACTOR: i16 = 100; //100%
pub const MTU: u16 = 1492;

// Penalties
pub const PENALTY_NOT_FOUND: i8 = 1;
pub const PENALTY_CORRUPTION: i8 = 1;

//pub const DB_HOST: &str = "127.0.0.1";
//pub const DB_HOST: &str = "172.17.0.2";
pub const REPLICATION_STRATEGY_DB: &str = "SimpleStrategy"; //in real-world: NetworkTopologyStrategy
pub const REPLICATION_FACTOR_DB: &str = "1"; //in real-world: >1

// Local folder
pub const FILE_STORAGE_FOLDER: &str = "/tmp/saved_files";

// File status
pub const FILE_STATUS_TO_DISTRIBUTE: &str = "to_distribute";
pub const FILE_STATUS_READY: &str = "ready";

// Cron jobs
pub const CRON_JOB_RATING: &str = "0 * * * *"; // every hour