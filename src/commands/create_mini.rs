use crate::{
    config::CreateConfig,
    document::DocumentAdditions,
    fonts::{self, FontReference},
};
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};

pub fn main(config: CreateConfig) {
    let mut doc = generate_document();
    config.apply_and_save(&mut doc);
}

pub(crate) fn generate_document() -> Document {
    let mut doc = Document::with_version("1.7");

    let pages_id = doc.new_object_id();

    let font_ref = fonts::type1("Helvetica").add_to_doc(&mut doc);

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary!{
            "F2" => font_ref.object_id(),
        },
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F2".into(), 36.into()]),
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

    doc.add_catalog(pages_id);

    doc
}
