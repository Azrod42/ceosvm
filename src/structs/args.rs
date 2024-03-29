use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}
