use std::{ path::PathBuf};
use std::net::SocketAddr;
use axum::routing::get;
use tracing::info;
use axum::{Router};
use anyhow::{Result};


pub async fn process_httpserve(dir: &PathBuf, port: i16) -> Result<()> {
    info!("Serving {:?} on port {}", dir,  port);
    // axum router
    let router = Router::new()
        .route("/", get(index_handle));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); 
    let listen = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Serving {:?} on addr {:?}", dir,  addr);

    axum::serve(listen, router).await.unwrap();
    Ok(())
}

async fn index_handle() -> &'static str {
    "hello world"
}