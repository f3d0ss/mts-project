use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The chain where the script will listen for the NFT metadata
    #[arg(short, long)]
    chain: String,

    /// The controller address
    #[arg(short, long)]
    address: String,
}

pub fn parse_args() -> (String, String) {
    let args = Args::parse();

    (args.chain, args.address)
}
