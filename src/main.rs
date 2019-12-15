use boolinator::Boolinator;
use clap::App;
use run_script::{run_script, ScriptOptions};

fn main() {
    let matches = App::new("ssh-home")
        .arg("-v, --verbose 'Enables verbose logging'")
        .arg("-p, --port=[PORT] 'Connect to PORT rather than the default")
        .get_matches();

    let dmenu_passwd_cmd = r##"echo -n $(echo "" | dmenu -p "Password: " -nf "#222222")"##;
    let (_, password, _) = run_script!(dmenu_passwd_cmd).unwrap();

    let d5_cmd = format!("curl -u dsock:{} https://d5.codesections.com", password);
    let (_, ip, _) = run_script!(d5_cmd).unwrap();

    let mut options = ScriptOptions::new();
    options.capture_output = false; // False will print it to the parent process output.
    let verbose = matches.is_present("verbose").as_some("-vvv").unwrap_or("");
    let remote_socket = "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra";
    let port = matches.value_of("port").unwrap_or("2222");
    let curl_cmd = format!(
        "ssh -A -l dsock -R {} {} -p {} {}",
        remote_socket, ip, port, verbose,
    );
    run_script!(curl_cmd, &options).unwrap();
}
