use clap::Parser;
use structs::args::Args;

use crate::parser::pars_args;
mod api;
mod auth;
mod file;
mod parser;
mod structs;
mod utils;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let auth_data = auth::get_auth_token().await.unwrap();
    pars_args(args, auth_data);
    Ok(())
}
