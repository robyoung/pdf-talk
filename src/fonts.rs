use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryFrom;

use lopdf::{dictionary, Document, Object, ObjectId, Stream};
use ttf_parser::{Face, GlyphId};

pub fn type0(font: &[u8]) -> FontType0Builder {
    FontType0Builder {
        font_data: font.to_vec(),
    }
}

pub struct FontType0Builder {
    font_data: Vec<u8>,
}

impl FontType0Builder {
    pub fn add_to_doc(self, doc: &mut Document) -> ObjectId {
        let face = Face::parse(&self.font_data, 0).expect("could not parse font data");
        let glyph_ids = get_glyph_id_to_char_map(&face);
        let cmap_info = get_cmap_info(&face, &glyph_ids);

        // add font stream
        let stream_id = doc.add_object(
            Stream::new(
                dictionary! {
                    "Length1" => self.font_data.len() as u32,
                },
                self.font_data.clone(),
            )
            .with_compression(false),
        );

        // add font descriptor object
        let descriptor_id = doc.add_object(dictionary! {
            "Type" => "FontDescriptor",
            "FontName" => "FontName",
            "Flags" => 32,
            "FontBBox" => Object::Array(vec![
                Object::Integer(0),
                Object::Integer(cmap_info.max_height as i64),
                Object::Integer(cmap_info.total_width as i64),
                Object::Integer(cmap_info.max_height as i64), //
            ]),
            "ItalicAngle" => 0,
            "Ascent" => face.ascender(),
            "Descent" => face.descender(),
            "CapHeight" => face.ascender(),
            "StemV" => 80,
            "MissingWidth" => 500,
            "FontFile2" => stream_id,
        });
        // add descendant font object
        let descendant_font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "CIDFontType2",
            "BaseFont" => "FontName",
            "CIDSystemInfo" => dictionary! {
                "Registry" => "Adobe",
                "Ordering" => "Identity",
                "Supplement" => 0,
            },
            "FontDescriptor" => descriptor_id,
            "DW" => 1000,
        });
        let to_unicode_id = create_to_unicode(doc, &cmap_info);

        // add font object
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type0",
            "BaseFont" => "FontName",
            "Encoding" => "Identity-H",
            "DescendantFonts" => vec![descendant_font_id.into()],
            "ToUnicode" => to_unicode_id,
        });
        font_id
    }
}

fn create_to_unicode(doc: &mut Document, cmap_info: &CmapInfo) -> ObjectId {
    let all_cmap_blocks = create_cmap_blocks(cmap_info);
    let cid_to_unicode_map = create_cid_unicode_map("FontName".to_owned(), all_cmap_blocks);

    let stream_id = doc.add_object(Stream::new(
        dictionary! {},
        cid_to_unicode_map.as_bytes().to_vec(),
    ));
    stream_id
}

fn get_glyph_id_to_char_map(face: &Face) -> HashMap<u16, char> {
    let mut map = HashMap::new();
    let subtables = face
        .tables()
        .cmap
        .expect("no cmap found")
        .subtables
        .clone()
        .into_iter()
        .filter(|s| s.is_unicode());
    for subtable in subtables {
        subtable.codepoints(|c| {
            if let Ok(ch) = char::try_from(c) {
                if let Some(idx) = subtable.glyph_index(c).filter(|idx| idx.0 > 0) {
                    map.insert(idx.0, ch);
                }
            }
        });
    }
    map
}

struct CmapInfo {
    max_height: u32,
    total_width: u32,
    char_dims: BTreeMap<GlyphId, (char, u32, u32)>,
}

