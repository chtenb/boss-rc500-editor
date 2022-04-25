use clap::Parser;
mod arith;
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
    /// Pull the configuration settings from a connected rc500 to the given directory
    Pull {
        working_dir: String,
        /// If the working directory already contains a config file, overwrite it
        #[clap(short, long)]
        overwrite: bool,
    },
    /// Push the configuration settings in the given directory to a connected rc500
    Push { working_dir: String },
    /// Interactively edit the configuration settings
    Edit { filename: String },
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Command::Pull {
            working_dir,
            overwrite: _,
        } => io::pull(&working_dir),
        Command::Push { working_dir } => io::push(&working_dir),
        Command::Edit { filename } => {
            let res = reader::read(&filename)
                .and_then(|mut config| editor::init(&mut config).map_err(|e| format!("{:?}", e)));
            if let Err(err) = res {
                println!("{:?}", err)
            }
            Ok(())
        }
    };
    match result {
        Err(e) => {
            println!("{}", e);
            std::process::exit(exit_codes::ERROR)
        }
        Ok(()) => std::process::exit(exit_codes::OK),
    };
}
