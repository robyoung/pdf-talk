use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use printpdf::{Color, PdfDocument, Pt, Rgb};
use std::{fs::File, io::BufWriter};

use crate::{
    config::{Config, Driver, FontFile, FontType},
    fonts::{self, FontReference},
};

static FIRA_CODE: FontFile = FontFile {
    full: "assets/FiraCodeNerdFontMono-Medium.ttf",
    subset: "assets/FiraCodeNerdFontMono-Medium.subset.ttf",
};
// static ROBOTO_FONT_PATH: &str = "assets/RobotoMedium.ttf";

pub fn main(config: Config) {
    match config.driver {
        Driver::Lopdf => create_with_lopdf(config),
        Driver::Printpdf => create_with_printpdf(config),
    }
}

fn create_with_printpdf(config: Config) {
    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Pt(595.0).into(),
        Pt(842.0).into(),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font_file = File::open(config.font_path(&FIRA_CODE)).expect("could not open font file");
    let font = doc
        .add_external_font(font_file)
        .expect("could not add font to document");

    let black = Rgb::new(0.0, 0.0, 0.0, None);
    current_layer.set_fill_color(Color::Rgb(black.clone()));
    current_layer.set_outline_color(Color::Rgb(black));

    current_layer.begin_text_section();
    current_layer.set_font(&font, 36.0);
    current_layer.set_text_cursor(Pt(100.0).into(), Pt(600.0).into());
    current_layer.set_line_height(48.0);
    current_layer.write_text("This is a block of text that", &font);
    current_layer.add_line_break();
    current_layer.write_text("should spread across the page.", &font);
    current_layer.end_text_section();

    let file = File::create(config.output).expect("Failed to open file");
    doc.save(&mut BufWriter::new(file))
        .expect("could not write PDF");
}

fn create_with_lopdf(config: Config) {
    let mut doc = Document::with_version("1.3");

    let font_data = std::fs::read(config.font_path(&FIRA_CODE)).expect("could not read font file");

    if let FontType::Type0 = config.font_type {
        let font_ref = fonts::type0(&font_data).add_to_doc(&mut doc);
        create_page_with_lopdf(config, doc, &font_ref)
    } else {
        let font_ref = fonts::true_type(&font_data).add_to_doc(&mut doc);
        create_page_with_lopdf(config, doc, &font_ref)
    }
}

fn create_page_with_lopdf(config: Config, mut doc: Document, font_ref: &dyn FontReference) {
    let pages_id = doc.new_object_id();
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary!{
            "F0" => font_ref.object_id(),
        },

    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F0".into(), 36.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("TL", vec![48.into()]),
            Operation::new("Tj", font_ref.render_text("This is a block of text that")),
            Operation::new("T*", vec![]),
            Operation::new("Tj", font_ref.render_text("should spread across the page.")),
            Operation::new("ET", vec![]),
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
    let mut file = BufWriter::new(File::create(config.output).expect("Failed to open file"));
    doc.save_to(&mut file).expect("Failed to write PDF");
}
