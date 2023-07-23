mod generate;
mod init;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {},
    Init { scaffold: String, name: String },
    NewScaff { name: String, crates: Vec<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name, scaffold } => init::init_rs_proj(name.clone(), scaffold.clone()),
        Commands::Generate {} => generate::initial_startup(),
        Commands::NewScaff { name, crates } => {
            init::init_scaff(name.clone(), crates.clone()).expect("Could not initialize")
        }
    }
}
