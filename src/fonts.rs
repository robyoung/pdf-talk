use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryFrom;

use lopdf::Dictionary;
use lopdf::StringFormat;
use lopdf::{dictionary, Document, Object, ObjectId, Stream};
use owned_ttf_parser::{AsFaceRef, Face, GlyphId, OwnedFace};

// TODO @robyoung refactor this to make the usage clearer.
pub trait FontReference {
    fn object_id(&self) -> ObjectId;
    fn render_text(&self, text: &str) -> Vec<Object>;
}

#[derive(Default, Clone)]
pub struct InternalFontReference {
    object_id: ObjectId,
}

impl FontReference for InternalFontReference {
    fn object_id(&self) -> ObjectId {
        self.object_id
    }

    fn render_text(&self, text: &str) -> Vec<Object> {
        vec![Object::string_literal(text)]
    }
}

pub struct ExternalFontReference {
    object_id: ObjectId,
    face: OwnedFace,
}

impl ExternalFontReference {
    fn new(object_id: ObjectId, font_data: Vec<u8>) -> Self {
        Self {
            object_id,
            face: OwnedFace::from_vec(font_data, 0).unwrap(),
        }
    }
}

impl FontReference for ExternalFontReference {
    fn object_id(&self) -> ObjectId {
        self.object_id
    }

    fn render_text(&self, text: &str) -> Vec<Object> {
        vec![Object::String(
            text.chars()
                .filter_map(|ch| self.face.as_face_ref().glyph_index(ch).map(|gid| gid.0))
                .flat_map(|x| vec![(x >> 8) as u8, (x & 255) as u8])
                .collect::<Vec<u8>>(),
            StringFormat::Hexadecimal,
        )]
    }
}

pub fn type0(font: &[u8]) -> FontType0Builder {
    FontType0Builder::new(font)
}

pub struct FontType0Builder {
    font_data: Vec<u8>,
}

impl FontType0Builder {
    pub fn new(font: &[u8]) -> Self {
        Self {
            font_data: font.to_vec(),
        }
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        Ok(Self::new(&std::fs::read(path)?))
    }

    pub fn add_to_doc(self, doc: &mut Document) -> ExternalFontReference {
        let face = Face::parse(&self.font_data, 0).expect("could not parse font data");
        let glyph_ids = get_glyph_id_to_char_map(&face);
        let cmap_info = get_cmap_info(&face, &glyph_ids);
        let font_name = "F0";

        // add font stream
        let stream_id = doc.add_object(
            Stream::new(
                dictionary! {
                    "Length1" => self.font_data.len() as u32,
                },
                self.font_data.clone(),
            )
            .with_compression(true),
        );

        // add font descriptor object
        let descriptor_id = doc.add_object(dictionary! {
            "Type" => "FontDescriptor",
            "FontName" => font_name,
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
            "FontFile2" => stream_id,
        });
        // add descendant font object
        let descendant_font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "CIDFontType2",
            "BaseFont" => font_name,
            "CIDSystemInfo" => dictionary! {
                "Registry" => Object::String("Adobe".into(), StringFormat::Literal),
                "Ordering" => Object::String("Identity".into(), StringFormat::Literal),
                "Supplement" => 0,
            },
            "FontDescriptor" => descriptor_id,
            "W" => create_width_list(&face),
            "DW" => 1000,
        });
        let to_unicode_id = create_to_unicode(doc, &cmap_info, font_name);

        // add font object
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type0",
            "BaseFont" => font_name,
            "Encoding" => "Identity-H",
            "DescendantFonts" => vec![descendant_font_id.into()],
            "ToUnicode" => to_unicode_id,
        });
        ExternalFontReference::new(font_id, self.font_data)
    }
}

fn create_to_unicode(doc: &mut Document, cmap_info: &CmapInfo, face_name: &str) -> ObjectId {
    let all_cmap_blocks = create_cmap_blocks(cmap_info);
    let cid_to_unicode_map = create_cid_unicode_map(face_name.to_owned(), all_cmap_blocks);

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
    cmap: BTreeMap<GlyphId, char>,
}

