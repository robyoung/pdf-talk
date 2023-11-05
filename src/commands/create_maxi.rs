use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use std::fs::File;
use std::io::{self, Read};

use crate::{config::Config, fonts};

pub fn main(config: Config) {
    let mut doc = Document::with_version("1.7");

    let pages_id = doc.new_object_id();

    let font_path = "documents/Fira-Code-med-complete-mono.ttf";
    // let font_path = "documents/Fira-subset.ttf";
    let mut font_file = File::open(font_path).expect("could not open file");
    let mut font_data = vec![];
    font_file
        .read_to_end(&mut font_data)
        .expect("failed to read font file");
    let font_id = fonts::true_type(&font_data).add_to_doc(&mut doc);

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary!{
            "F1" => font_id,
        },

    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 36.into()]),
            Operation::new("Td", vec![50.into(), 600.into()]),
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
    doc.reference_table.cross_reference_type = config.xref_type;
    let mut stdout = io::stdout();
    doc.save_to(&mut stdout).expect("Failed to write PDF");
}
