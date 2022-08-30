pub mod binary;
pub mod node;
pub mod registry;
pub mod service;

use std::str::FromStr;

use clap::{command, ArgMatches, Command};

fn main() {
    let cmd = Command::new("nodemgr")
        .bin_name("nodemgr")
        .subcommand_required(true)
        .subcommands(vec![
            command!("add").help("Add new Selendra node").args(&[
                clap::arg!(--"name" <PATH>)
                    .required(true)
                    .help("Node name to be created")
                    .value_parser(clap::value_parser!(String)),
                clap::arg!(--"path" <PATH>)
                    .required(true)
                    .help("Path to store node database")
                    .value_parser(clap::value_parser!(String)),
                clap::arg!(--"chain" <PATH>)
                    .required(true)
                    .help("Chain name: mainnet or testnet")
                    .value_parser(clap::value_parser!(String)),
            ]),
            command!("export").args(&[clap::arg!(--"file" <PATH>)
                .required(true)
                .value_parser(clap::value_parser!(String))]),
            command!("import").args(&[clap::arg!(--"file" <PATH>)
                .required(true)
                .value_parser(clap::value_parser!(String))]),
            command!("init"),
            command!("list"),
            command!("remove").args(&[clap::arg!(--"name" <PATH>)
                .required(true)
                .value_parser(clap::value_parser!(String))]),
            command!("start"),
            command!("stop").help("Stop all running nodes"),
        ]);
    let matches = cmd.get_matches();

    if let Some((init, arg)) = matches.subcommand() {
        if init == "init" {
            handle_init()
        }

        if init == "import" {
            handle_import(arg)
        }
    }

    if !registry::Registry::config_exist() {
        println!("No registry found. Please run:");
        println!("sudo nodemgr init");
        return ();
    }

    handler(matches)
}

fn handler(matches: ArgMatches) {
    let mut database = registry::Registry::load();

    match matches.subcommand() {
        Some(("add", matches)) => handle_add(matches, &mut database),
        Some(("export", matches)) => handle_export(matches, &mut database),
        Some(("list", _)) => handle_list(&mut database),
        Some(("remove", matches)) => handle_remove(matches, &mut database),
        Some(("start", _)) => handle_start(&mut database),
        Some(("stop", _)) => handle_stop(&mut database),
        Some(("init", _)) => {}
        Some(("import", _)) => {}
        _ => println!("Invalid subcommand"),
    };
}

fn handle_add(matches: &ArgMatches, database: &mut registry::Registry) {
    let name = matches.get_one::<String>("name");
    let path = matches.get_one::<String>("path");
    let chain = matches.get_one::<String>("chain");

    let new_node = node::Node::new(
        name.unwrap(),
        path.unwrap(),
        &node::Chain::from_str(chain.unwrap()).unwrap(),
    );
    database.add_node(&new_node);
}

fn handle_remove(matches: &ArgMatches, database: &mut registry::Registry) {
    let name = matches.get_one::<String>("name").unwrap();
    database.remove_node(name);
}

fn handle_init() {
    binary::download();
    registry::Registry::init();
}

fn handle_list(database: &registry::Registry) {
    database.list()
}

fn handle_export(matches: &ArgMatches, database: &registry::Registry) {
    let file = matches.get_one::<String>("file").unwrap();
    database.export(file)
}

fn handle_import(matches: &ArgMatches) {
    let file = matches.get_one::<String>("file").unwrap();
    let r = registry::Registry::from_toml(file);
    r.enable_all_service();
}

fn handle_start(database: &mut registry::Registry) {
    database.enable_all_service();
}

fn handle_stop(database: &mut registry::Registry) {
    database.disable_all_service();
}
