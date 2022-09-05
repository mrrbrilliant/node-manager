use std::path::Path;
use std::process::{Command, Stdio};
use crate::download::{download_file, continue_file};

pub fn download() {
    let version = get_latest();
    #[cfg(debug_assertions)]
    let bin_path = "./selendra";
    #[cfg(not(debug_assertions))]
    let bin_path = "/usr/bin/selendra";

    println!("Downloading Selendra version: {}", version);


    match download_file(format!("https://github.com/selendra/selendra/releases/download/{version}/selendra", version = version).as_ref(), continue_file(bin_path)) {
        Ok(_) => {},
        Err(t) => println!("Error: {}", t),
    };

    Command::new("chmod").args(&["+x", bin_path]).output().expect("Failed to change selendra permission");
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
