use lopdf::{dictionary, Document, Object};

use crate::{
    config::CreateConfig,
    document::{content, DocumentAdditions},
    fonts::{FontMap, FontType0Builder},
};

pub fn main(config: CreateConfig) {
    let mut doc = Document::with_version("1.7");

    let font_path = "assets/Georgia.ttf";

    let font_ref = FontType0Builder::from_file(font_path)
        .expect("could not read font file")
        .add_to_doc(&mut doc);

    let pages_id = doc.new_object_id();

    let font_map = FontMap::with_one("F1", font_ref);

    let resources_id = doc.add_object(dictionary! {
        "Font" => font_map.as_dictionary()
    });

    let content_id = content(font_map)
        .font("F1", 36)
        .move_to(100, 200)
        .leading(48)
        .colour((0.106, 0.259, 0.471))
        .text("What even is a PDF?")
        .add_to_doc(&mut doc);

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
        "MediaBox" => vec![0.into(), 0.into(), 960.into(), 540.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    doc.add_catalog(pages_id);
    config.apply_and_save(&mut doc);

    println!("create deck");
}
