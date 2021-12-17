use env_logger::Builder;

pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day16;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;

use log::LevelFilter;
use std::sync::Once;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Trace;

pub fn init_logger() {
    LOGGER.call_once(|| init_logger_impl());
}

static LOGGER: Once = Once::new();

fn init_logger_impl() {
    match std::env::var(env_logger::DEFAULT_FILTER_ENV).ok() {
        Some(_) => {
            // RUST_LOG exists; env_logger will use it.
            Builder::from_default_env().init();
        }
        None => {
            // RUST_LOG does not exist; use default log level for this crate only.
            Builder::new()
                .filter(Some(env!("CARGO_CRATE_NAME")), DEFAULT_LOG_LEVEL)
                .init();
        }
    }
}
