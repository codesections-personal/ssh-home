use clap::App;
use run_script::{run_script, ScriptOptions};

fn main() {
    let matches = App::new("ssh-home")
        .arg("-v, --verbose 'Enables verbose logging'")
        .arg("-p, --port=[PORT] 'Connect to PORT rather than the default")
        .get_matches();

    let (_, mut pw, _) = run_script!(r##"echo "" | dmenu -p "Password: " -nf "#222222""##).unwrap();
    pw.pop();

    let (_, mut ip, _) = run_script!(format!(
        r#"curl -u dsock:{} https://d5.codesections.com"#,
        pw
    ))
    .unwrap();
    ip.pop();

    println!("Pass: {}\nIP: {}", pw, ip);

    let mut options = ScriptOptions::new();
    options.capture_output = false; // False will print it to the parent process output.
    run_script!(
        format!(
            r#"ssh -A -l dsock -R {}\
               {}\
                -p {}"#,
            "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra",
            ip,
            matches.value_of("port").unwrap_or("2222")
        ),
        &options
    )
    .unwrap();
}
