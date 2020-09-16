extern crate chrono;
extern crate rayon;

mod simulator;

mod club;
mod continent;
mod country;
mod league;
mod r#match;
mod server;
mod transfers;

mod shared;
mod utils;

mod generators;

use club::*;
use country::*;

use crate::server::Server;
use crate::utils::TimeEstimation;
use crate::simulator::{FootballSimulator, SimulatorData};

// #[global_allocator]
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[actix_web::main]
async fn main() {
    let server = Server::new("0.0.0.0:18000");

    server.start().await;
}
