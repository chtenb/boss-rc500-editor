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
        #[clap(short, long, default_value("."))]
        working_dir: String,
        /// If the working directory already contains a config file, overwrite it
        #[clap(short, long)]
        overwrite: bool,
    },
    /// Push the configuration settings in the given directory to a connected rc500.
    /// If the configuration settings are somehow invalid, the RC500 will use a backup configuration file
    /// that we do not touch. You will notice this by the fact that your changes are not applied.
    Push {
        /// Optionally specify a different working directory than the current one
        #[clap(short, long, default_value("."))]
        working_dir: String,
    },
    /// Interactively edit the configuration settings
    Edit {
        /// Optionally specify a different configuration working_dir than the default one
        #[clap(short, long, default_value("."))]
        working_dir: String,
    },
}

fn main() {
    let args = Args::parse();
    let result = match args.command {
        Command::Pull { working_dir, overwrite } => io::pull(&working_dir, overwrite),
        Command::Push { working_dir } => io::push(&working_dir),
        Command::Edit { working_dir } => {
            let res = read(&working_dir).and_then(|mut config| match editor::init(&mut config) {
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

fn read(working_dir: &str) -> Result<model::Config, String> {
    // Only use the newest of the two
    let (path1, path2) = io::config_file_paths(&working_dir);
    let config1 = reader::read(&path1)?;
    let config2 = reader::read(&path2)?;
    if config1.suffix[0] < config2.suffix[0] {
        Ok(config2)
    } else {
        Ok(config1)
    }
}
