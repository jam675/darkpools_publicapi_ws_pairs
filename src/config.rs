use bitwyre_ws_core::get_executable_name;
use bitwyre_ws_core::get_mandatory_env_int;
use bitwyre_ws_core::get_mandatory_env_string;
use std::time::Duration;

const ENV_MAX_CLIENT: &str = "MAX_CLIENT";
const ENV_SERVICE_IP: &str = "SERVICE_IP";
const ENV_SERVICE_PORT: &str = "SERVICE_PORT";
const ENV_SERVICE_PATH: &str = "SERVICE_PATH";
const ENV_RAPID_REQUEST_LIMIT_MS: &str = "RAPID_REQUEST_LIMIT_MS";
const ENV_REDIS_HOST: &str = "REDIS_HOST";
const ENV_REDIS_PORT: &str = "REDIS_PORT";
const ENV_REDIS_DB: &str = "REDIS_DB";

#[cfg(debug_assertions)]
pub(crate) fn is_debugging() -> bool {
    true
}

#[cfg(not(debug_assertions))]
pub(crate) fn is_debugging() -> bool {
    false
}

#[derive(Clone)]
pub(crate) struct ServiceConfig {
    pub debug_build: bool,
    pub executable_name: String,
    pub max_clients: u32,
    pub service_baseurl: String,
    pub service_path: String,
    pub rapid_request_limit: Duration,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_db: u8,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let max_clients = get_mandatory_env_int(ENV_MAX_CLIENT) as u32;
        let service_baseurl = format!(
            "{}:{}",
            get_mandatory_env_string(ENV_SERVICE_IP),
            get_mandatory_env_int(ENV_SERVICE_PORT)
        );
        let service_path = get_mandatory_env_string(ENV_SERVICE_PATH);
        let rapid_request_limit =
            Duration::from_millis(get_mandatory_env_int(ENV_RAPID_REQUEST_LIMIT_MS) as u64);
        let redis_host = get_mandatory_env_string(ENV_REDIS_HOST);
        let redis_port = get_mandatory_env_int(ENV_REDIS_PORT) as u16;
        let redis_db = get_mandatory_env_int(ENV_REDIS_DB) as u8;
        Self {
            debug_build: is_debugging(),
            executable_name: get_executable_name(),
            max_clients,
            service_baseurl,
            service_path,
            rapid_request_limit,
            redis_host,
            redis_port,
            redis_db,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_creation_is_successful() {
        let max_clients: u32 = 2000;
        let ping_limit_ms: u64 = 1000;
        let bind_ip = "127.0.0.1";
        let bind_port = 7000;
        let bind_path = "/public/time";
        let bind_baseurl = format!("{}:{}", bind_ip, bind_port);
        let rapid_request_limit = Duration::from_millis(ping_limit_ms);
        let redis_host = "redis-master";
        let redis_port = 6379 as u16;
        let redis_db = 66 as u8;
        env::set_var(ENV_MAX_CLIENT, max_clients.to_string());
        env::set_var(ENV_SERVICE_IP, bind_ip);
        env::set_var(ENV_SERVICE_PORT, bind_port.to_string());
        env::set_var(ENV_SERVICE_PATH, bind_path);
        env::set_var(ENV_RAPID_REQUEST_LIMIT_MS, ping_limit_ms.to_string());
        env::set_var(ENV_REDIS_HOST, redis_host);
        env::set_var(ENV_REDIS_PORT, redis_port.to_string());
        env::set_var(ENV_REDIS_DB, redis_db.to_string());
        let config: ServiceConfig = Default::default();
        assert_eq!(config.max_clients, max_clients);
        assert_eq!(config.service_baseurl, bind_baseurl);
        assert_eq!(config.service_path, bind_path);
        assert_eq!(config.rapid_request_limit, rapid_request_limit);
        assert_eq!(config.redis_host, redis_host);
        assert_eq!(config.redis_port, redis_port);
        assert_eq!(config.redis_db, redis_db);
    }
}
