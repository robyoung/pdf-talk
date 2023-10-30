use std::io;

use clap::{Parser, Subcommand};
use lopdf::{
    content::{Content, Operation},
    dictionary,
    xref::XrefType,
    Document, Object, Stream,
};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a minimal PDF document.
    CreateMin,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::CreateMin => create_min(),
    }
}

fn create_min() {
    let mut doc = Document::with_version("1.7");

    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Ariel",
    });

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary!{
            "F1" => font_id,
        },
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 36.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("TL", vec![48.into()]),
            Operation::new(
                "Tj",
                vec![Object::string_literal("This is a block of text that")],
            ),
            Operation::new("T*", vec![]),
            Operation::new(
                "Tj",
                vec![Object::string_literal("should spread across the page.")],
            ),
            Operation::new("ER", vec![]),
        ],
    };
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.reference_table.cross_reference_type = XrefType::CrossReferenceStream;
    let mut stdout = io::stdout();
    doc.save_to(&mut stdout).expect("Failed to write PDF");
}
