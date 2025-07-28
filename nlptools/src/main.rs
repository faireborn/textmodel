use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Char { command: Option<String> },
    Word { command: Option<String> },
    Sent { command: Option<String> },
    Doc { command: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(subcommand) => match subcommand {
            Commands::Char { command } => {
                println!("{}", command.clone().unwrap())
            }
            Commands::Word { command } => {
                println!("{}", command.clone().unwrap())
            }
            Commands::Sent { command } => {
                println!("{}", command.clone().unwrap())
            }
            Commands::Doc { command } => {
                println!("{}", command.clone().unwrap())
            }
        },
        None => {
            println!("Default subcommand");
        }
    }
}
