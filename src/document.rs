//! Helpers for building PDF documents
//!
//! This includes a collection of helpers for making building PDF documents a
//! little bit easier. It is somewhat equivalent to a lower level version of the
//! crate `printpdf`.
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, ObjectId, Stream,
};

use crate::fonts::{FontMap, FontReference};

/// Adds helper methods to [lopdf::Document].
pub(crate) trait DocumentAdditions {
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId;
}

impl DocumentAdditions for Document {
    /// Add a basic catalog object
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId {
        let catalog_id = self.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        self.trailer.set("Root", catalog_id);
        catalog_id
    }
}

/// Possible values in a `TJ` text showing array.
///
/// A text showing array (`TJ`) can either be a string literal or a numeric position
/// offset. See section 9.4.3 of the PDF spec.
pub(crate) enum TextItem<'a> {
    String(&'a str),
    Position(f32),
}

#[allow(dead_code)]
pub(crate) fn text_item<'a, T: Into<TextItem<'a>>>(t: T) -> TextItem<'a> {
    t.into()
}

impl<'a> From<TextItem<'a>> for Object {
    fn from(value: TextItem<'a>) -> Self {
        match value {
            TextItem::String(s) => Object::string_literal(s),
            TextItem::Position(p) => Object::Real(p),
        }
    }
}

impl<'a> From<&'a str> for TextItem<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}

impl<'a> From<f32> for TextItem<'a> {
    fn from(value: f32) -> Self {
        Self::Position(value)
    }
}

impl<'a> From<i32> for TextItem<'a> {
    fn from(value: i32) -> Self {
        Self::Position(value as f32)
    }
}

/// Helper for building PDF content streams
///
/// A fluid interface to avoid lots of verbose lopdf code.
/// It needs to know about the fonts that are used so that it can
/// render them properly because Type0 fonts need to be hex encoded.
#[derive(Default)]
pub(crate) struct ContentBuilder {
    operations: Vec<Operation>,
    font_map: FontMap,
    current_font: Option<String>,
}

impl ContentBuilder {
    fn new(font_map: FontMap) -> Self {
        ContentBuilder {
            operations: vec![Operation::new("BT", vec![])],
            font_map,
            ..Default::default()
        }
    }

    pub fn font(mut self, font: &str, size: u32) -> Self {
        if !self.font_map.contains_key(font) {
            panic!("Font {} does not exist in font map", font);
        }
        self.current_font = Some(font.to_owned());
        self.operations
            .push(Operation::new("Tf", vec![font.into(), size.into()]));
        self
    }

    fn current_font_ref(&self) -> &Box<dyn FontReference> {
        self.current_font
            .as_ref()
            .and_then(|font| self.font_map.get(font))
            .expect(
                "Attempting to use a font before referencing it. Call `font` to set a Tf first.",
            )
    }

    pub fn move_to(mut self, x: i32, y: i32) -> Self {
        self.operations
            .push(Operation::new("Td", vec![x.into(), y.into()]));
        self
    }

    pub fn leading(mut self, l: u32) -> Self {
        self.operations.push(Operation::new("TL", vec![l.into()]));
        self
    }

    pub fn colour(mut self, c: (f32, f32, f32)) -> Self {
        self.operations.push(Operation::new(
            "rg",
            vec![c.0.into(), c.1.into(), c.2.into()],
        ));
        self
    }

    #[allow(dead_code)]
    pub fn text_items(mut self, text: Vec<TextItem>) -> Self {
        self.operations.push(Operation::new(
            "TJ",
            vec![Object::Array(
                text.into_iter()
                    .flat_map(|t| match t {
                        TextItem::String(s) => self.current_font_ref().render_text(s),
                        TextItem::Position(_) => vec![Into::<Object>::into(t)],
                    })
                    .collect::<Vec<Object>>(),
            )],
        ));
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.operations.push(Operation::new(
            "Tj",
            self.current_font_ref().render_text(text),
        ));
        self
    }

    pub fn build_operations(mut self) -> Vec<Operation> {
        self.operations.push(Operation::new("ET", vec![]));
        self.operations
    }

    pub fn build_content(self) -> Content {
        Content {
            operations: self.build_operations(),
        }
    }

    pub fn add_to_doc(self, doc: &mut Document) -> ObjectId {
        doc.add_object(Stream::new(
            dictionary! {},
            self.build_content().encode().unwrap(),
        ))
    }
}

pub(crate) fn content(font_map: FontMap) -> ContentBuilder {
    ContentBuilder::new(font_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::InternalFontReference;

    #[test]
    fn text_builder() {
        let font_map = FontMap::with_one("F1", InternalFontReference::default());
        let operations = content(font_map)
            .font("F1", 36)
            .move_to(100, 200)
            .leading(36)
            .text("Some text")
            .build_operations();

        assert_eq!(
            operations
                .into_iter()
                .map(|op| op.operator)
                .collect::<Vec<_>>(),
            vec![
                "BT".to_owned(),
                "Tf".to_owned(),
                "Td".to_owned(),
                "TL".to_owned(),
                "Tj".to_owned(),
                "ET".to_owned(),
            ]
        )
    }

    #[test]
    #[should_panic]
    fn add_text_without_font() {
        let font_map = FontMap::new();
        content(font_map).text("some text");
    }

    #[test]
    #[should_panic]
    fn use_font_not_in_font_map() {
        let font_map = FontMap::with_one("F1", InternalFontReference::default());
        content(font_map).font("F2", 10);
    }
}
