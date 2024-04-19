use lopdf::{content::Content, dictionary, xobject, Document, Object, ObjectId};

use crate::{
    commands::{create_maxi, create_mini},
    config::CreateConfig,
    document::{Colour, ContentBuilder, DocumentAdditions, Resources},
    fonts::{self, FontType0Builder},
};

const DARK_BLUE: Colour = (0.106, 0.259, 0.471);
const LIGHT_BLUE: Colour = (0., 0.624, 0.855);
const BRIGHT_RED: Colour = (0.97, 0., 0.1);
const BLACK: Colour = (0., 0., 0.);
const GREY: Colour = (0.5, 0.5, 0.5);

const MAGENTA: Colour = (0.9, 0.2, 0.6);
const MUSTARD: Colour = (0.8, 0.8, 0.5);
const PALE_BLUE: Colour = (0.6, 0.6, 0.9);
const PALE_GREEN: Colour = (0.6, 0.9, 0.6);
const PALE_RED: Colour = (0.9, 0.6, 0.6);

fn lighter(colour: Colour, factor: f32) -> Colour {
    (
        colour.0 + (1. - colour.0) * factor,
        colour.1 + (1. - colour.1) * factor,
        colour.2 + (1. - colour.2) * factor,
    )
}

const PDF_LOGO: &str = r#"q
435.02 0 m
f
0.984314 0.203922 0.286275 rg 
455.188 296.719 m
447.98 283.867 423.066 279.832 411.641 278.012 c
402.633 276.582 393.434 276.152 384.324 276.16 c
377.172 276.113 370.121 276.465 363.164 276.863 c
360.598 277.035 358.059 277.254 355.523 277.477 c
352.922 274.785 350.41 272.004 347.977 269.16 c
332.5 250.84 320.004 230.066 309.824 208.477 c
312.527 198.047 314.691 187.16 315.996 175.852 c
318.379 155.219 319.199 131.703 311.469 112.031 c
308.797 105.238 301.68 96.973 293.469 101.09 c
284.027 105.824 281.371 119.234 280.602 128.742 c
279.98 136.426 280.414 144.129 281.703 151.688 c
283.02 159.293 285.133 166.516 287.438 173.73 c
289.586 180.344 291.957 186.906 294.539 193.406 c
292.898 198.523 291.16 203.555 289.332 208.453 c
285.074 219.621 280.469 230.23 276.039 240.434 c
273.707 245.496 271.418 250.449 269.184 255.293 c
262.105 270.836 254.426 286.098 245.801 300.848 c
225.691 307.941 207.641 316.168 192.652 325.832 c
184.613 331.027 177.508 336.668 171.594 342.855 c
166.012 348.699 160.336 356.281 159.836 364.672 c
159.559 369.41 161.434 374.008 165.316 376.816 c
170.652 380.805 177.727 380.539 183.918 379.242 c
204.203 374.988 219.777 357.551 233.043 342.855 c
242.18 332.73 252.578 319.879 263.457 304.332 c
263.48 304.297 263.504 304.262 263.531 304.227 c
282.188 298.441 302.496 293.559 324.047 289.969 c
333.891 288.336 344 287 354.309 286.062 c
361.555 292.852 369.387 299.035 377.934 304.199 c
384.59 308.293 391.668 311.75 399.078 314.328 c
406.57 316.777 414.105 318.773 421.891 320.039 c
425.82 320.602 429.84 320.855 433.945 320.703 c
443.109 320.355 456.266 316.84 457.129 305.723 c
457.395 302.316 456.66 299.332 455.188 296.719 c
h
235.781 317.121 m
231.488 323.77 227.352 329.75 223.449 335.012 c
213.898 348.012 202.988 363.449 187.191 369.219 c
184.191 370.316 180.242 371.449 176.078 371.203 c
172.367 370.984 168.711 369.348 168.879 365.137 c
168.961 362.934 170.043 360.121 171.695 357.359 c
173.508 354.328 175.75 351.551 178.176 348.996 c
183.379 343.531 189.957 338.23 197.523 333.324 c
209.133 325.797 222.988 319.02 238.434 312.969 c
237.547 314.375 236.66 315.773 235.781 317.121 c
h
289.652 150.371 m
288.461 143.418 288.281 136.367 289.016 129.586 c
289.379 126.195 290.07 122.887 291.07 119.785 c
291.914 117.156 293.746 110.73 296.66 109.906 c
301.469 108.543 302.949 118.867 303.492 121.789 c
306.594 138.406 303.859 156.883 300.141 173.184 c
299.551 175.773 298.891 178.324 298.219 180.867 c
297.066 177.699 295.969 174.52 294.957 171.324 c
292.777 164.348 290.789 157.293 289.652 150.371 c
h
322.668 281.539 m 
304.602 284.469 287.414 288.305 271.258 292.836 c
273.203 292.289 282.094 275.441 284.074 271.945 c
293.418 255.492 301.062 238.219 306.551 220.102 c
316.25 239.281 328.02 257.629 342.477 273.957 c
343.809 275.441 345.16 276.906 346.539 278.352 c
338.438 279.227 330.465 280.289 322.668 281.539 c
h
444.859 304.68 m 
444.199 308.254 436.57 310.293 433.008 310.855 c
422.488 312.512 411.359 311.188 401.289 307.805 c
394.383 305.484 387.719 302.328 381.387 298.527 c
375.094 294.727 369.207 290.254 363.703 285.32 c
370.488 284.914 377.359 284.648 384.262 284.777 c
391.164 284.848 398.121 285.195 404.992 286.094 c
417.875 287.527 432.312 291.953 442.457 300.312 c
444.453 301.961 445.09 303.418 444.859 304.68 c
h
435.02 99.988 m 
f
Q
"#;

