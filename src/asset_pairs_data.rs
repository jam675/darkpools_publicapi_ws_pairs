use crate::error;
use redis;
use redis::Client;
use redis::Commands;
use redis::Connection;
use redis::ConnectionAddr;
use redis::ConnectionInfo;
use redis::RedisError;

fn is_redis_available(
    host: String,
    port: u16,
    db_num: u8,
) -> (bool, Result<Connection, RedisError>) {
    let redis_address = ConnectionAddr::Tcp(host.clone(), port);
    let redis_connection_info = ConnectionInfo {
        addr: Box::from(redis_address),
        db: i64::from(db_num),
        passwd: None,
    };
    let redis_client = Client::open(redis_connection_info).expect("Invalid Redis URL!");
    let connection_result = redis_client.get_connection();
    (connection_result.is_ok(), connection_result)
}

fn create_redis_connection(host: String, port: u16, db_num: u8) -> Connection {
    let (is_available, conn) = is_redis_available(host.clone(), port, db_num);
    if !is_available {
        panic!(format!(
            "No Redis found in {}:{} db {}!",
            host, port, db_num
        ));
    }
    conn.unwrap()
}

pub fn get_asset_pairs_data(host: String, port: u16, db_num: u8) -> Option<String> {
    let mut redis_connection = create_redis_connection(host, port, db_num);
    let result: Result<String, RedisError> = redis_connection.get("assets");
    match result {
        Ok(v) => Some(v),
        Err(err) => {
            error!("{:?}", err);
            None
        }
    }
}
