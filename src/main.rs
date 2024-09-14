use clap::Parser;
use dirs::config_dir;
use std::fs;
mod args;
mod handlers;

use args::{Action, ProjArgs};
use handlers::*;

fn main() {
    let projpath = fs::read_to_string(config_dir().unwrap().join("proj-cmd/projrc"))
        .expect("unable to open config file at ~/.config/proj/projpath")
        .trim()
        .to_string();

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
}
