use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::flag;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn signal_handler() -> Arc<AtomicBool> {
    let term = Arc::new(AtomicBool::new(false));

    // register for SIGTERM and SIGINT
    flag::register(SIGTERM, Arc::clone(&term)).unwrap();
    flag::register(SIGINT, Arc::clone(&term)).unwrap();

    term
}