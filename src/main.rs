use clap::Parser;
mod exit_codes;
mod io;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    working_dir: String,

    /// If the working directory already contains a config file, overwrite it
    #[clap(short, long)]
    overwrite: bool,
}

fn main() {
    let args = Args::parse();
    match io::pull(&args.working_dir) {
        Err(()) => std::process::exit(exit_codes::ERROR),
        Ok(()) => std::process::exit(exit_codes::OK),
    }
}
