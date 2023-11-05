use lopdf::{dictionary, Document, Object, ObjectId, Stream};
use std::io::Read;

pub(crate) fn type1(base_font: &str) -> FontType1Builder {
    FontType1Builder {
        base_font: base_font.to_owned(),
    }
}

#[derive(Default)]
pub(crate) struct FontType1Builder {
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

pub(crate) fn true_type(font: &[u8]) -> FontTrueTypeBuilder {
    FontTrueTypeBuilder {
        font_data: font.to_vec(),
    }
}

pub(crate) struct FontTrueTypeBuilder {
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
