#[macro_use]
extern crate log;
extern crate env_logger;
extern crate pdf_generator;

use env_logger::{Builder, Target};
use std::env;
use pdf_generator::service::ReportService;
use pdf_generator::routes::mount_routes;

fn main() {
    init_logger();
    info!("Starting pdf-generator...");

    match ReportService::new() {
        Ok(s) => {
            let error = mount_routes(s).launch();
            drop(error);
        }
        Err(e) => {
            error!("Failed to start pdf-generator service, error: {:?}", e);
            panic!(e)
        }
    }
}

fn init_logger() {
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    env::var("RUST_LOG").iter().for_each(|s| { builder.parse(s.as_str()); });
    builder.init();
}