pub fn main(config: CreateConfig) {
    let mut doc = Document::with_version("1.7");

    let pages_id = doc.new_object_id();

    // TODO @robyoung add resources and lopdf::Document to a wrapper type
    let mut resources = Resources::default();
    setup_fonts(&mut doc, &mut resources);
    setup_images(&mut doc, &mut resources);

    let resources_id = resources.add_to_doc(&mut doc);
    let mut page_ids = vec![
        title::page(&mut doc, &resources, pages_id),
        what::page(&mut doc, &resources, pages_id),
        history::page(&mut doc, &resources, pages_id),
        three_documents::page(&mut doc, &resources, pages_id),
        tools::page(&mut doc, &resources, pages_id),
    ];
    page_ids.append(&mut file_structure::pages(&mut doc, &resources, pages_id));
    page_ids.append(&mut doc_structure::pages(&mut doc, &resources, pages_id));

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.iter().map(|&p| p.into()).collect::<Vec<Object>>(),
        "Count" => (page_ids.len() as u32),
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 960.into(), 540.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    doc.add_catalog(pages_id);
    config.apply_and_save(&mut doc);

    println!("create deck");
}

fn setup_fonts(doc: &mut Document, resources: &mut Resources) {
    let font_ref = FontType0Builder::from_file("assets/Georgia.ttf")
        .expect("could not read font file")
        .add_to_doc(doc);
    resources.set_font("F1", font_ref);

    // manually import font from mini
    let font_ref = fonts::type1("Helvetica").add_to_doc(doc);
    resources.set_font("F2", font_ref);

    // manually import font from maxi
    let font_ref = FontType0Builder::from_file("assets/FiraCodeNerdFontMono-Medium.ttf")
        .expect("could not read font file")
        .add_to_doc(doc);
    resources.set_font("F3", font_ref);
}

fn setup_images(doc: &mut Document, resources: &mut Resources) {
    let image_stream =
        xobject::image("assets/web-small.jpg").expect("could not read web screenshot");
    let image_id = doc.add_object(image_stream);
    resources.set_xobject("Im1", image_id);

    let image_stream = xobject::image("assets/tnt-logo.png").expect("could not read tnt logo");
    let image_id = doc.add_object(image_stream);
    resources.set_xobject("Im3", image_id);

    // manually import image from maxi page 3
    let image_stream = xobject::image("assets/horsey.jpg").expect("could not read image file");
    let image_id = doc.add_object(image_stream);
    resources.set_xobject("Im4", image_id);
}

mod title {
    //! Page 1 of the deck
    use super::*;

    pub fn page(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let content_builder = ContentBuilder::new(resources)
            // write text
            .begin_text()
            .font("F1", 38)
            .text_position(530, 350)
            .colour(DARK_BLUE) // dark blue
            .text("What even is a PDF?")
            .end_text()
            .begin_text()
            .font("F1", 17)
            .text_position(770, 100)
            .colour(LIGHT_BLUE) // pale blue
            .text("January 2024")
            .end_text();

        let content_builder = add_tnt_logo(content_builder);
        let content_builder = add_pdf_logo(content_builder);

        content_builder.add_to_doc_with_page(doc, pages_id)
    }

    fn add_tnt_logo(b: ContentBuilder) -> ContentBuilder {
        let x = 737;
        let y = 457;
        let line_height = 28;
        let line_offset = 28;
        let line_width = 1.8;
        b
            // place image
            .save_graphics_state()
            .cm_position(x, y)
            .cm_scale(118f32 * 1.64, 17f32 * 1.64)
            .add_xobject("Im3")
            .restore_graphics_state()
            // place white line
            .save_graphics_state()
            .scolour((1., 1., 1.))
            .line_width(line_width)
            .begin_path(x + line_offset, y)
            .append_straight_line(x + line_offset, y + line_height)
            .stroke_path()
            .restore_graphics_state()
            // place blue line
            .thin_blue_line((500, 80), (880, 80))
        // draw ruler
        // .save_graphics_state()
        // .scolour((0., 0., 0.))
        // .line_width(0.5)
        // .begin_path(880, 0)
        // .append_straight_line(880, 600)
        // .stroke_path()
        // .restore_graphics_state()
    }

    fn add_pdf_logo(b: ContentBuilder) -> ContentBuilder {
        let content = Content::decode(PDF_LOGO.as_bytes()).expect("unable to parse PDF Logo bytes");
        // flip vertically (negative y)
        // scale to 1.4 size
        // move (-200, 600)
        let mut b = b.modify_trans_matrix(1.4, 0, 0, -1.4, -200, 600);
        b.operations.extend(content.operations);

        b
    }
}
mod what {
    //! Page 2 of the deck
    use super::*;

    pub fn page(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let mut c = TextConfig::new(70, 360);
        let bullet_space = 60;
        let content_builder = ContentBuilder::new(resources)
            .title("What is a PDF?")
            .bullet_text(
                "Portable: independent of application software, hardware and operating system.",
                c.then_down(bullet_space)
            )
            .bullet_text(
                "Document: complete description of fixed-layout flat document.",
                c.then_down(bullet_space)
            ).bullet_text(
                "File: everything needed to present the document can be stored within a single file.",
                c.then_down(bullet_space)
            );
        content_builder.add_to_doc_with_page(doc, pages_id)
    }
}

