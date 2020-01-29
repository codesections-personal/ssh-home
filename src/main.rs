use clap::{crate_name, crate_version, App, ArgMatches};
use d5_cli::D5;
use ssh_home::SshHomeBuilder;
use std::error::Error;
use utils::{BoxedErr, Die};

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

    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    let ip: std::net::Ipv4Addr = match cli.value_of("ip") {
        Some(ip) => ip
            .parse()
            .map_err(|_| BoxedErr::new(&format!("{} is not a valid IP address", ip)))?,
        None => D5::new().with_password(cli.value_of("pass")).try_ip()?,
    };

    let mut ssh_home = SshHomeBuilder::new();
    ssh_home
        .ip(Some(ip))
        .command(cli.value_of("command"))
        .verbose_flag(cli.is_present("verbose"));
    if cli.is_present("verbose") {
        ssh_home.verbose_flag(true);
    }
    ssh_home.build()?.exec()?;
    Ok(())
}
