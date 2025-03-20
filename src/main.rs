use clap::{Arg, ArgAction, Command};
use port_claim::{kill_port_process, port_available};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .about("A tool to check if ports are in use and kill processes using them")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Prints verbose information"),
        )
        .arg(
            Arg::new("ports")
                .num_args(1..)
                .value_name("PORT")
                .help("Ports to check and kill if in use"),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");

    // Get ports from arguments
    if let Some(ports) = matches.get_many::<String>("ports") {
        for port_str in ports {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    if !port_available(port, verbose) {
                        kill_port_process(port, verbose);
                    }
                }
                Err(_) => {
                    eprintln!("Error: '{}' is not a valid port number", port_str);
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Error: No ports specified");
        std::process::exit(1);
    }
}
