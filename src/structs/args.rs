use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct ListVersionsParams {
    /// Filter by tenant
    #[clap(short, long)]
    pub tenant: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct CreateVersionParams {
    #[clap(short, long)]
    pub tenant: String,

    ///Should end by semver
    #[clap(short, long)]
    pub version: String,

    ///checksum
    #[clap(short, long)]
    pub md5: String,

    ///S3 firmware path
    #[clap(short, long)]
    pub path: String,
}

#[derive(Args, Debug, Clone)]
pub struct SetStableParams {
    #[clap(short, long)]
    pub tenant: String,

    ///Should end by semver
    #[clap(short, long)]
    pub version: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CeosFirmware {
    ///List all ceos firmware versions
    List(ListVersionsParams),

    ///Create ceos firmware version
    CreateVersion(CreateVersionParams),

    ///Set a specific version stable
    SetStable(SetStableParams),
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    ///Manage ceos firmware versions
    #[clap(subcommand)]
    CeosFirmware(CeosFirmware),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(propagate_version = false)]
pub struct CliArgs {
    ///Remove auth cache
    #[arg(short, long, default_value_t = false)]
    pub remove_cache: bool,

    #[command(subcommand)]
    pub cmd: Command,
}
