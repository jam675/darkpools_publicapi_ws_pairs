#!/usr/bin/env bash

function set_default_env() {
    if [ -z "${!1}" ]; then
        echo "INFO! Setting '$1' value to '$2'"
        export $1=$2
    fi
}

function exit_on_undefined_env() {
    if [ -z "${!1}" ]; then
        echo "ERROR! '$1' is undefined, exiting..." 1>&2
        exit 1
    fi
}

function exit_on_any_error() {
    if [ -n "$?" ] && [ "$?" -ne 0 ]; then
        echo "An error occurred, exiting..." 1>&2
        exit 1
    fi
}

# Set these environment variables to its default value if undefined
set_default_env     MAX_CLIENT              16384
set_default_env     SERVICE_IP              "0.0.0.0"
set_default_env     SERVICE_PORT            4001
set_default_env     SERVICE_PATH            "/ws/public/pairs"
set_default_env     RAPID_REQUEST_LIMIT_MS  100
set_default_env     REDIS_HOST              "localhost"
set_default_env     REDIS_PORT              6379
set_default_env     REDIS_DB                12

# Cargo run switch
case "$1" in
formatter)
    cargo fmt --all
    ;;
test)
    cargo test
    ;;
lint)
    cargo clippy --all-targets --all-features -- -D warnings --verbose
    ;;
'')
    cargo run
    ;;
audit)
    cargo audit
    ;;
*)
    echo "Invalid parameter $1!" 1>&2
    echo "Usage: $0 [formatter|test|lint|memcheck|audit]"
    exit 1
    ;;
esac