mod history {
    //! Page 3 of the deck
    use super::*;
    pub fn page(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let item_vert_offset = 370;
        let item_vert_space = 50;
        let mut list_text = TextConfig::new(140, item_vert_offset);
        let mut date_text = TextConfig::new(70, item_vert_offset).with_colour(BRIGHT_RED);
        let content_builder = ContentBuilder::new(resources)
            .title("History of PDF")
            .text_with("1990", date_text.then_down(item_vert_space))
            .text_with(
                "The Camelot Project launched",
                list_text.then_down(item_vert_space),
            )
            .text_with("1993", date_text.then_down(item_vert_space))
            .text_with(
                "Portable Document Format 1.0 launched",
                list_text.then_down(item_vert_space),
            )
            .text_with("1994", date_text.then_down(item_vert_space))
            .text_with(
                "PDF 1.1 passwords and better encryption",
                list_text.then_down(item_vert_space),
            )
            .text_with("2001", date_text.then_down(item_vert_space))
            .text_with(
                "PDF 1.4 accessibility and transparency",
                list_text.then_down(item_vert_space),
            )
            .text_with("2006", date_text.then_down(item_vert_space))
            .text_with(
                "PDF 1.7 stabilisation",
                list_text.then_down(item_vert_space),
            )
            .text_with("2008", date_text.then_down(item_vert_space))
            .text_with(
                "PDF 1.7 ISO standardisation",
                list_text.then_down(item_vert_space),
            )
            .text_with("2017", date_text.then_down(item_vert_space))
            .text_with(
                "PDF 2.0 elimination of prioritary elements",
                list_text.then_down(item_vert_space),
            );

        content_builder.add_to_doc_with_page(doc, pages_id)
    }
}

mod three_documents {
    //! Page 4 of the deck

    use super::*;

    pub fn page(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let mut builder = ContentBuilder::new(resources);
        builder = add_mini_doc(builder);
        builder = add_maxi_doc(builder);
        builder = add_web_doc(builder);

        builder.add_to_doc_with_page(doc, pages_id)
    }

    fn add_mini_doc(mut b: ContentBuilder) -> ContentBuilder {
        let mini_doc = create_mini::generate_document();
        let media_box = get_media_box(&mini_doc);
        let page_id = mini_doc.page_iter().next().unwrap();
        let content = Content::decode(&mini_doc.get_page_content(page_id).unwrap()).unwrap();

        b = b
            .title("Three documents")
            .text_at(50, 350, "mini.pdf")
            .save_graphics_state()
            .cm_position(50, 160)
            .cm_scale(0.2, 0.2)
            .scolour(BLACK)
            .line_width(1.)
            .begin_path(0, 0)
            .append_straight_line(0, media_box[3])
            .append_straight_line(media_box[2], media_box[3])
            .append_straight_line(media_box[2], 0)
            .close_subpath()
            .stroke_path();

        b.operations.extend(content.operations);
        b.restore_graphics_state()
    }

    fn as_f32(o: &Object) -> f32 {
        o.as_f32()
            .or_else(|_| o.as_i64().map(|i| i as f32))
            .unwrap()
    }

    fn get_media_box(doc: &Document) -> [f32; 4] {
        let page_tree_id = doc
            .catalog()
            .and_then(|c| c.get(b"Pages"))
            .and_then(Object::as_reference)
            .unwrap();
        let media_box = doc
            .get_dictionary(page_tree_id)
            .and_then(|d| d.get(b"MediaBox"))
            .and_then(Object::as_array)
            .unwrap();

        [
            as_f32(&media_box[0]),
            as_f32(&media_box[1]),
            as_f32(&media_box[2]),
            as_f32(&media_box[3]),
        ]
    }

    fn add_maxi_doc(mut b: ContentBuilder) -> ContentBuilder {
        let maxi_doc = create_maxi::generate_document(&CreateConfig::default());
        let media_box = get_media_box(&maxi_doc);
        for (i, page_id) in maxi_doc.page_iter().enumerate() {
            let content = Content::decode(&maxi_doc.get_page_content(page_id).unwrap()).unwrap();
            b = b
                .text_at(350, 350, "maxi.pdf")
                .save_graphics_state()
                .cm_position(200 + (i as i32 * 150), 190)
                .cm_scale(0.2, 0.2)
                .scolour(BLACK)
                .line_width(1.)
                .begin_path(0, 0)
                .append_straight_line(0, media_box[3])
                .append_straight_line(media_box[2], media_box[3])
                .append_straight_line(media_box[2], 0)
                .close_subpath()
                .stroke_path();

            b.operations.extend(content.operations);
            b = b.restore_graphics_state();
        }

        b
    }

    fn add_web_doc(mut b: ContentBuilder) -> ContentBuilder {
        b = b
            .text_at(690, 350, "web.pdf")
            .save_graphics_state()
            .cm_position(670, 50)
            .cm_scale(200, 270)
            .add_xobject("Im1")
            .restore_graphics_state();
        b
    }
}

mod tools {
    //! Page 5 of the deck

