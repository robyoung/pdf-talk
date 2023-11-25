use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use lopdf::xref::XrefType;
use pdf_talk::commands::{create_maxi::main as create_maxi, create_mini::main as create_mini};
use pdf_talk::config::{CreateConfig, FontType};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Args, Debug)]
struct CreateArgs {
    /// What format to use for the cross-reference table.
    #[arg(short, long, value_enum, default_value = "table")]
    pub xref_type: XrefTypeWrapper,

    /// Which kind of font to use.
    #[arg(short, long, value_enum, default_value = "type0")]
    pub font_type: FontType,

    /// Disable stream compression entirely.
    #[arg(short = 'z', long)]
    pub no_compress: bool,

    /// Compress content streams, requires `compress` to take effect.
    #[arg(short, long)]
    pub compress_content: bool,

    /// Use the subsetted font.
    #[arg(short, long)]
    pub subset: bool,

    #[command(subcommand)]
    pub command: CreateCommand,
}

#[derive(Args, Debug)]
struct CreateOutput {
    /// Output file
    #[arg()]
    pub output: PathBuf,
}

impl From<CreateArgs> for CreateConfig {
    fn from(args: CreateArgs) -> CreateConfig {
        let output = match args.command {
            CreateCommand::Mini(output) | CreateCommand::Maxi(output) => output.output,
        };
        CreateConfig {
            xref_type: args.xref_type.into(),
            font_type: args.font_type,
            compress: !args.no_compress,
            compress_content: args.compress_content,
            subset: args.subset,
            output,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create various PDF documents
    Create(CreateArgs),
}

#[derive(Subcommand, Debug)]
enum CreateCommand {
    /// Create a minimal PDF document.
    Mini(CreateOutput),

    /// Create a PDF document that showcases various features.
    Maxi(CreateOutput),
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum XrefTypeWrapper {
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
        Command::Create(create_args) => match create_args.command {
            CreateCommand::Mini(_) => create_mini(create_args.into()),
            CreateCommand::Maxi(_) => create_maxi(create_args.into()),
        },
    }
}
