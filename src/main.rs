use clap::Parser;
mod arith;
mod descriptions;
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
    /// Optionally specify a different working directory than the current one
    #[clap(short, long, default_value("."))]
    working_dir: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Pull the configuration settings from a connected rc500 to the given directory
    Pull {
        /// If the working directory already contains a config file, overwrite it
        #[clap(short, long)]
        overwrite: bool,
    },
    /// Push the configuration settings in the given directory to a connected rc500.
    /// If the configuration settings are somehow invalid, the RC500 will use a backup configuration file
    /// that we do not touch. You will notice this by the fact that your changes are not applied.
    Push {},
    /// Interactively edit the configuration settings
    Edit {},
}

fn main() {
    let args = Args::parse();
    match run(args) {
        Err(e) => {
            println!("{}", e);
            std::process::exit(exit_codes::ERROR)
        }
        Ok(msg) => {
            println!("{}", msg);
            std::process::exit(exit_codes::OK)
        }
    };
}

fn run(args: Args) -> Result<String, String> {
    match args.command {
        Command::Pull { overwrite } => {
            io::print_devices()?;
            io::pull(&args.working_dir, overwrite)
        }
        Command::Push {} => {
            io::print_devices()?;
            io::push(&args.working_dir)
        }
        Command::Edit {} => {
            let mut config = read(&args.working_dir)?;
            match editor::editor(&mut config, &args.working_dir) {
                Err(e) => Err(format!("{:?}", e)),
                Ok(()) => Ok("Exiting editor".to_string()),
            }
        }
    }
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
