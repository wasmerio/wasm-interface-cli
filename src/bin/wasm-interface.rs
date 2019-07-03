use structopt::StructOpt;
use wasm_interface_cli::commands;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "check")]
    Check(commands::CheckOpt),
}

fn main() {
    let args = Command::from_args();
    let result = match args {
        Command::Check(check_opt) => commands::check(check_opt),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(-1);
    }
}
