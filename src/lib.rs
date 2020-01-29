use derive_builder::Builder;
use std::{error::Error, net::Ipv4Addr, os::unix::process::CommandExt, process::Command};
use utils::{dependencies2, BoxedErr, CommandUtils};

#[derive(Builder, Debug)]
#[builder(default)]
pub struct SshHome<'a> {
    ip: Option<Ipv4Addr>,
    pub port: String,
    username: String,
    remote_socket: String,
    verbose_flag: bool,
    command: Option<&'a str>,
}
impl SshHomeBuilder<'_> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> SshHome<'a> {
    fn build_cmd(self) -> Command {
        let mut cmd = Command::new("ssh");
        cmd.flag(Some("-A"))
            .option(Some(&["-l", &self.username]))
            .option(Some(&["-R", &self.remote_socket]))
            .option(Some(&["-p", &self.port]))
            .positional_arg(self.ip.map(|addr| addr.to_string()))
            .flag(match self.verbose_flag {
                true => Some("-vvv"),
                false => None,
            })
            .positional_arg(self.command.map(String::from));
        cmd
    }
    pub fn exec(self) -> Result<(), Box<dyn Error>> {
        dependencies2(vec!["ssh"])?;
        self.build_cmd().exec();
        Ok(())
    }

    pub fn run(self) -> Result<(String, String), Box<dyn Error>> {
        dependencies2(vec!["ssh"])?;
        let output = self.build_cmd().output()?;
        if output.status.success() {
            Ok((
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
            ))
        } else {
            Err(BoxedErr::new(&String::from_utf8(output.stderr)?))
        }
    }
    // pub fn with_ip(self, ip: Ipv4Addr) -> Self {
    //     Self {
    //         ip: Some(ip),
    //         ..self
    //     }
    // }

    // pub fn new() -> Self {
    //     Self::default()
    // }
    // pub fn with_verbose_flag(self, verbose_flag: bool) -> Self {
    //     Self {
    //         verbose_flag,
    //         ..self
    //     }
    // }
    // pub fn with_port(self, port: Option<&str>) -> Self {
    //     if let Some(port) = port {
    //         Self {
    //             port: port.to_string(),
    //             ..self
    //         }
    //     } else {
    //         self
    //     }
    // }
    // pub fn with_username(self, username: Option<&str>) -> Self {
    //     if let Some(username) = username {
    //         Self {
    //             username: username.to_string(),
    //             ..self
    //         }
    //     } else {
    //         self
    //     }
    // }
    // pub fn with_command(self, command: Option<&'a str>) -> Self {
    //     if let Some(command) = command {
    //         Self {
    //             command: Some(command),
    //             ..self
    //         }
    //     } else {
    //         self
    //     }
    // }
}
impl<'a> Default for SshHome<'a> {
    fn default() -> Self {
        Self {
            ip: None,
            port: "2222".to_string(),
            username: "dsock".to_string(),
            remote_socket:
                "/run/user/1000/gnupg/S.gpg-agent:/run/user/1000/gnupg/S.gpg-agent.extra"
                    .to_string(),
            verbose_flag: false,
            command: None,
        }
    }
}
