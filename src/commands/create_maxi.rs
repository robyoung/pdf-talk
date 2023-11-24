use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, ObjectId, Stream,
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
        Pt(600.0).into(),
        Pt(800.0).into(),
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

    let kids = vec![
        create_page_one(&mut doc, pages_id, font_ref).into(),
        create_page_two(&mut doc, pages_id, font_ref).into(),
    ];
    let page_count = kids.len();

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => kids,
        "Count" => page_count as u32,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 600.into(), 600.into()],
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

fn create_page_one(
    doc: &mut Document,
    pages_id: ObjectId,
    font_ref: &dyn FontReference,
) -> ObjectId {
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F0".into(), 28.into()]),
            Operation::new("Td", vec![50.into(), 300.into()]),
            Operation::new("TL", vec![36.into()]),
            Operation::new("Tj", font_ref.render_text("This is a block of text that")),
            Operation::new("T*", vec![]),
            Operation::new("Tj", font_ref.render_text("should spread across the page.")),
            Operation::new("ET", vec![]),
        ],
    };
    let content_id = doc
        .add_object(Stream::new(dictionary! {}, content.encode().unwrap()).with_compression(false));
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    })
}

fn create_page_two(
    doc: &mut Document,
    pages_id: ObjectId,
    font_ref: &dyn FontReference,
) -> ObjectId {
    let circle_centre = (300.0, 300.0);
    let circle_radius = 100.0;
    let content = Content {
        operations: [
            vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F0".into(), 36.into()]),
                Operation::new("Td", vec![140.into(), 500.into()]),
                Operation::new("TL", vec![48.into()]),
                Operation::new("Tj", font_ref.render_text("Circle say YAY!")),
                Operation::new("ET", vec![]),
            ],
            make_circle_go_yay(circle_radius, circle_centre),
        ]
        .concat(),
    };
    let content_id = doc
        .add_object(Stream::new(dictionary! {}, content.encode().unwrap()).with_compression(false));
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    })
}

fn make_circle_go_yay(radius: f64, center: (f64, f64)) -> Vec<Operation> {
    let c = 4.0 / 3.0 * (f64::sqrt(2.0) - 1.0);
    let yay_offset = radius / 2.0;

    vec![
        Operation::new("q", vec![]),
        Operation::new("w", vec![3.into()]),
        Operation::new("RG", vec![0.into(), 1.into(), 0.into()]),
        Operation::new("m", vec![center.0.into(), (center.1 - radius).into()]),
        // Make the circle
        Operation::new(
            "c",
            vec![
                (center.0 + radius * c).into(),
                (center.1 - radius).into(),
                (center.0 + radius).into(),
                (center.1 - radius * c).into(),
                (center.0 + radius).into(),
                center.1.into(),
            ],
        ),
        Operation::new(
            "c",
            vec![
                (center.0 + radius).into(),
                (center.1 + radius * c).into(),
                (center.0 + radius * c).into(),
                (center.1 + radius).into(),
                center.0.into(),
                (center.1 + radius).into(),
            ],
        ),
        Operation::new(
            "c",
            vec![
                (center.0 - radius * c).into(),
                (center.1 + radius).into(),
                (center.0 - radius).into(),
                (center.1 + radius * c).into(),
                (center.0 - radius).into(),
                center.1.into(),
            ],
        ),
        Operation::new(
            "c",
            vec![
                (center.0 - radius).into(),
                (center.1 - radius * c).into(),
                (center.0 - radius * c).into(),
                (center.1 - radius).into(),
                center.0.into(),
                (center.1 - radius).into(),
            ],
        ),
        // Make the YAY
        Operation::new(
            "m",
            vec![(center.0 - radius - yay_offset).into(), center.1.into()],
        ),
        Operation::new(
            "l",
            vec![
                (center.0 - radius - yay_offset * 2.0).into(),
                (center.1 + radius).into(),
            ],
        ),
        Operation::new(
            "m",
            vec![(center.0 + radius + yay_offset).into(), center.1.into()],
        ),
        Operation::new(
            "l",
            vec![
                (center.0 + radius + yay_offset * 2.0).into(),
                (center.1 + radius).into(),
            ],
        ),
        Operation::new("S", vec![]),
        Operation::new("Q", vec![]),
    ]
}
