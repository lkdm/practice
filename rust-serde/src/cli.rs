use clap::{Command, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell, generate};

/// Generate Zsh completion script
pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    )
}

#[derive(Parser, Debug)]
#[command(name = "mtutil", version, about, long_about = None)]
pub struct Opt {
    /// Action being taken
    #[command(subcommand)]
    pub command: Commands,

    /// Will output debug logs when enabled
    #[arg(short, long)]
    pub debug: bool,

    /// Used to generate auto completions for shells
    #[arg(long = "generate", value_enum)]
    pub generator: Option<Shell>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Runs a set of services
    Run {
        #[arg(value_enum)]
        args: RunServiceOpts,
    },
    /// Installs necessary libraries
    Install {
        #[arg(value_enum)]
        args: InstallOpts,
    },
    /// Compiles various projects
    Compile {
        #[arg(value_enum)]
        args: CompileOpts,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum RunServiceOpts {
    /// Runs everything
    Full,
    /// Starts the following services: SASS watch, Flask, Server TS, IRIS, and frontend/backend TS Watch
    Services,
    /// Runs Flask
    Flask,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum InstallOpts {
    /// Installs all libs via npm, Gulp, pip, and go get
    All,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CompileOpts {
    /// Compiles all TS, Go, and Gulp projects
    All,
    /// Compiles Gulp projects
    Gulp,
    /// Compiles backend Typescript projects
    TsBackend,
    /// Compiles frontend Typescript projects
    TsFrontend,
    /// Recompiles the dbclone Go project
    DbClone,
    /// Recompiles IRIS
    Iris,
}
