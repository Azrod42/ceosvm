use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long, default_value_t = false)]
    list: bool,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.list)
    }
    Ok(())
}
