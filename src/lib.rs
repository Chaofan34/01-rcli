mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, CsvOpts, GenPassOpts, HttpSubCommand, Opts, SubCommand,
    TextSignFormat, TextSubCommand,
};
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
