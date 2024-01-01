use lopdf::{dictionary, Document, Object};

use crate::{config::CreateConfig, document::DocumentAdditions};

pub fn main(config: CreateConfig) {
    let mut doc = Document::with_version("2.0");

    let pages_id = doc.new_object_id();
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![],
        "Count" => 0,
        "MediaBox" => vec![0.into(), 0.into(), 960.into(), 540.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    doc.add_catalog(pages_id);
    config.apply_and_save(&mut doc);

    println!("create deck");
}