fn get_cmap_info(face: &Face, glyph_ids: &HashMap<u16, char>) -> CmapInfo {
    let mut info = CmapInfo {
        max_height: 0,
        total_width: 0,
        char_dims: BTreeMap::new(),
    };
    info.char_dims
        .insert(GlyphId(0), (char::from_u32(0).unwrap(), 0, 0));
    // lifted from printpdf
    let descender = face.descender();
    for (glyph_id, &c) in glyph_ids.iter() {
        let glyph_id = GlyphId(*glyph_id);
        if let Some(width) = face.glyph_hor_advance(glyph_id) {
            let height = face
                .glyph_bounding_box(glyph_id)
                .map(|bbox| bbox.y_max - bbox.y_min - descender)
                .unwrap_or(1000) as u32;
            info.total_width += width as u32;
            if height > info.max_height {
                info.max_height = height;
            }
            info.char_dims
                .insert(glyph_id, (c, width as u32, height as u32));
        }
    }
    info
}

fn create_cid_unicode_map(face_name: String, all_cmap_blocks: Vec<CmapBlock>) -> String {
    let mut cid_to_unicode_map =
        format!(include_str!("../assets/gid_to_unicode_beg.txt"), face_name);

    for cmap_block in all_cmap_blocks
        .into_iter()
        .filter(|block| !block.is_empty() || block.len() < 100)
    {
        cid_to_unicode_map.push_str(format!("{} beginbfchar\r\n", cmap_block.len()).as_str());
        for (glyph_id, unicode) in cmap_block {
            cid_to_unicode_map.push_str(format!("<{glyph_id:04x}> <{unicode:04x}>\n").as_str());
        }
        cid_to_unicode_map.push_str("endbfchar\r\n");
    }

    cid_to_unicode_map.push_str(include_str!("../assets/gid_to_unicode_end.txt"));
    cid_to_unicode_map
}

type UnicodeCodePoint = u32;
type CmapBlock = Vec<(u32, UnicodeCodePoint)>;

fn create_cmap_blocks(cmap_info: &CmapInfo) -> Vec<CmapBlock> {
    // lifted from printpdf
    let mut current_first_bit = 0_u16;
    let mut all_cmap_blocks = Vec::new();
    let mut current_cmap_block = Vec::new();
    for (&glyph_id, &(unicode, _, _)) in cmap_info.char_dims.iter() {
        if (glyph_id.0 >> 8) as u16 != current_first_bit || current_cmap_block.len() >= 100 {
            all_cmap_blocks.push(current_cmap_block.clone());
            current_cmap_block = Vec::new();
            current_first_bit = (glyph_id.0 >> 8) as u16;
        }

        current_cmap_block.push((glyph_id.0 as u32, unicode as u32));
    }
    all_cmap_blocks.push(current_cmap_block);
    all_cmap_blocks
}

pub fn type1(base_font: &str) -> FontType1Builder {
    FontType1Builder {
        base_font: base_font.to_owned(),
    }
}

#[derive(Default)]
pub struct FontType1Builder {
    base_font: String,
}

impl FontType1Builder {
    pub fn add_to_doc(self, doc: &mut Document) -> ObjectId {
        doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => self.base_font,
        })
    }
}

pub fn true_type(font: &[u8]) -> FontTrueTypeBuilder {
    FontTrueTypeBuilder {
        font_data: font.to_vec(),
    }
}

pub struct FontTrueTypeBuilder {
    font_data: Vec<u8>,
}

impl FontTrueTypeBuilder {
    pub fn add_to_doc(self, doc: &mut Document) -> ObjectId {
        // add font stream
        let stream_id = doc.add_object(Stream::new(dictionary! {}, self.font_data));

        // add font descriptor object
        let descriptor_id = doc.add_object(dictionary! {
            "Type" => "FontDescriptor",
            "FontName" => "FontName",
            "Flags" => 32,
            "FontBBox" => Object::Array(vec![Object::Integer(0), Object::Integer(-200), Object::Integer(1000), Object::Integer(800),]),
            "ItalicAngle" => 0,
            "Ascent" => 800,
            "Descent" => -200,
            "CapHeight" => 700,
            "StemV" => 80,
            "MissingWidth" => 500,
            "FontFile2" => stream_id,
        });
        // add font object
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "TrueType",
            "BaseFont" => "FontName",
            "FirstChar" => 0,
            "LastChar" => 0,
            "Widths" => vec![],
            "FontDescriptor" => descriptor_id,

        });
        font_id
    }
}
