#[macro_use]
extern crate lazy_static;

mod asset_pairs_data;
mod config;

pub use bitwyre_ws_core::{debug, error, info, trace, warn};

use asset_pairs_data::get_asset_pairs_data;
use bitwyre_ws_core::init_log;
use bitwyre_ws_core::run_reactive_websocket_service;
use bitwyre_ws_core::ReactiveWebsocketConfig;
use bitwyre_ws_core::ReactiveWebsocketState;
use config::is_debugging;
use config::ServiceConfig;
use std::io::Result as IOResult;
use std::sync::Arc;

lazy_static! {
    static ref CONFIG: ServiceConfig = ServiceConfig::default();
    static ref STATE: ReactiveWebsocketState =
        ReactiveWebsocketState::new(ReactiveWebsocketConfig {
            binding_url: CONFIG.service_baseurl.clone(),
            binding_path: CONFIG.service_path.clone(),
            max_clients: CONFIG.max_clients as usize,
            rapid_request_limit: CONFIG.rapid_request_limit,
            message_handler: Arc::new(&get_tradeable_asset_pairs),
        });
}

fn get_tradeable_asset_pairs(request_string: String) -> Option<String> {
    if request_string.to_lowercase().starts_with("get") {
        return get_asset_pairs_data(
            CONFIG.redis_host.clone(),
            CONFIG.redis_port,
            CONFIG.redis_db,
        );
    }
    None
}

fn main() -> IOResult<()> {
    init_log(is_debugging(), None);
    run_reactive_websocket_service(Arc::new(&STATE))
}
