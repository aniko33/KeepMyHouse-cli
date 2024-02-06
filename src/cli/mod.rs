use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Actions {
    /// Create new database
    Init(Init),
    /// Open a database
    Open(Opendb),
    /// List of elements
    List(ListCmd),
    /// Export db
    Export(Export),
}

#[derive(Args)]
pub struct Init {
    pub filename: String,
}

#[derive(Args)]
pub struct Opendb {
    pub filename: String,
    #[arg(short)]
    pub encryption: String,
    #[arg(long)]
    pub file: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct ListCmd {
    #[arg(short)]
    pub encryption: bool,
    #[arg(short)]
    pub formatexport: bool,
}

#[derive(Args)]
pub struct Export {
    pub input: String,
    pub output: String,
    #[arg(short, long)]
    pub format: String,
    #[arg(short)]
    pub encryption: String,
    #[arg(short, long)]
    pub keyfile: bool,
}
