use crate::args::*;
use dirs::config_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::{exit, Stdio};

fn get_dir_contents(path: &str) -> Result<Vec<String>, std::io::Error> {
    let path = Path::new(path);
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut names = Vec::new();
            for dir in entries.flatten() {
                if dir.path().is_dir() {
                    names.push(dir.file_name().to_string_lossy().to_string());
                }
            }
            Ok(names)
        }
        Err(e) => Err(e),
    }
}

// Byte-indexes into `target`/`name` (`&target[..index]`) previously panicked on any
// non-ASCII character (not a char boundary) and on an empty `target` (`target.len() - 1`
// underflow). Indexing by `.chars()` instead keeps this Unicode-safe.
fn search(names_in: Vec<String>, target: &str) -> String {
    if target.is_empty() {
        return "".to_string();
    }
    let mut names = names_in;
    if names.iter().any(|n| n == target) {
        return target.to_string();
    }

    let target_char_count = target.chars().count();
    let mut found = false;
    let mut index = 1;
    while !found {
        let mut searchlist: Vec<String> = Vec::new();
        let searchstr: String = target.chars().take(index).collect();

        for name in &names {
            let name_prefix: String = name.chars().take(index).collect();
            if name.chars().count() > index && name_prefix == searchstr {
                searchlist.push(name.clone());
            }
        }

        if searchlist.is_empty() {
            break;
        }

        if index >= target_char_count {
            if searchlist.len() == 1 {
                found = true;
            } else {
                return "INSUFFICENT".to_string();
            }
        }

        names = searchlist;
        index += 1;
    }
    if found {
        names[0].clone()
    } else {
        "".to_string()
    }
}

pub fn handle_goto(projpath: String, goto: GotoProj) {
    if let Some(project) = goto.project {
        let path = format!("{}/{}", projpath, goto.proj_group);
        let names = match get_dir_contents(&path) {
            Ok(n) => n,
            Err(_) => {
                println!(
                    "Error: Proj group could not be found! : {}",
                    goto.proj_group
                );
                exit(1);
            }
        };
        let out = search(names, &project);
        if out.is_empty() {
            println!("Error: Proj group could not be found! : {project}");
            exit(1);
        } else if out == "INSUFFICENT" {
            println!("Error: More than 1 match exists! (add more letters)");
            exit(1);
        } else {
            println!("x cd \"{}/{}/{}\"", projpath, goto.proj_group, out);
        }
    } else {
        let names = match get_dir_contents(&projpath) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("proj-cmd: goto: {e}");
                exit(1);
            }
        };
        let out = search(names, &goto.proj_group);
        if out.is_empty() {
            println!(
                "Error: Proj group could not be found! : {}",
                goto.proj_group
            );
            exit(1);
        } else {
            println!("x cd \"{projpath}/{out}\"");
        }
    }
}

pub fn handle_list(projpath: String, list: ListProj) {
    let (chr, path) = match list.proj_group {
        None => ('󰾂', Path::new(&projpath).join("")),
        Some(proj_to_list) => ('󰆧', Path::new(&projpath).join(proj_to_list)),
    };

    // Read the directory once: the previous version computed `maxlen` via a
    // separate get_dir_contents(...).unwrap() call *before* this error-handled
    // read, so a missing proj group panicked instead of hitting the message below.
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("proj-cmd: list: Proj group or project does not exist");
            exit(1);
        }
    };

    let mut names = Vec::new();
    for dir in entries.flatten() {
        if dir.path().is_dir() {
            names.push(dir.file_name().to_string_lossy().to_string());
        }
    }

    let maxlen = names.iter().map(|n| n.len()).max().unwrap_or(0).max(10);

    let bot = format!("╰{}╯", "─".repeat(maxlen + 4));
    let top = format!("╭{}╮", "─".repeat(maxlen + 4));

    println!("{top}");
    for name in &names {
        println!("│ {chr} {name:<maxlen$} │");
    }
    println!("{bot}");
}

pub fn handle_make(projpath: String, make: CreateNewProjGroup) {
    let name_of_proj = make.proj_group_name;
    let path = Path::new(&projpath).join(&name_of_proj);
    match fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("proj-cmd: make: {e}");
            exit(1);
        }
    }

    println!("x cd {path:?}");
}

pub fn handle_create(projpath: String, make: CreateNewProject) {
    let proj_group_name = make.proj_group;
    let name_of_project = make.project_name;
    let path = Path::new(&projpath).join(&proj_group_name);
    if !path.exists() {
        eprintln!("proj-cmd: create: No such proj group `{proj_group_name}`");
        exit(1)
    }
    let project_path = path.join(&name_of_project);
    match fs::create_dir(&project_path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("proj_cmd: create: {e}");
            exit(1);
        }
    }
    let script_path = path.join("proj-init");
    if script_path.exists() {
        let _ = Command::new(script_path)
            .arg(&project_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    println!("x cd {project_path:?}");
}

pub fn handle_setup(setup: SetupProj) {
    let Some(path) = config_dir().map(|d| d.join("proj-cmd")) else {
        eprintln!("proj-cmd: setup: could not determine config directory");
        exit(1);
    };

    let Some(proj_home) = setup.proj_home_path else {
        let projrc_path = path.join("projrc");

        if projrc_path.exists() {
            match fs::read_to_string(&projrc_path) {
                Ok(proj_home) => println!("Current proj_home_path = {}", proj_home.trim()),
                Err(e) => {
                    eprintln!("proj-cmd: setup: {e}");
                    exit(1);
                }
            }
        } else {
            eprintln!("proj-cmd: setup: proj_home has not been configured yet");
            exit(1);
        }
        return;
    };
    let proj_home = Path::new(&proj_home);
    let _ = fs::create_dir(&path);
    let mut file = match File::create(path.join("projrc")) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("proj-cmd: setup: {e}");
            exit(1);
        }
    };
    match write!(file, "{}", proj_home.display()) {
        Ok(_) => {
            println!("set proj_home to `{proj_home:?}`");
        }
        Err(e) => {
            eprintln!("proj-cmd: setup: {e}");
            exit(1);
        }
    }
}

pub fn handle_init(init: Shell) {
    let cmd = init.cmd.as_deref().unwrap_or("proj");

    match init.shell.as_str() {
        "zsh" | "bash" => {
            println!(
                "{}",
                include_str!("../shell-scripts/proj.sh").replace("name", cmd)
            )
        }
        "fish" => {
            println!(
                "{}",
                include_str!("../shell-scripts/proj-fish.fish").replace("name", cmd)
            )
        }
        "nu" => {
            println!(
                "{}",
                include_str!("../shell-scripts/proj-nu.nu").replace("name", cmd)
            )
        }
        _ => {
            eprintln!("proj-cmd: init: Only zsh, bash, nu and fish are supported :( ");
            exit(1)
        }
    }
}

pub fn handle_zip(projpath: String, zip: ZipProj) {
    let zip_name = {
        if zip.project_name.is_none() {
            zip.proj_group.clone()
        } else {
            zip.project_name.clone().unwrap()
        }
    };

    let path_to_zip = format!(
        "{projpath}/{}/{}",
        zip.proj_group,
        zip.project_name.unwrap_or("".to_string())
    );

    if !Path::new(&path_to_zip).exists() {
        eprintln!("proj-cmd: zip: Proj group or project does not exist");
        exit(1);
    }

    println!("Zipping {path_to_zip} ...");
    let _ = Command::new("zip")
        .arg("-qr")
        .arg(zip_name)
        .arg(path_to_zip)
        .status();
    println!("Done :)");
}
