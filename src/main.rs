#[macro_use]
extern crate lazy_static;

mod config;

pub use bitwyre_ws_core::{debug, error, info, trace, warn};

use bitwyre_ws_core::init_log;
use bitwyre_ws_core::run_reactive_websocket_service;
use bitwyre_ws_core::ReactiveWebsocketConfig;
use bitwyre_ws_core::ReactiveWebsocketState;
use config::is_debugging;
use config::ServiceConfig;
use std::io::Result as IOResult;
use std::sync::Arc;

fn get_tradeable_asset_pairs(request_string: String) -> Option<String> {
    if request_string.to_lowercase().starts_with("get") {
        return Some(
            r#"
{"pairs": ["btc_idr", "bch_idr", "eth_idr", "ltc_idr", "xmr_idr", "bch_btc", "eth_btc", "ltc_btc",
"xmr_btc", "eth_bch", "ltc_bch", "xmr_bch"]}"#
                .to_owned(),
        );
    }
    None
}

fn main() -> IOResult<()> {
    init_log(is_debugging(), None);
    lazy_static! {
        static ref CONFIG: ServiceConfig = ServiceConfig::default();
        static ref STATE: ReactiveWebsocketState =
            ReactiveWebsocketState::new(ReactiveWebsocketConfig {
                binding_url: CONFIG.service_baseurl.clone(),
                binding_path: CONFIG.service_path.clone(),
                max_clients: CONFIG.max_clients as usize,
                rapid_request_interval: CONFIG.rapid_request_interval,
                message_handler: Arc::new(&get_tradeable_asset_pairs),
            });
    }
    run_reactive_websocket_service(Arc::new(&STATE))
}
