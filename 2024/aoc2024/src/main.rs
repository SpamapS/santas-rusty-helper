pub mod day1;
pub mod day2;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does day1 things
    Day1 {},
    Day2 {},
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Day1 {}) => {
            day1::day1(None);
            day1::day1(Some("2024d1p1.txt".into()));
        }
        Some(Commands::Day2 {}) => {
            day2::day2(None);
        }
        None => {}
    }

    // Continued program logic goes here...
}
