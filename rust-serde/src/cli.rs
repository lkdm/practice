use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};

pub struct Cli(Opt);

impl Cli {
    pub fn new() -> Self {
        let args = Opt::parse();
        Self(args)
    }
    pub fn args(&self) -> &Opt {
        &self.0
    }

    /// Generate Zsh completion script
    pub fn generate_completions(&self) {
        if let Some(generator) = self.0.generator {
            let &mut cmd = self.0.command();
            generate(
                generator,
                cmd,
                cmd.get_name().to_string(),
                &mut std::io::stdout(),
            )
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "mtutil", version, about, long_about = None)]
pub struct Opt {
    /// Action being taken
    #[command(subcommand)]
    command: Commands,

    /// Will output debug logs when enabled
    #[arg(short, long)]
    debug: bool,

    /// Used to generate auto completions for shells
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
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