    use super::*;
    pub fn page(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let mut black = TextConfig::new(160, 360);
        let mut red = TextConfig::new(70, 360).with_colour(BRIGHT_RED);
        let bullet_space = 60;
        let bullet_x = -20;
        let bullet_y = 3;
        ContentBuilder::new(resources)
            .title("PDF tools")
            .bullet(red.x + bullet_x, red.y + bullet_y)
            .text_with("qpdf:", red.then_down(bullet_space))
            .text_with(
                "useful for exploring PDF files on the command line.",
                black.then_down(bullet_space),
            )
            .bullet(red.x + bullet_x, red.y + bullet_y)
            .text_with("mutool:", red.then_down(bullet_space))
            .text_with(
                "useful for extracting fonts and images from PDF files.",
                black.then_down(bullet_space),
            )
            .bullet(red.x + bullet_x, red.y + bullet_y)
            .text_with("allsorts:", red.then_down(bullet_space))
            .text_with(
                "useful for exploring and subsetting fonts.",
                black.then_down(bullet_space),
            )
            .add_to_doc_with_page(doc, pages_id)
    }
}

mod file_structure {
    //! Page 6 of the deck

    use super::*;
    pub fn pages(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> Vec<ObjectId> {
        vec![
            page_for_min_pdf(doc, resources, pages_id),
            page_for_max_pdf(doc, resources, pages_id),
            page_for_appended(doc, resources, pages_id),
        ]
    }

    fn page_for_min_pdf(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let mut c = TextConfig::new(70, 360).with_font("F3", 20);

        let b = ContentBuilder::new(resources);

        // add text on left
        let b = b
            .title("File structure - min.pdf")
            .text_with("Header", c.then_down(50))
            .text_with("Body (68%)", c.then_down(50))
            .text_with("Cross-reference table", c.then_down(50))
            .text_with("Trailer", c.then_down(50))
            .text_with("Startxref", c.then_down(50));

        // add file render
        let b = RoundBox::new((550, 50), 300., 350.)
            .colour(GREY)
            .file_overview()
            .add_section(MAGENTA, 10) // Header
            .add_section(MUSTARD, 457 - 10) // Body
            .add_section(PALE_BLUE, 606 - 457) // xref
            .add_section(PALE_GREEN, 637 - 606) // Trailer
            .add_section(PALE_RED, 656 - 637) // Startxref
            .build(b);

        b.add_to_doc_with_page(doc, pages_id)
    }

    fn page_for_max_pdf(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
        let mut c = TextConfig::new(70, 360).with_font("F3", 20);

        let b = ContentBuilder::new(resources);

        // add text on left
        let b = b
            .title("File structure - max.pdf")
            .text_with("Header", c.then_down(50))
            .text_with("Body (99.6%)", c.then_down(50))
            .text_with("Cross-reference table", c.then_down(50))
            .text_with("Trailer", c.then_down(50))
            .text_with("Startxref", c.then_down(50));

        // add file render
        let b = RoundBox::new((550, 50), 300., 350.)
            .colour(GREY)
            .file_overview()
            .add_section(MAGENTA, 10) // Header
            .add_section(MUSTARD, 104371 - 10) // Body
            .add_section(PALE_BLUE, 104701 - 104371) // xref
            .add_section(PALE_GREEN, 104734 - 104701) // Trailer
            .add_section(PALE_RED, 104755 - 104734) // Startxref
            .build(b);

        b.add_to_doc_with_page(doc, pages_id)
    }

    fn page_for_appended(
        doc: &mut Document,
        resources: &Resources,
        pages_id: ObjectId,
    ) -> ObjectId {
        let mut c = TextConfig::new(70, 360).with_font("F3", 15);
        let v = 30;

        let b = ContentBuilder::new(resources);

        // add text on left
        let b = b
            .title("File structure - appended.pdf")
            .text_with("Header", c.then_down(v))
            .text_with("Body", c.then_down(v))
            .text_with("Cross-reference table", c.then_down(v))
            .text_with("Trailer", c.then_down(v))
            .text_with("Startxref", c.then_down(v))
            .text_with("Body", c.then_down(v))
            .text_with("Cross-reference table", c.then_down(v))
            .text_with("Trailer", c.then_down(v))
            .text_with("Startxref", c.then_down(v));

        // add file render
        let b = RoundBox::new((550, 50), 300., 350.)
            .colour(GREY)
            .file_overview()
            // first section
            .add_section(MAGENTA, 10) // Header
            .add_section(MUSTARD, 447) // Body
            .add_section(PALE_BLUE, 149) // xref
            .add_section(PALE_GREEN, 31) // Trailer
            .add_section(PALE_RED, 19) // Startxref
            // second section
            .add_section(MUSTARD, 100) // Body
            .add_section(PALE_BLUE, 161) // xref
            .add_section(PALE_GREEN, 31) // Trailer
            .add_section(PALE_RED, 19) // Startxref
            .build(b);

        b.add_to_doc_with_page(doc, pages_id)
    }
}

mod doc_structure {
    //! Page 7 of the deck

    use super::*;

