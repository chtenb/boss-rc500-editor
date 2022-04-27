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
        /// Optionally specify a different working directory than the current one
        #[clap(short, long, default_value("rc500.xml"))]
        filename: String,
        /// If the working directory already contains a config file, overwrite it
        #[clap(short, long)]
        overwrite: bool,
    },
    /// Push the configuration settings in the given directory to a connected rc500.
    /// If the configuration settings are somehow invalid, the RC500 will use a backup configuration file
    /// that we do not touch. You will notice this by the fact that your changes are not applied.
    Push {
        /// Optionally specify a different working directory than the current one
        #[clap(short, long, default_value("rc500.xml"))]
        filename: String,
    },
    /// Interactively edit the configuration settings
    Edit {
        /// Optionally specify a different configuration filename than the default one
        #[clap(short, long, default_value("rc500.xml"))]
        filename: String,
    },
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Command::Pull { filename, overwrite } => io::pull(&filename, overwrite),
        Command::Push { filename } => io::push(&filename),
        Command::Edit { filename } => {
            let res = reader::read(&filename).and_then(|mut config| match editor::init(&mut config) {
                Err(e) => Err(format!("{:?}", e)),
                Ok(()) => Ok(()),
            });
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
