use lopdf::{dictionary, Document, Object, Stream};

use crate::{
    config::CreateConfig,
    document::{text, text_item as t, DocumentAdditions},
    fonts::{self, FontReference},
};

pub fn main(config: CreateConfig) {
    let mut doc = Document::with_version("1.7");

    let font_data = std::fs::read("assets/Georgia.ttf").expect("could not read font file");
    let font_ref = fonts::true_type(&font_data).add_to_doc(&mut doc);

    let pages_id = doc.new_object_id();

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_ref.object_id(),
        }
    });
    let content = text()
        .font("F1", 36)
        .move_to(100, 200)
        .leading(48)
        .word_spacing(-8f32)
        .colour((0.106, 0.259, 0.471))
        .text(vec![
            t("W"),
            t(-500),
            t("h"),
            t(-50),
            t("at even i"),
            t(200),
            t("s"),
            t(" a P"),
            t(-100),
            t("D"),
            t(-200),
            t("F"),
            t(-100),
            t("?"),
        ])
        .build_content();
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
        "MediaBox" => vec![0.into(), 0.into(), 960.into(), 540.into()],
        // "MediaBox" => vec![0.into(), 0.into(), 960.into(), 540.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    doc.add_catalog(pages_id);
    config.apply_and_save(&mut doc);

    println!("create deck");
}
