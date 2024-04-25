use std::path::PathBuf;

use crate::CmdExector;

use super::verify_path;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "serve a http request")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: i16,
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => {
                crate::process_httpserve(&opts.dir, opts.port)
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }
}
