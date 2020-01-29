use clap::{crate_name, crate_version, App, ArgMatches};
use d5_cli::D5;
use std::{error::Error, os::unix::process::CommandExt, process::Command};
use utils::{dependencies2, BoxedErr, CommandUtils, Die};

fn main() {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("SSH into my home development server.")
        .arg("-v, --verbose 'Enables verbose logging'")
        .arg("-p, --port=[PORT] 'Connect to PORT rather than the default'")
        .arg("-P  --pass [PASSWORD] 'The password to use when getting the IP address from d5'")
        .arg("-c --command [COMMAND] 'Send command via ssh instead of connecting for interactive use'")
        .arg("--ip [IP_ADDRESS] 'The IP address to use (instead of getting it via d5)'")
        .get_matches();
    dependencies2(vec!["ssh"]).unwrap_or_die();

    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    let ip: std::net::Ipv4Addr = match cli.value_of("ip") {
        Some(ip) => ip
            .parse()
            .map_err(|_| BoxedErr::new(&format!("{} is not a valid IP address", ip)))?,
        None => D5::new().with_password(cli.value_of("pass")).try_ip()?,
    };

    let mut ssh_command = Command::new("ssh");
    let remote_socket = "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra";

    ssh_command
        .flag("-A")
        .option(&["-l", "dsock"])
        .option(&["-R", remote_socket])
        .option(&["-p", cli.value_of("port").unwrap_or("2222")])
        .arg(ip.to_string());
    if cli.is_present("verbose") {
        ssh_command.flag("-vvv");
    }
    if let Some(command) = cli.value_of("command") {
        ssh_command.arg(command);
    }
    ssh_command.exec();
    Ok(())
}
