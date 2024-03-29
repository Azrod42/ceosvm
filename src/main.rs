use clap::Parser;
use structs::args::CliArgs;

use crate::parser::pars_args;
mod api;
mod auth;
mod file;
mod parser;
mod structs;
mod utils;

const BASE_URL: &'static str = "https://api.scmegahub.com/v1";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = CliArgs::parse();
    let auth_data = auth::get_auth_tokens(&args.remove_cache).await.unwrap();
    pars_args(args, auth_data).await;
    Ok(())
}