    pub fn pages(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> Vec<ObjectId> {
        vec![
            page_for_structure(doc, resources, pages_id),
            page_with_file_location(doc, resources, pages_id),
            page_with_resources_highlighted(doc, resources, pages_id),
            page_with_page_tree(doc, resources, pages_id),
        ]
    }

    fn page_for_structure(
        doc: &mut Document,
        resources: &Resources,
        pages_id: ObjectId,
    ) -> ObjectId {
        let b = ContentBuilder::new(resources);

        let b = b.title("Document structure");

        let offset_x = 450;

        // Boxes with text
        let mut round_box = RoundBox::new((600, 350), 100., 50.).radius(5.);
        let c = TextConfig::new(70, 360).with_font("F3", 15);

        let b = round_box.clone().origin((offset_x - 20, 350)).build(b);
        let b = b.text_with("Trailer", c.at((offset_x, 370)));

        let b = round_box.origin((offset_x + 80, 280)).build(b);
        let b = b.text_with("Info Dict", c.at((offset_x + 90, 300)));

        let b = round_box.origin((offset_x - 120, 280)).build(b);
        let b = b.text_with("Catalog", c.at((offset_x - 100, 300)));

        let b = round_box.origin((offset_x - 240, 210)).build(b);
        let b = b.text_with("Page Tree", c.at((offset_x - 230, 230)));

        let page_x = offset_x - 360;
        let page_y = 130;
        let b = round_box.origin((offset_x + 50, page_y)).build(b);
        let b = b.text_with("Resources", c.at((offset_x + 60, page_y + 20)));

        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 1", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        let page_x = offset_x - 120;
        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 2", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        // Connecting lines
        let b = b
            .save_graphics_state()
            // Trailer -> Info Dict
            .draw_line((offset_x + 30, 350), (offset_x + 130, 330))
            // Trailer -> Catalog
            .draw_line((offset_x + 30, 350), (offset_x - 30, 330))
            // Catalog -> Page Tree
            .draw_line((offset_x - 70, 280), (offset_x - 190, 260))
            .draw_line((offset_x - 190, 210), (offset_x + 100, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 310, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 70, page_y + 50))
            .draw_line((offset_x - 310, 130), (offset_x - 365, 110))
            .draw_line((offset_x - 310, 130), (offset_x - 255, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 125, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 15, 110))
            .stroke_path()
            .restore_graphics_state();

        b.add_to_doc_with_page(doc, pages_id)
    }

    fn page_with_file_location(
        doc: &mut Document,
        resources: &Resources,
        pages_id: ObjectId,
    ) -> ObjectId {
        let b = ContentBuilder::new(resources);

        let b = b.title("Document structure - file structure");

        let offset_x = 450;

        // Boxes with text
        let mut round_box = RoundBox::new((600, 350), 100., 50.).radius(5.);
        let c = TextConfig::new(70, 360).with_font("F3", 15);

        let b = round_box
            .clone()
            .origin((offset_x - 20, 350))
            .fill(PALE_GREEN)
            .build(b);
        let b = b.text_with("Trailer", c.at((offset_x, 370)));

        let b = round_box
            .origin((offset_x + 80, 280))
            .fill(MUSTARD)
            .build(b);
        let b = b.text_with("Info Dict", c.at((offset_x + 90, 300)));

        let b = round_box.origin((offset_x - 120, 280)).build(b);
        let b = b.text_with("Catalog", c.at((offset_x - 100, 300)));

        let b = round_box.origin((offset_x - 240, 210)).build(b);
        let b = b.text_with("Page Tree", c.at((offset_x - 230, 230)));

        let page_x = offset_x - 360;
        let page_y = 130;
        let b = round_box.origin((offset_x + 50, page_y)).build(b);
        let b = b.text_with("Resources", c.at((offset_x + 60, page_y + 20)));

        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 1", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        let page_x = offset_x - 120;
        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 2", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        // Connecting lines
        let b = b
            .save_graphics_state()
            // Trailer -> Info Dict
            .draw_line((offset_x + 30, 350), (offset_x + 130, 330))
            // Trailer -> Catalog
            .draw_line((offset_x + 30, 350), (offset_x - 30, 330))
            // Catalog -> Page Tree
            .draw_line((offset_x - 70, 280), (offset_x - 190, 260))
            .draw_line((offset_x - 190, 210), (offset_x + 100, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 310, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 70, page_y + 50))
            .draw_line((offset_x - 310, 130), (offset_x - 365, 110))
            .draw_line((offset_x - 310, 130), (offset_x - 255, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 125, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 15, 110))
            .stroke_path()
            .restore_graphics_state();

        let b = RoundBox::new((700, 50), 200., 350.)
            .colour(GREY)
            .file_overview()
            .add_section(MAGENTA, 10) // Header
            .add_section(MUSTARD, 350) // Body
            .add_section(PALE_BLUE, 100) // xref
            .add_section(PALE_GREEN, 50) // Trailer
            .add_section(PALE_RED, 50) // Startxref
            .build(b);

        b.add_to_doc_with_page(doc, pages_id)
    }

    fn page_with_resources_highlighted(
        doc: &mut Document,
        resources: &Resources,
        pages_id: ObjectId,
    ) -> ObjectId {
        let b = ContentBuilder::new(resources);

        let b = b.title("Document structure - resources");

        let offset_x = 450;

        // Boxes with text
        let mut round_box = RoundBox::new((600, 350), 100., 50.).radius(5.);
        let c = TextConfig::new(70, 360).with_font("F3", 15);

        let b = round_box.clone().origin((offset_x - 20, 350)).build(b);
        let b = b.text_with("Trailer", c.at((offset_x, 370)));

        let b = round_box.origin((offset_x + 80, 280)).build(b);
        let b = b.text_with("Info Dict", c.at((offset_x + 90, 300)));

        let b = round_box.origin((offset_x - 120, 280)).build(b);
        let b = b.text_with("Catalog", c.at((offset_x - 100, 300)));

        let b = round_box.origin((offset_x - 240, 210)).build(b);
        let b = b.text_with("Page Tree", c.at((offset_x - 230, 230)));

        let highlight = lighter(PALE_GREEN, 0.2);
        let page_x = offset_x - 360;
        let page_y = 130;
        let b = round_box
            .origin((offset_x + 50, page_y))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Resources", c.at((offset_x + 60, page_y + 20)));

        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 1", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box
            .origin((page_x + 55, 60))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        let page_x = offset_x - 120;
        let b = round_box.origin((page_x, page_y)).build(b);
        let b = b.text_with("Page 2", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box
            .origin((page_x + 55, 60))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        // Connecting lines
        let b = b
            .save_graphics_state()
            // Trailer -> Info Dict
            .draw_line((offset_x + 30, 350), (offset_x + 130, 330))
            // Trailer -> Catalog
            .draw_line((offset_x + 30, 350), (offset_x - 30, 330))
            // Catalog -> Page Tree
            .draw_line((offset_x - 70, 280), (offset_x - 190, 260))
            .draw_line((offset_x - 190, 210), (offset_x + 100, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 310, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 70, page_y + 50))
            .draw_line((offset_x - 310, 130), (offset_x - 365, 110))
            .draw_line((offset_x - 310, 130), (offset_x - 255, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 125, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 15, 110))
            .stroke_path()
            .restore_graphics_state();

        b.add_to_doc_with_page(doc, pages_id)
    }

    fn page_with_page_tree(
        doc: &mut Document,
        resources: &Resources,
        pages_id: ObjectId,
    ) -> ObjectId {
        let b = ContentBuilder::new(resources);

        let b = b.title("Document structure - page tree");

        let offset_x = 450;

        // Boxes with text
        let mut round_box = RoundBox::new((600, 350), 100., 50.).radius(5.);
        let c = TextConfig::new(70, 360).with_font("F3", 15);

        let b = round_box.clone().origin((offset_x - 20, 350)).build(b);
        let b = b.text_with("Trailer", c.at((offset_x, 370)));

        let b = round_box.origin((offset_x + 80, 280)).build(b);
        let b = b.text_with("Info Dict", c.at((offset_x + 90, 300)));

        let b = round_box.origin((offset_x - 120, 280)).build(b);
        let b = b.text_with("Catalog", c.at((offset_x - 100, 300)));

        let highlight = lighter(PALE_GREEN, 0.2);
        let b = round_box
            .origin((offset_x - 240, 210))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Page Tree", c.at((offset_x - 230, 230)));

        let page_x = offset_x - 360;
        let page_y = 130;
        let b = round_box.origin((offset_x + 50, page_y)).build(b);
        let b = b.text_with("Resources", c.at((offset_x + 60, page_y + 20)));

        let b = round_box
            .origin((page_x, page_y))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Page 1", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        let page_x = offset_x - 120;
        let b = round_box
            .origin((page_x, page_y))
            .clone()
            .fill(highlight)
            .build(b);
        let b = b.text_with("Page 2", c.at((page_x + 25, page_y + 20)));

        let b = round_box.origin((page_x - 55, 60)).build(b);
        let b = b.text_with("Content", c.at((page_x - 38, 80)));

        let b = round_box.origin((page_x + 55, 60)).build(b);
        let b = b.text_with("Resources", c.at((page_x + 65, 80)));

        // Connecting lines
        let b = b
            .save_graphics_state()
            // Trailer -> Info Dict
            .draw_line((offset_x + 30, 350), (offset_x + 130, 330))
            // Trailer -> Catalog
            .draw_line((offset_x + 30, 350), (offset_x - 30, 330))
            // Catalog -> Page Tree
            .draw_line((offset_x - 70, 280), (offset_x - 190, 260))
            .draw_line((offset_x - 190, 210), (offset_x + 100, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 310, page_y + 50))
            .draw_line((offset_x - 190, 210), (offset_x - 70, page_y + 50))
            .draw_line((offset_x - 310, 130), (offset_x - 365, 110))
            .draw_line((offset_x - 310, 130), (offset_x - 255, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 125, 110))
            .draw_line((offset_x - 70, 130), (offset_x - 15, 110))
            .stroke_path()
            .restore_graphics_state();

        let b = b
            .save_graphics_state()
            .cm_position(750, 350)
            .cm_scale(0.7, 0.7);

        let ox = 70;
        let oy = 80;

        let gy = |n: i32| -oy / 2 * n - 100 / 2 * n;

        let b = round_box.origin((-10, 0)).build(b);
        let b = b.text_with("Page Tree", c.at((0, 20)));

        let b = round_box.origin((-ox, -100)).build(b);
        let b = b.text_with("Page Tree", c.at((-ox + 10, -80)));

        let b = round_box.origin((-ox, gy(2))).build(b);
        let b = b.text_with("Resources", c.at((-ox + 10, gy(2) + 20)));

        let b = round_box.origin((-ox, gy(3))).build(b);
        let b = b.text_with("Page 1", c.at((-ox + 20, gy(3) + 20)));

        let b = round_box.origin((-ox, gy(4))).build(b);
        let b = b.text_with("Page 2", c.at((-ox + 20, gy(4) + 20)));

        let b = round_box.origin((ox, -100)).build(b);
        let b = b.text_with("Page Tree", c.at((ox + 10, -80)));

        let b = round_box.origin((ox, gy(2))).build(b);
        let b = b.text_with("Resources", c.at((ox + 10, gy(2) + 20)));

        let b = round_box.origin((ox, gy(3))).build(b);
        let b = b.text_with("Page 3", c.at((ox + 20, gy(3) + 20)));

        let b = round_box.origin((ox, gy(4))).build(b);
        let b = b.text_with("Page 4", c.at((ox + 20, gy(4) + 20)));

        let gy = |n: i32| -100 / 2 * (n - 1) - oy / 2 * n - oy / 4;

        let b = b
            .save_graphics_state()
            // From root
            .draw_line((ox / 2, 0), (-ox / 2, -50))
            .draw_line((ox / 2, 0), ((ox / 2) * 3, -50))
            // Left side
            .draw_line((-ox, -75), (-ox - 20, -75))
            .draw_line((-ox - 20, -75), (-ox - 20, gy(4)))
            .draw_line((-ox - 20, gy(2)), (-ox, gy(2)))
            .draw_line((-ox - 20, gy(3)), (-ox, gy(3)))
            .draw_line((-ox - 20, gy(4)), (-ox, gy(4)))
            // Right side
            .draw_line((ox + 100, -75), (ox + 100 + 20, -75))
            .draw_line((ox + 100 + 20, -75), (ox + 100 + 20, gy(4)))
            .draw_line((ox + 100, gy(2)), (ox + 100 + 20, gy(2)))
            .draw_line((ox + 100, gy(3)), (ox + 100 + 20, gy(3)))
            .draw_line((ox + 100, gy(4)), (ox + 100 + 20, gy(4)))
            .stroke_path()
            .restore_graphics_state();

        let b = b.restore_graphics_state();

        b.add_to_doc_with_page(doc, pages_id)
    }
}

type Coord = (i32, i32);

trait ContentBuilderAdditions {
    fn bullet(self, x: i32, y: i32) -> Self;
    fn bullet_text(self, text: &str, config: TextConfig) -> Self;
    fn title_text(self, text: &str) -> Self;
    fn title(self, text: &str) -> Self;
    fn text_at(self, x: i32, y: i32, text: &str) -> Self;
    fn text_with(self, text: &str, config: TextConfig) -> Self;
    fn thick_blue_line(self, from: Coord, to: Coord) -> Self;
    fn thin_blue_line(self, from: Coord, to: Coord) -> Self;
}

impl<'a> ContentBuilderAdditions for ContentBuilder<'a> {
    fn bullet(self, x: i32, y: i32) -> Self {
        let size = 6;
        self.save_graphics_state()
            .colour(LIGHT_BLUE)
            .begin_path(x, y)
            .append_straight_line(x, y + size)
            .append_straight_line(x + size, y + size)
            .append_straight_line(x + size, y)
            .append_straight_line(x, y)
            .fill_path()
            .restore_graphics_state()
    }

    fn bullet_text(self, text: &str, config: TextConfig) -> Self {
        self.bullet(config.x, config.y)
            .text_at(config.x + 15, config.y - 3, text)
    }

    fn title_text(self, text: &str) -> Self {
        self.begin_text()
            .font("F1", 38)
            .text_position(50, 450)
            .colour(DARK_BLUE)
            .text(text)
            .end_text()
    }

    fn title(self, text: &str) -> Self {
        self.title_text(text).thick_blue_line((50, 440), (900, 440))
    }

    fn thick_blue_line(self, from: Coord, to: Coord) -> Self {
        blue_line(self, from, to, 1.)
    }

    fn thin_blue_line(self, from: Coord, to: Coord) -> Self {
        blue_line(self, from, to, 0.5)
    }

    fn text_at(self, x: i32, y: i32, text: &str) -> Self {
        self.text_with(text, TextConfig::new(x, y))
    }

    fn text_with(self, text: &str, config: TextConfig) -> Self {
        let (font_name, font_size) = config.font.unwrap_or((String::from("F1"), 20));
        self.begin_text()
            .font(&font_name, font_size)
            .text_position(config.x, config.y)
            .colour(config.colour.unwrap_or(DARK_BLUE))
            .text(text)
            .end_text()
    }
}

#[derive(Default, Clone)]
struct TextConfig {
    x: i32,
    y: i32,
    font: Option<(String, u32)>,
    colour: Option<Colour>,
}

impl TextConfig {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
    fn then_down(&mut self, y: i32) -> Self {
        let result = self.clone();
        self.y -= y;
        result
    }

    fn at(&self, origin: (i32, i32)) -> Self {
        Self {
            x: origin.0,
            y: origin.1,
            ..self.clone()
        }
    }

    fn with_font(self, font: &str, size: u32) -> Self {
        Self {
            font: Some((String::from(font), size)),
            ..self
        }
    }

    fn with_colour(self, colour: Colour) -> Self {
        Self {
            colour: Some(colour),
            ..self
        }
    }
}

#[derive(Clone)]
struct RoundBox {
    origin: (i32, i32),
    width: f32,
    height: f32,
    radius: f32,
    line_width: f32,
    stroke_colour: Colour,
    fill_colour: Option<Colour>,
}

impl RoundBox {
    fn new(origin: (i32, i32), width: f32, height: f32) -> Self {
        Self {
            origin,
            width,
            height,
            radius: 15.,
            line_width: 1.,
            stroke_colour: BLACK,
            fill_colour: None,
        }
    }

    fn origin(&mut self, origin: (i32, i32)) -> &mut Self {
        self.origin = origin;
        self
    }

    fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    fn colour(mut self, colour: Colour) -> Self {
        self.stroke_colour = colour;
        self
    }

    fn fill(&mut self, colour: Colour) -> &mut Self {
        self.fill_colour = Some(colour);
        self
    }

    fn no_fill(&mut self) -> &mut Self {
        self.fill_colour = None;
        self
    }

    fn build<'b>(&self, b: ContentBuilder<'b>) -> ContentBuilder<'b> {
        let b = self.setup_state(b);
        let b = self.draw_box(b);
        b.restore_graphics_state()
    }