fn get_cmap_info(face: &Face, glyph_ids: &HashMap<u16, char>) -> CmapInfo {
    let mut info = CmapInfo {
        max_height: 0,
        total_width: 0,
        cmap: BTreeMap::new(),
    };
    info.cmap.insert(GlyphId(0), char::from_u32(0).unwrap());
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
            info.cmap.insert(glyph_id, c);
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
    for (&glyph_id, &unicode) in cmap_info.cmap.iter() {
        if glyph_id.0 >> 8 != current_first_bit || current_cmap_block.len() >= 100 {
            all_cmap_blocks.push(current_cmap_block.clone());
            current_cmap_block = Vec::new();
            current_first_bit = glyph_id.0 >> 8;
        }

        current_cmap_block.push((glyph_id.0 as u32, unicode as u32));
    }
    all_cmap_blocks.push(current_cmap_block);
    all_cmap_blocks
}

fn create_width_list(face: &Face) -> Vec<Object> {
    let mut widths = Vec::new();
    let mut current_low_gid = 0;
    let mut current_high_gid = 0;
    let mut current_widths = Vec::new();
    let scaling_factor = 1000.0 / face.units_per_em() as f32;

    for gid in 0..face.number_of_glyphs() {
        if let Some(width) = face.glyph_hor_advance(GlyphId(gid)) {
            let width = (width as f32 * scaling_factor) as i64;
            if gid == current_high_gid {
                current_widths.push(Object::Integer(width));
                current_high_gid += 1;
            } else {
                widths.push(Object::Integer(current_low_gid as i64));
                widths.push(Object::Array(std::mem::take(&mut current_widths)));
                current_widths.push(Object::Integer(width));
                current_low_gid = gid;
                current_high_gid = gid + 1;
            }
        }
    }

    widths.push(Object::Integer(current_low_gid as i64));
    widths.push(Object::Array(std::mem::take(&mut current_widths)));

    widths
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
    pub fn add_to_doc(self, doc: &mut Document) -> InternalFontReference {
        let object_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => self.base_font,
        });

        InternalFontReference { object_id }
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
    pub fn add_to_doc(self, doc: &mut Document) -> InternalFontReference {
        let face = Face::parse(&self.font_data, 0).expect("could not parse font data");
        let glyph_ids = get_glyph_id_to_char_map(&face);
        let cmap_info = get_cmap_info(&face, &glyph_ids);

        // add font stream
        let stream_id = doc.add_object(Stream::new(
            dictionary! {
                "Length1" => self.font_data.len() as u32,
            },
            self.font_data.clone(),
        ));

        // add font descriptor object
        let descriptor_id = doc.add_object(dictionary! {
            "Type" => "FontDescriptor",
            "FontName" => "FontName",
            "Flags" => 32,
            "FontBBox" => Object::Array(vec![
                Object::Integer(0),
                Object::Integer(cmap_info.max_height as i64),
                Object::Integer(cmap_info.total_width as i64),
                Object::Integer(cmap_info.max_height as i64),
            ]),
            "ItalicAngle" => 0,
            "Ascent" => face.ascender(),
            "Descent" => face.descender(),
            "CapHeight" => face.ascender(),
            "StemV" => 80,
            "MissingWidth" => 1000,
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
        InternalFontReference { object_id: font_id }
    }
}

/// A mapping of key to [FontReference]
///
/// This is to build the font dictionary and to look up the correct `FontReference` to use
/// when encoding text.
#[derive(Default)]
pub(crate) struct FontMap {
    fonts: HashMap<String, Box<dyn FontReference>>,
}

impl FontMap {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn with_one<T: FontReference + 'static>(key: &str, font: T) -> Self {
        let mut this = Self::new();
        this.insert(key, font);
        this
    }

    pub fn insert<T: FontReference + 'static>(&mut self, key: &str, value: T) {
        if !key.chars().all(|c| c.is_ascii_alphanumeric()) {
            panic!(
                "Font reference key must be ASCII alpha numeric but was {}",
                key
            );
        }
        self.fonts.insert(key.to_owned(), Box::new(value));
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.fonts.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn FontReference>> {
        self.fonts.get(key)
    }

    pub fn as_dictionary(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        for (key, value) in self.fonts.iter() {
            dict.set(key.as_str(), value.object_id());
        }
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn add_to_font_map_with_invalid_key() {
        let mut font_map = FontMap::new();
        font_map.insert("this is invalid", InternalFontReference::default());
    }
}
