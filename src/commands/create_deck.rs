use lopdf::{dictionary, xobject, Document, Object};

use crate::{
    config::CreateConfig,
    document::{ContentBuilder, DocumentAdditions, IdMap},
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
    let image_stream = xobject::image("assets/tnt-logo.png").expect("could not read tnt logo");
    let image_id = doc.add_object(image_stream);

    let image_map = IdMap::with_one("Im1", image_id);

    let resources_id = doc.add_object(dictionary! {
        "Font" => font_map.as_dictionary(),
        "XObject" => image_map.as_dictionary(),
    });

    let content_id = ContentBuilder::default()
        .with_font_map(font_map)
        .with_xobject_map(image_map)
        .begin_text()
        .font("F1", 36)
        .move_to(100, 200)
        .leading(48)
        .colour((0.106, 0.259, 0.471))
        .text("What even is a PDF?")
        .end_text()
        .save_graphics_state()
        .cm_position(100, 100)
        .cm_scale(118, 17)
        .add_xobject("Im1")
        .restore_graphics_state()
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
