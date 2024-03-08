use lopdf::{
    content::{Content, Operation},
    dictionary, xobject, Document, Object, ObjectId, Stream,
};

use crate::{
    config::{CreateConfig, FontFile, FontType},
    document::DocumentAdditions,
    fonts::{self, FontReference},
};

static FIRA_CODE: FontFile = FontFile {
    full: "assets/FiraCodeNerdFontMono-Medium.ttf",
    subset: "assets/FiraCodeNerdFontMono-Medium.subset.ttf",
};
// static ROBOTO_FONT_PATH: &str = "assets/RobotoMedium.ttf";

pub fn main(config: CreateConfig) {
    let mut doc = generate_document(&config);
    config.apply_and_save(&mut doc);
}

pub(crate) fn generate_document(config: &CreateConfig) -> Document {
    let mut doc = Document::with_version("1.3");

    let font_data = std::fs::read(config.font_path(&FIRA_CODE)).expect("could not read font file");

    if let FontType::Type0 = config.font_type {
        let font_ref = fonts::type0(&font_data).add_to_doc(&mut doc);
        generate_pages(config, doc, &font_ref)
    } else {
        let font_ref = fonts::true_type(&font_data).add_to_doc(&mut doc);
        generate_pages(config, doc, &font_ref)
    }
}

fn generate_pages(
    config: &CreateConfig,
    mut doc: Document,
    font_ref: &dyn FontReference,
) -> Document {
    let pages_id = doc.new_object_id();
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary!{
            "F3" => font_ref.object_id(),
        },

    });

    let kids = vec![
        create_page_one(&mut doc, &config, pages_id, font_ref).into(),
        create_page_two(&mut doc, &config, pages_id, font_ref).into(),
        create_page_three(&mut doc, &config, pages_id, font_ref).into(),
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
    doc.add_catalog(pages_id);
    doc
}

fn create_page_one(
    doc: &mut Document,
    config: &CreateConfig,
    pages_id: ObjectId,
    font_ref: &dyn FontReference,
) -> ObjectId {
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F3".into(), 28.into()]),
            Operation::new("Td", vec![50.into(), 300.into()]),
            Operation::new("TL", vec![36.into()]),
            Operation::new("Tj", font_ref.render_text("This is a block of text that")),
            Operation::new("T*", vec![]),
            Operation::new("Tj", font_ref.render_text("should spread across the page.")),
            Operation::new("ET", vec![]),
        ],
    };
    let content_id = doc.add_object(
        Stream::new(dictionary! {}, content.encode().unwrap())
            .with_compression(config.compress_content),
    );
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    })
}

fn create_page_two(
    doc: &mut Document,
    config: &CreateConfig,
    pages_id: ObjectId,
    font_ref: &dyn FontReference,
) -> ObjectId {
    let circle_centre = (300.0, 300.0);
    let circle_radius = 100.0;
    let content = Content {
        operations: [
            vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F3".into(), 36.into()]),
                Operation::new("Td", vec![140.into(), 500.into()]),
                Operation::new("TL", vec![48.into()]),
                Operation::new("Tj", font_ref.render_text("Circle say YAY!")),
                Operation::new("ET", vec![]),
            ],
            make_circle_go_yay(circle_radius, circle_centre),
        ]
        .concat(),
    };
    let content_id = doc.add_object(
        Stream::new(dictionary! {}, content.encode().unwrap())
            .with_compression(config.compress_content),
    );
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
        Operation::new("w", vec![5.into()]),
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

fn create_page_three(
    doc: &mut Document,
    config: &CreateConfig,
    pages_id: ObjectId,
    font_ref: &dyn FontReference,
) -> ObjectId {
    let image_stream = xobject::image("assets/horsey.jpg").expect("could not read image file");
    let image_id = doc.add_object(image_stream);
    let _m = 400;
    let content_id = doc.add_object(
        Stream::new(
            dictionary! {},
            Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F3".into(), 28.into()]),
                    Operation::new("Td", vec![50.into(), 550.into()]),
                    Operation::new("TL", vec![36.into()]),
                    Operation::new("Tj", font_ref.render_text("Horsey say NEIGH!")),
                    Operation::new("ET", vec![]),
                    Operation::new("q", vec![]),
                    // Transformation matrix:
                    // See 8.3.3 Common Transformations in the PDF spec
                    // The argument is an array [a b c d e f] where
                    // a and d are the horizontal and vertical scaling factors
                    // b and c are the horizontal and vertical skew factors
                    // e and f are the horizontal and vertical translation values in the units of the coordinate system
                    // rotation is calculated by [rc, rs, -rs, rc, 0, 0] where rc = cos(angle) and rs = sin(angle)
                    Operation::new(
                        "cm",
                        vec![
                            10.into(),     // a scale by factor of 10 in x domain
                            (300).into(),  // b skew by factor of 300 in x domain
                            (-300).into(), // c skew by factor of -300 in y domain
                            10.into(),     // d scale by factor of 10 in y domain
                            400.into(),    // e move 400 units to the right
                            50.into(),     // f move 50 units down
                        ],
                    ),
                    Operation::new("Do", vec!["Im4".into()]),
                    Operation::new("Q", vec![]),
                ],
            }
            .encode()
            .unwrap(),
        )
        .with_compression(config.compress_content),
    );
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "Resources" => dictionary!{
            "XObject" => dictionary!{
                "Im4" => image_id,
            },
        },
    })
}