    fn setup_state<'b>(&self, b: ContentBuilder<'b>) -> ContentBuilder<'b> {
        let b = b
            .save_graphics_state()
            .cm_position(self.origin.0, self.origin.1)
            .scolour(self.stroke_colour)
            .line_width(self.line_width);
        if let Some(colour) = self.fill_colour {
            b.colour(colour)
        } else {
            b
        }
    }

    fn draw_box<'b>(&self, b: ContentBuilder<'b>) -> ContentBuilder<'b> {
        let k: f32 = 4.0 / 3.0 * (f32::sqrt(2.0) - 1.0);
        let radius = self.radius;
        let width = self.width;
        let height = self.height;
        let b = b
            .begin_path(radius, 0)
            .append_straight_line(width - radius, 0)
            .append_curve(
                // (width - r, 0) to (width, r)
                (width - radius) + radius * k,
                0,
                width,
                radius * k,
                width,
                0. + radius,
            )
            .append_straight_line(width, height - radius)
            .append_curve(
                // (width, height - r) to (width - r, height)
                width,
                (height - radius) + radius * k,
                (width - radius) + radius * k,
                height,
                width - radius,
                height,
            )
            .append_straight_line(radius, height)
            .append_curve(
                // (r, height) to (0, height - r)
                radius * k,
                height,
                0,
                (height - radius) + radius * k,
                0,
                height - radius,
            )
            .append_straight_line(0, radius)
            .append_curve(
                // (0, r) to (r, 0)
                0,
                radius * k,
                radius * k,
                0,
                radius,
                0,
            );

        if self.fill_colour.is_some() {
            b.fill_stroke_path()
        } else {
            b.stroke_path()
        }
    }

    fn file_overview(self) -> FileOverview {
        FileOverview::new(self)
    }
}

