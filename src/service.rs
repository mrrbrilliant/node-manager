use crate::node::Node;
use crate::registry::Registry;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::process::{Command, Stdio};

impl Node {
    pub fn gen_service(&self) -> String {
        let template = format!(
            r#"[Unit]
Description=Selendra@{name}
After=network.target
Documentation=https://github.com/selendra/selendra

[Service]
ExecStart=/usr/bin/selendra --base-path {path} --chain testnet --port 30333 --rpc-port 9934 --ws-port 9944 --prometheus-port 9616 --rpc-methods Unsafe --rpc-cors all --pruning archive --validator --name "{name}" --bootnodes /ip4/157.245.56.213/tcp/30333/p2p/12D3KooWDLR899Spcx4xJ3U8cZttv9zjzJoey3HKaTZiNqwojZJB
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
"#,
            name = self.name.to_case(Case::Title),
            path = self.path
        );

        template
    }

    pub fn service_name(&self) -> String {
        format!("selendra.{}.service", &self.name.clone())
    }

    pub fn service_file(&self) -> String {
        #[cfg(debug_assertions)]
        {
            format!("selendra.{}.service", &self.name.clone())
        }

        #[cfg(not(debug_assertions))]
        {
            format!(
                "/etc/systemd/system/selendra.{}.service",
                &self.name.clone()
            )
        }
    }

    pub fn gen_service_file(&self) -> Result<()> {
        let data = Self::gen_service(&self);
        let file_path = self.service_file();
        let mut file = File::create(file_path)?;
        file.write_all(data.as_bytes())
    }

    pub fn is_enabled(&self) -> bool {
        let cmd = Command::new("systemctl")
            .args(&vec!["is-enable", &self.service_name()])
            .stdout(Stdio::piped())
            .output()
            .expect("Could not check service");

        let status = cmd.status.success();

        if !status {
            return false;
        }

        let buffer = String::from_utf8(cmd.stdout).unwrap();

        if buffer != "enabled" {
            return false;
        }

        true
    }

    pub fn enable_service(&self) {
        if self.is_enabled() {
            ()
        }

        let gen = self.gen_service_file();

        if let Err(e) = gen {
            println!("{}", e.kind().to_string())
        }

        let mut cmd = Command::new("sudo")
            .args(&vec!["systemctl", "enable", "--now", &self.service_name()])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to enable service");
        cmd.wait().unwrap();
    }

    pub fn disable_service(&self) {
        if !self.is_enabled() {
            ()
        }

        let mut cmd = Command::new("sudo")
            .args(&vec!["systemctl", "disable", "--now", &self.service_name()])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to enable service");
        cmd.wait().unwrap();

        let mut rem = Command::new("sudo")
            .args(&vec!["rm", "-rf", "--now", &self.service_file()])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to enable service");
        rem.wait().unwrap();
    }
}

impl Registry {}
