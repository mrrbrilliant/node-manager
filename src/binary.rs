use std::path::Path;
use std::process::{Command, Stdio};

pub fn download() {
    let version = get_latest();
    #[cfg(debug_assertions)]
    let bin_path = ".";
    #[cfg(not(debug_assertions))]
    let bin_path = "/usr/bin";

    println!("Downloading Selendra version: {}", version);
    let commands = format!(
        r#"#!bin/bash
sudo wget https://github.com/selendra/selendra/releases/download/{version}/selendra -P {bin_path} -q --show-progress --progress=bar:force:noscroll &&
sudo chmod +x {bin_path}/selendra
"#,
        version = version,
        bin_path = bin_path
    );
    let mut cmd = Command::new("sh")
        .args(&vec!["-e", "-c", commands.as_ref()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait();

    if let Err(error) = status {
        println!("{}", error.kind().to_string())
    }
}

// git ls-remote --refs --sort="-version:refname" --symref  --tags https://github.com/paritytech/polkadot.git
fn get_latest() -> String {
    println!("Check for latest release at Selendra Github!");
    let cmd = Command::new("git")
        .args(&vec![
            "ls-remote",
            "--refs",
            "--tags",
            "https://github.com/selendra/selendra.git",
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let buffer = String::from_utf8(cmd.stdout).unwrap();
    let latest_line = buffer.lines().last().unwrap();
    let refs: String = latest_line.split_whitespace().last().unwrap().to_string();
    let version = refs.split("/").last().unwrap().to_string();
    version
}

pub fn exists() -> bool {
    Path::new("/usr/bin/selendra").exists()
}
