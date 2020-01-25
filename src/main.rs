use clap::{crate_name, crate_version, App};
use std::{os::unix::process::CommandExt, process::Command};
use utils::{dependencies, sh, Die};

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
    dependencies!(&["ssh"]);
    let ip = cli.value_of("ip").map(String::from).unwrap_or_else(|| {
        dependencies!(&["d5"]);
        sh(&match cli.value_of("pass") {
            Some(password) => format!("d5 --pass {}", password),
            None => format!("d5"),
        })
    });
    let ip = ip.parse::<std::net::Ipv4Addr>().unwrap_or_die();
    fn shell(cmd: &str) -> Command {
        let mut words = cmd.split(char::is_whitespace);
        let mut cmd = Command::new(words.nth(0).unwrap());
        for word in words {
            if !word.is_empty() {
                cmd.arg(word);
            }
        }
        cmd
    }

    let mut cmd = shell(&format!(
        "ssh -A -l dsock -R {remote_socket} {ip} -p {port} {verbose} {cmd}",
        remote_socket = "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra",
        ip = ip,
        port = cli.value_of("port").unwrap_or("2222"),
        verbose = match cli.is_present("verbose") {
            true => "-vvv",
            false => "",
        },
        cmd = cli.value_of("command").unwrap_or_default(),
    ));
    cmd.exec();
    // let mut cmd = Command::new("ssh");
    // cmd.arg("-A")
    //     .arg("-l")
    //     .arg("dsock")
    //     .arg("-R")
    //     .arg("/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra")
    //     .arg(ip.to_string())
    //     .arg("-p")
    //     .arg(cli.value_of("port").unwrap_or("2222"));
    // if cli.is_present("verbose") {
    //     cmd.arg("-vvv");
    // }
    // if let Some(command) = cli.value_of("command") {
    //     cmd.arg(command);
    // }
    // cmd.exec();

    //    let mut options = ScriptOptions::new();
    //    options.output_redirection = IoOptions::Inherit; // prints output to the parent process output.
    // run_script!(
    //     format!(
    //         "ssh -A -l dsock -R {remote_socket} {ip} -p {port} {verbose} '{cmd}'",
    //         remote_socket =
    //             "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra",
    //         ip = ip,
    //         port = cli.value_of("port").unwrap_or("2222"),
    //         verbose = cli.is_present("verbose").as_some("-vvv").unwrap_or(""),
    //         cmd = cli.value_of("command").unwrap_or_default(),
    //     ),
    //     &options
    // )
    // .unwrap_or_die();
}
