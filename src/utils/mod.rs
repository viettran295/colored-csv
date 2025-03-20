use std::env;
use env_logger::builder;

pub fn init() {
    unsafe {
        env::set_var("RUST_LOG", "trace");
    }
    builder()
        .format_timestamp(None)
        .init();
}