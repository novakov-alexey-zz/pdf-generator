extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate pdf_generator;

use dotenv::dotenv;
use env_logger::{Builder, Target};
// use std::env;
use pdf_generator::service::ReportService;
use pdf_generator::routes::mount_routes;

fn main() {
    dotenv().ok();
    init_logger();
    info!("Starting pdf-generator...");

    match ReportService::new() {
        Ok(s) => {
            let error = mount_routes(s).launch();
            drop(error);
        }
        Err(e) => {
            error!("Failed to start pdf-generator service, error: {:?}", e);
            panic!("{:?}", e)
        }
    }
}

fn init_logger() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}