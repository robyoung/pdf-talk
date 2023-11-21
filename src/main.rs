use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use lopdf::xref::XrefType;
use pdf_talk::commands::{create_maxi::main as create_maxi, create_mini::main as create_mini};
use pdf_talk::config::{Config, Driver, FontType};

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    /// What format to use for the cross-reference table
    #[arg(short, long, value_enum, default_value = "table")]
    pub xref_type: XrefTypeWrapper,

    #[arg(short, long, value_enum, default_value = "type0")]
    pub font_type: FontType,

    #[arg(short, long, value_enum, default_value = "lopdf")]
    pub driver: Driver,

    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(short, long)]
    pub subset: bool,

    #[command(subcommand)]
    command: Command,
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Config {
        Config {
            xref_type: cli.xref_type.into(),
            font_type: cli.font_type,
            driver: cli.driver,
            output: cli.output,
            subset: cli.subset,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a minimal PDF document.
    CreateMini,

    /// Create a PDF document that showcases various features.
    CreateMaxi,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum XrefTypeWrapper {
    Stream,
    Table,
}

impl From<XrefTypeWrapper> for XrefType {
    fn from(wrapper: XrefTypeWrapper) -> Self {
        match wrapper {
            XrefTypeWrapper::Table => XrefType::CrossReferenceTable,
            XrefTypeWrapper::Stream => XrefType::CrossReferenceStream,
        }
    }
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::CreateMini => create_mini(cli.into()),
        Command::CreateMaxi => create_maxi(cli.into()),
    }
}
