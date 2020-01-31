use clap::{crate_name, crate_version, App, ArgMatches};
use d5_cli::D5;
use ssh_home::SshHome;
use std::error::Error;
use utils::Die;

fn main() {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("SSH into my home development server.")
        .arg("-v, --verbose 'Enables verbose logging'")
        .arg("-p, --port=[PORT] 'Connect to PORT rather than the default'")
        .arg("-P  --pass [PASSWORD] 'The password to use when getting the IP address from d5'")
        .arg("-c --command [COMMAND] 'Send command via ssh instead of connecting for interactive use'")
        .arg("--ip [IP_ADDRESS] 'The IP address to use (instead of getting it via d5)'")
        .arg("--src 'Prints this program's source to stdout'")
        .get_matches();

    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    if cli.is_present("src") {
        print!(
            "/// main.rs\n{main}\n\n/// lib.rs\n{lib}",
            main = include_str!("main.rs"),
            lib = include_str!("lib.rs")
        );
        return Ok(());
    }
    let ip: std::net::Ipv4Addr = match cli.value_of("ip") {
        Some(ip) => ip
            .parse()
            .map_err(|_| format!("{} is not a valid IP address", ip))?,
        None => {
            let mut d5 = D5::new();
            d5.password = cli.value_of("pass");
            d5.try_ip()?
        }
    };

    let mut ssh_home = SshHome::new(ip);
    if cli.is_present("verbose") {
        ssh_home.verbose = true;
    }
    if let Some(user) = cli.value_of("user") {
        ssh_home.user = user;
    }
    if let Some(port) = cli.value_of("port") {
        ssh_home.port = port;
    }
    if let Some(command) = cli.value_of("command") {
        ssh_home.command = Some(command);
    }
    ssh_home.exec()?;
    Ok(())
}
