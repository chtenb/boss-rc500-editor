use clap::Parser;
mod editor;
mod exit_codes;
mod io;
mod model;
mod reader;
mod writer;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Pull {
        working_dir: String,
        /// If the working directory already contains a config file, overwrite it
        #[clap(short, long)]
        overwrite: bool,
    },
    Push,
    Read{
        filename: String,
    },
    Write,
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Command::Pull {
            working_dir,
            overwrite: _,
        } => io::pull(&working_dir),
        Command::Push => {
            Err("Not implemented".to_string())
        },
        Command::Read{filename} => {
            let config = reader::read(&filename);
            Ok(())
        },
        Command::Write => {
            Err("Not implemented".to_string())
        }
    };
    match result {
        Err(e) => {
            println!("{}", e);
            std::process::exit(exit_codes::ERROR)
        },
        Ok(()) => std::process::exit(exit_codes::OK),
    };
}