#[derive(Debug)]
struct Section {
    colour: Colour,
    size: usize,
}

struct FileOverview {
    pub round_box: RoundBox,
    num_lines: usize,
    sections: Vec<Section>,
}

impl FileOverview {
    fn new(round_box: RoundBox) -> Self {
        Self {
            round_box,
            num_lines: 10,
            sections: vec![],
        }
    }

    fn add_section(mut self, colour: Colour, size: usize) -> Self {
        self.sections.push(Section { colour, size });
        self
    }

    /// Draw lines for the file overview
    fn draw_lines<'b>(&self, b: ContentBuilder<'b>) -> ContentBuilder<'b> {
        let total_ticks = self.sections.iter().map(|s| s.size).sum::<usize>() as f32;
        let lines_per_tick = self.num_lines as f32 / total_ticks;
        let y_margin = self.round_box.radius * 1.;
        let x_margin = self.round_box.width * 0.05;
        let inner_height = self.round_box.height - y_margin * 2.;
        let line_height = inner_height / self.num_lines as f32;
        let line_margin = line_height * 0.1;
        let stroke_width = line_height - line_margin * 2.;
        let line_length = self.round_box.width - x_margin * 2.;

        let mut y = y_margin + (self.num_lines as f32 - 0.5) * line_height;
        let mut b = b;
        let mut used_width: f32 = 0.;
        let mut lines_done = 0;

        // draw coloured lines for each section
        for section in &self.sections {
            // how many lines to draw
            let mut lines = section.size as f32 * lines_per_tick;

            // is there any lefttover space in a line from the previous section?
            if used_width > 0. {
                let end = x_margin + line_length * f32::min(used_width + lines, 1.);

                b = b
                    .save_graphics_state()
                    .scolour(section.colour)
                    .line_width(stroke_width)
                    .begin_path(x_margin + line_length * used_width, y)
                    .append_straight_line(end, y)
                    .stroke_path()
                    .restore_graphics_state();

                // if the used width is less than a full line move on to the next section
                if lines + used_width < 1. {
                    used_width += lines;
                    continue;
                } else {
                    lines -= 1. - used_width;
                    y -= line_height;
                    lines_done += 1;
                }
            }

            // draw full lines
            for _ in 0..lines.floor() as usize {
                b = b
                    .save_graphics_state()
                    .scolour(section.colour)
                    .line_width(stroke_width)
                    .begin_path(x_margin, y)
                    .append_straight_line(x_margin + line_length, y)
                    .stroke_path()
                    .restore_graphics_state();
                y -= line_height;
                lines_done += 1;
            }

            // draw a partial line for any remainder
            used_width = lines.fract();
            if used_width > 0. && lines_done < self.num_lines {
                b = b
                    .save_graphics_state()
                    .scolour(section.colour)
                    .line_width(stroke_width)
                    .begin_path(x_margin, y)
                    .append_straight_line(x_margin + line_length * used_width, y)
                    .stroke_path()
                    .restore_graphics_state();
            }
        }
        b
    }

    fn build(self, b: ContentBuilder) -> ContentBuilder {
        let b = self.round_box.setup_state(b);
        let b = self.draw_lines(b);
        let b = self.round_box.draw_box(b);
        b.restore_graphics_state()
    }
}

fn blue_line(b: ContentBuilder, from: Coord, to: Coord, width: f32) -> ContentBuilder {
    let (x1, y1) = from;
    let (x2, y2) = to;
    b.save_graphics_state()
        .scolour(LIGHT_BLUE)
        .line_width(width)
        .begin_path(x1, y1)
        .append_straight_line(x2, y2)
        .stroke_path()
        .restore_graphics_state()
}
