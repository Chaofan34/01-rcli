use std::path::PathBuf;

use clap::Parser;
use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "serve a http request")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t=8080)]
    pub port: i16,
}

