use std::{error::Error, net::Ipv4Addr, os::unix::process::CommandExt, process::Command};
use utils::dependencies;

#[derive(Debug)]
pub struct SshHome<'a> {
    ip: Ipv4Addr,
    pub port: &'a str,
    pub user: &'a str,
    socket: &'a str,
    pub verbose: bool,
    pub command: Option<&'a str>,
}

impl<'a> SshHome<'a> {
    pub fn new(ip: Ipv4Addr) -> Self {
        Self {
            ip,
            port: "2222",
            user: "dsock",
            socket: "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra",
            verbose: false,
            command: None,
        }
    }

    pub fn exec(self) -> Result<(), Box<dyn Error>> {
        self.build_cmd()?.exec();
        Ok(())
    }
    pub fn run(self) -> Result<(String, String), Box<dyn Error>> {
        let output = self.build_cmd()?.output()?;
        if output.status.success() {
            Ok((
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
            ))
        } else {
            Err(String::from_utf8(output.stderr)?.into())
        }
    }
    fn build_cmd(self) -> Result<Command, Box<dyn Error>> {
        dependencies(vec!["ssh"])?;

        let mut cmd = Command::new("ssh");
        cmd.arg("-A")
            .args(&["-l", &self.user])
            .args(&["-R", &self.socket])
            .args(&["-p", &self.port])
            .arg(self.ip.to_string());
        if self.verbose {
            cmd.arg("-vvv");
        }
        if let Some(command) = self.command {
            cmd.arg(command);
        }
        Ok(cmd)
    }
}
