pub mod display;

use clap::Parser;
use transfer::{
    cli::{Cli, Commands},
    receiver::handle_receive,
    sender::handle_send,
    telemetry::init_subscriber,
};

fn main() {
    let cli = Cli::parse();

    init_subscriber(cli.debug);

    match cli.command {
        Commands::Send(args) => {
            handle_send(args.target, &args.file);
        }
        Commands::Receive(args) => {
            let addr = format!("127.0.0.1:{}", args.port);
            handle_receive(addr, &args.output_dir);
        }
    }
}
