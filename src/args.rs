use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ProjArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// cd into a proj group or into an individual project in a group
    Goto(GotoProj),

    /// Make a new proj group
    Make(CreateNewProjGroup),

    /// Create a new project in specified proj group
    Create(CreateNewProject),

    /// List all projects in specified proj group
    List(ListProj),

    /// Setup the proj command
    Setup(SetupProj),

    /// Add this to the end of your shell's `rc`
    Init(Shell),
}

#[derive(Debug, Args)]
pub struct GotoProj {
    /// Name of proj group
    pub proj_group: String,

    /// Name of project within proj group <optional>
    pub project: Option<String>,
}

#[derive(Debug, Args)]
pub struct CreateNewProjGroup {
    /// Name of proj group to create
    pub proj_group_name: String,
}

#[derive(Debug, Args)]
pub struct ListProj {
    /// Proj group to list contents of
    pub proj_group: String,
}

#[derive(Debug, Args)]
pub struct CreateNewProject {
    /// proj group to create project in
    pub proj_group: String,

    /// Name of project to be created
    pub project_name: String,
}

#[derive(Debug, Args)]
pub struct SetupProj {
    /// Set proj home path (without this option proj will get the path instead)
    pub proj_home_path: Option<String>,
}

#[derive(Debug, Args)]
pub struct Shell {
    /// Shell name (`zsh`, `bash`, `fish`)
    pub shell: String,

    /// Name of command to use (Defaults to `proj`)
    pub cmd: Option<String>,
}
