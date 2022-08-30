use crate::node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::{from_str, to_string_pretty};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Registry {
    pub nodes: HashMap<String, Node>,
}

impl Default for Registry {
    fn default() -> Self {
        let nodes: HashMap<String, Node> = HashMap::new();
        Self { nodes }
    }
}

impl Registry {
    pub fn gen_config_file(&self) {
        #[cfg(debug_assertions)]
        let reg = "nodemgr.toml";
        #[cfg(not(debug_assertions))]
        let reg = "/etc/nodemgr.toml";

        let mut file = File::create(reg).unwrap();
        let data = to_string_pretty(&self.clone()).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        file.sync_all().unwrap();
        file.flush().unwrap();
    }

    pub fn export(&self, file: &str) {
        let mut file = File::create(file).unwrap();
        let data = to_string_pretty(&self.clone()).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        file.sync_all().unwrap();
        file.flush().unwrap();
    }

    pub fn import_toml(data: &str) -> Self {
        let value: Registry = from_str(data).unwrap();
        value
    }

    pub fn config_exist() -> bool {
        #[cfg(debug_assertions)]
        let reg = "nodemgr.toml";
        #[cfg(not(debug_assertions))]
        let reg = "/etc/nodemgr.toml";

        Path::new(reg).exists()
    }

    pub fn load() -> Self {
        #[cfg(debug_assertions)]
        let reg = "nodemgr.toml";
        #[cfg(not(debug_assertions))]
        let reg = "/etc/nodemgr.toml";
        Self::from_toml(reg)
    }

    pub fn from_toml(path: &str) -> Self {
        let mut buffer = String::new();
        let file = File::open(path);

        if file.is_err() {
            println!("File not found: {}", path);
            std::process::exit(1);
        }

        file.unwrap().read_to_string(&mut buffer).unwrap();
        Self::import_toml(&buffer)
    }

    pub fn add_node(&mut self, node: &Node) {
        let name = node.name.clone();
        if let None = self.nodes.get(&node.name) {
            self.nodes.insert(name, node.clone());
            self.gen_config_file()
        } else {
            println!("Node already exists");
        }
    }

    pub fn list(&self) {
        let keys: Vec<String> = self.nodes.keys().map(|x| x.to_string()).collect();
        println!("{}", keys.join(" "))
    }

    pub fn remove_node(&mut self, name: &str) {
        let node = self.nodes.get(name);
        if let Some(node) = node {
            node.disable_service();
            self.nodes.remove(name);
            self.gen_config_file()
        } else {
            println!("Node not found");
        }
    }

    pub fn enable_all_service(&self) {
        let keys: Vec<String> = self.nodes.keys().map(|x| x.to_string()).collect();
        for key in keys.iter() {
            let value = self.nodes.get(key).clone();
            value.unwrap().gen_service_file().unwrap();
            value.unwrap().enable_service();
        }
    }

    pub fn disable_all_service(&self) {
        let keys: Vec<String> = self.nodes.keys().map(|x| x.to_string()).collect();
        for key in keys.iter() {
            let value = self.nodes.get(key).clone();
            value.unwrap().gen_service_file().unwrap();
            value.unwrap().disable_service();
        }
    }

    pub fn init() {
        let data = Self::default();
        data.gen_config_file();
    }
}
