use std::{ path::PathBuf};

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;
use anyhow::Result;


pub fn process_httpserve(dir: &PathBuf, port: i16) -> Result<()> {
    info!("Serving {:?} on port {}", dir,  port);
    // axum router
    // let router = Router::new()
    //     .route("/", get(index_handle));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); 
    info!("Serving {:?} on addr {:?}", dir,  addr);

    Ok(())
}

async fn index_handle() -> &'static str {
    "hello world"
}