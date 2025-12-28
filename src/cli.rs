use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nfdiff", version, about = "Netfilter snapshot & diff tool")]
pub struct Cli {
    #[command(Subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Snap {
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}
