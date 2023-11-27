pub mod client;
pub mod server;
use clap::{Args, Parser, Subcommand};
use client::Client;
use server::Server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(Args)]
struct ServerArgs {
    #[arg(short, long)]
    address: String,
}

#[derive(Args)]
struct ClientArgs {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    address: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server(args) => {
            let s = Server::new(args.address);
            s.serve().unwrap();
        }
        Commands::Client(args) => {
            let c = Client::new(args.name, args.address);
            c.connect().unwrap();
        }
    }
}
