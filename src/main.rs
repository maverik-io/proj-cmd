use clap::Parser;
use std::fs;
mod args;
mod handlers;

use args::{Action, ProjArgs};
use handlers::*;

fn main() {
    let Some(config_dir) = dirs::config_dir() else {
        eprintln!("proj-cmd: could not determine config directory");
        std::process::exit(1);
    };
    let config_path = config_dir.join("proj-cmd/projrc");

    if let Ok(projpath) = fs::read_to_string(&config_path) {
        let args = ProjArgs::parse();

        match args.action {
            Action::Goto(goto) => handle_goto(projpath, goto),
            Action::List(list) => handle_list(projpath, list),
            Action::Make(make) => handle_make(projpath, make),
            Action::Create(create) => handle_create(projpath, create),
            Action::Setup(setup) => handle_setup(setup),
            Action::Init(init) => handle_init(init),
            Action::Zip(zip) => handle_zip(projpath, zip),
        }
    } else {
        let Some(home_path) = dirs::home_dir() else {
            eprintln!("proj-cmd: could not determine home directory");
            std::process::exit(1);
        };
        let home_path = home_path.to_string_lossy().to_string();

        println!("Cannot find config file. Creating...");
        if let Err(e) = fs::create_dir_all(config_dir.join("proj-cmd")) {
            eprintln!("proj-cmd: failed to create config directory: {e}");
            std::process::exit(1);
        }
        if let Err(e) = fs::write(&config_path, &home_path) {
            eprintln!("proj-cmd: failed to write config file: {e}");
            std::process::exit(1);
        }
        println!("Project root set to home dir, Use proj setup <path> to update ");
    }
}
