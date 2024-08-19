use crate::args::*;
use dirs::config_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub fn handle_goto(projpath: String, goto: GotoProj) {
    let path = format!(
        "{}/{}/{}",
        projpath,
        goto.proj_group,
        goto.project.unwrap_or("".to_string())
    );
    if Path::new(&path).exists() {
        println!("x cd {:?}", path);
    } else {
        println!("Error: Proj group or project does not exist");
    }
}

pub fn handle_list(projpath: String, list: ListProj) {
    let path;
    let proj_to_list = list.proj_group.clone();
    let chr;
    if proj_to_list.is_none() {
        chr = '󰾂';
        path = Path::new(&projpath).join("");
    } else {
        chr = '󰆧';
        path = Path::new(&projpath).join(&proj_to_list.unwrap());
    }

    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut maxlen = 10;
            let mut names = Vec::new();
            for entry in entries {
                if let Ok(dir) = entry {
                    if dir.path().is_dir() {
                        let name_to_print = dir.file_name();
                        if name_to_print.len() > maxlen {
                            maxlen = name_to_print.len();
                        }
                        names.push(name_to_print);
                    }
                }
            }

            let bot = format!("╰{}╯", "─".repeat(maxlen + 4));
            let top = format!("╭{}╮", "─".repeat(maxlen + 4));

            println!("{}", top);
            for name in names {
                println!("│ {chr} {:<maxlen$} │", name.to_str().unwrap());
            }
            println!("{}", bot);
        }
        Err(e) => {
            println!(
                "Error: No such proj group `{}`\n{e}",
                list.proj_group.unwrap_or("Huh?".to_string())
            );
            exit(1)
        }
    }
}

pub fn handle_make(projpath: String, make: CreateNewProjGroup) {
    let name_of_proj = make.proj_group_name;
    let path = Path::new(&projpath).join(&name_of_proj);
    match fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {e}");
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
        println!("Error: No such proj group {proj_group_name} ");
        exit(1)
    }
    let path = path.join(&name_of_project);
    match fs::create_dir(&path) {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }

    println!("x cd {path:?}");
}

pub fn handle_setup(setup: SetupProj) {
    let path = config_dir().unwrap().join("proj-cmd");

    if setup.proj_home_path.is_none() {
        let projrc_path = path.join("projrc");

        if projrc_path.exists() {
            let proj_home = fs::read_to_string(projrc_path).unwrap();
            let proj_home = proj_home.trim();
            println!("Current proj_home_path = {proj_home}");
        } else {
            println!("proj_home has not been configured yet");
        }
    } else {
        let proj_home = setup.proj_home_path.unwrap();
        let proj_home = Path::new(&proj_home);
        let _ = fs::create_dir(&path);
        let mut file = File::create(path.join("projrc")).unwrap();
        match write!(file, "{}", proj_home.display()) {
            Ok(_) => {
                print!("set proj_home to {proj_home:?}");
            }
            Err(e) => {
                println!("Error: {e}");
                exit(1);
            }
        }
    }
}

pub fn handle_init(init: Shell) {
    let cmd = init.cmd.unwrap_or(String::from("proj"));

    match init.shell.as_str() {
        "zsh" | "bash" => {
            println!(
                "
{cmd}() {{
  returned=$(proj-cmd $@)
  if [[ $returned == x\\ * ]]; then
    eval ${{returned:2}}
  else
    echo $returned
  fi
}}
"
            )
        }
        "fish" => {
            println!(
                "
function {cmd}
    set returned (proj-cmd $argv)
    if string match -qr '^x\\ ' $returned
        eval (string sub -s 3 $returned)
    else
        echo $returned
    end
end
"
            )
        }
        "nu" => {
            println!(
                "
def --env {cmd} [ ...args ] {{
    let returned = (proj-cmd ...$args)

    if ($returned | str starts-with \"x \") {{
        cd $returned  
    }} else {{
        echo $returned
    }}
}}
"
            )
        }
        _ => {
            println!("Error: Only zsh, bash, nu and fish are supported currently :( ");
            exit(1)
        }
    }
}
