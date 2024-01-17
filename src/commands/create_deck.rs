use lopdf::{content::Content, dictionary, xobject, Document, Object, ObjectId};

use crate::{
    config::CreateConfig,
    document::{Colour, ContentBuilder, DocumentAdditions, Resources},
    fonts::FontType0Builder,
};

const DARK_BLUE: Colour = (0.106, 0.259, 0.471);
const LIGHT_BLUE: Colour = (0., 0.624, 0.855);

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

    let font_ref = FontType0Builder::from_file("assets/Georgia.ttf")
        .expect("could not read font file")
        .add_to_doc(&mut doc);

    let pages_id = doc.new_object_id();

    let mut resources = Resources::default();
    resources.set_font("F1", font_ref);
    let image_stream = xobject::image("assets/tnt-logo.png").expect("could not read tnt logo");
    let image_id = doc.add_object(image_stream);
    resources.set_xobject("Im1", image_id);

    let resources_id = resources.add_to_doc(&mut doc);
    let page_id = page1(&mut doc, &resources, pages_id);

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

fn page1(doc: &mut Document, resources: &Resources, pages_id: ObjectId) -> ObjectId {
    let content_builder = ContentBuilder::new(resources)
        // write text
        .begin_text()
        .font("F1", 38)
        .move_to(530, 350)
        .colour(DARK_BLUE) // dark blue
        .text("What even is a PDF?")
        .end_text()
        .begin_text()
        .font("F1", 17)
        .move_to(770, 100)
        .colour(LIGHT_BLUE) // pale blue
        .text("January 2024")
        .end_text();

    let content_builder = add_tnt_logo(content_builder);
    let content_builder = add_pdf_logo(content_builder);

    let content_id = content_builder.add_to_doc(doc);

    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    })
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
        .add_xobject("Im1")
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
        .save_graphics_state()
        .scolour(LIGHT_BLUE)
        .line_width(0.5)
        .begin_path(500, 80)
        .append_straight_line(880, 80)
        .stroke_path()
        .restore_graphics_state()
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
    let content = Content::decode(&PDF_LOGO.as_bytes()).expect("unable to parse PDF Logo bytes");
    // flip vertically (negative y)
    // scale to 1.4 size
    // move (-200, 600)
    let mut b = b.modify_trans_matrix(1.4, 0, 0, -1.4, -200, 600);
    b.operations.extend(content.operations.into_iter());

    b
}
