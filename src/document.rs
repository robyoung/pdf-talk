//! Helpers for building PDF documents
//!
//! This includes a collection of helpers for making building PDF documents a
//! little bit easier. It is somewhat equivalent to a lower level version of the
//! crate `printpdf`.
use lopdf::{
    content::{Content, Operation},
    dictionary, Dictionary, Document, Object, ObjectId, Stream,
};
use std::collections::HashMap;

use crate::fonts::FontReference;

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
pub(crate) struct ContentBuilder<'a> {
    pub operations: Vec<Operation>,
    pub resources: &'a Resources,
    current_font: Option<String>,
}

pub type Colour = (f32, f32, f32);

impl<'a> ContentBuilder<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        Self {
            operations: vec![],
            current_font: None,
            resources,
        }
    }

    /// Push an operation with args in the fluid interface
    fn push(mut self, op: &str, args: Vec<Object>) -> Self {
        self.operations.push(Operation::new(op, args));
        self
    }

    /// Push an operation with no arguments (empty) in the fluid interface
    fn pushe(self, op: &str) -> Self {
        self.push(op, vec![])
    }

    /// Begin text input (`BT`)
    pub fn begin_text(self) -> Self {
        self.pushe("BT")
    }

    /// End text input (`ET`)
    pub fn end_text(self) -> Self {
        self.pushe("ET")
    }

    /// Set the text font (`Tf`)
    ///
    /// See section 9.3.1 of the PDF spec
    pub fn font(mut self, font: &str, size: u32) -> Self {
        if !self.resources.fonts.contains_key(font) {
            panic!("Font {} does not exist in font map", font);
        }
        self.current_font = Some(font.to_owned());
        self.push("Tf", vec![font.into(), size.into()])
    }

    #[allow(clippy::borrowed_box)]
    fn current_font_ref(&self) -> &Box<dyn FontReference> {
        self.current_font
            .as_ref()
            .and_then(|font| self.resources.fonts.get(font))
            .expect(
                "Attempting to use a font before referencing it. Call `font` to set a Tf first.",
            )
    }

    /// Move to start of the next line (`Td`)
    ///
    /// See section 9.4.2 of the PDF spec
    pub fn text_position(self, x: i32, y: i32) -> Self {
        self.push("Td", vec![x.into(), y.into()])
    }

    /// Set the text leading (`TL`)
    ///
    /// See section 9.3.1 of the PDF spec
    pub fn leading(self, l: u32) -> Self {
        self.push("TL", vec![l.into()])
    }

    /// Set nonstroking colour (`rg`)
    ///
    /// See section 8.6.8 of the PDF spec
    pub fn colour(self, c: Colour) -> Self {
        self.push("rg", vec![c.0.into(), c.1.into(), c.2.into()])
    }

    /// Set stroke colour (`RG`)
    ///
    /// See section 8.6.8 of the PDF spec
    pub fn scolour(self, c: Colour) -> Self {
        self.push("RG", vec![c.0.into(), c.1.into(), c.2.into()])
    }

    /// Show text string as array (`TJ`)
    ///
    /// See section 9.4.3 of the PDF spec
    #[allow(dead_code)]
    pub fn text_items(self, text: Vec<TextItem>) -> Self {
        let args = vec![Object::Array(
            text.into_iter()
                .flat_map(|t| match t {
                    TextItem::String(s) => self.current_font_ref().render_text(s),
                    TextItem::Position(_) => vec![Into::<Object>::into(t)],
                })
                .collect::<Vec<Object>>(),
        )];
        self.push("TJ", args)
    }

    /// Show text string (`Tj`)
    ///
    /// See section 9.4.3 of the PDF spec
    pub fn text(self, text: &str) -> Self {
        let text = self.current_font_ref().render_text(text);
        self.push("Tj", text)
    }

    /// Save current graphics state (`q`)
    ///
    /// See section 8.4.2 of the PDF spec
    pub fn save_graphics_state(self) -> Self {
        self.pushe("q")
    }

    /// Restore graphics state (`Q`)
    ///
    /// See section 8.4.4 of the PDF spec
    pub fn restore_graphics_state(self) -> Self {
        self.pushe("Q")
    }

    /// Modify transformation matrix (`cm`)
    ///
    /// See section 8.4.4 of the PDF spec
    /// See section 8.3.3 for common transformations
    pub fn modify_trans_matrix<
        A: Into<Number>,
        B: Into<Number>,
        C: Into<Number>,
        D: Into<Number>,
        E: Into<Number>,
        F: Into<Number>,
    >(
        self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
    ) -> Self {
        self.push(
            "cm",
            vec![
                Number::as_object(a),
                Number::as_object(b),
                Number::as_object(c),
                Number::as_object(d),
                Number::as_object(e),
                Number::as_object(f),
            ],
        )
    }

    /// Transformation matrix "translation"
    ///
    /// This modifies the transformation matrix to translate or move the origin
    /// to the specified position.
    pub fn cm_position<X: Into<Number>, Y: Into<Number>>(self, x: X, y: Y) -> Self {
        self.modify_trans_matrix(1, 0, 0, 1, x, y)
    }

    pub fn cm_scale<X: Into<Number>, Y: Into<Number>>(self, x: X, y: Y) -> Self {
        self.modify_trans_matrix(x, 0, 0, y, 0, 0)
    }

    pub fn cm_rotate(self, q: f32) -> Self {
        let rc = q.cos();
        let rs = q.sin();

        self.modify_trans_matrix(rc, rs, 0f32 - rs, rc, 0, 0)
    }

    /// Paint the specified XObject (`Do`)
    ///
    /// This usually means insert an image.
    /// See section 8.8 of the PDF spec
    pub fn add_xobject(self, key: &str) -> Self {
        if !self.resources.xobjects.contains_key(key) {
            panic!("Attempt to insert unknown xobject {}", key);
        }
        self.push("Do", vec![key.into()])
    }

    /// Set line width (`w`)
    ///
    /// See section 8.4.4 of the PDF spec
    pub fn line_width<T: Into<Number>>(self, w: T) -> Self {
        self.push("w", vec![Number::as_object(w)])
    }

    /// Begin path (`m`)
    ///
    /// See section 8.5.2
    pub fn begin_path<X: Into<Number>, Y: Into<Number>>(self, x: X, y: Y) -> Self {
        self.push("m", vec![Number::as_object(x), Number::as_object(y)])
    }

    pub fn append_straight_line<X: Into<Number>, Y: Into<Number>>(self, x: X, y: Y) -> Self {
        self.push("l", vec![Number::as_object(x), Number::as_object(y)])
    }

    pub fn close_subpath(self) -> Self {
        self.pushe("h")
    }

    pub fn stroke_path(self) -> Self {
        self.pushe("S")
    }

    pub fn fill_path(self) -> Self {
        self.pushe("f")
    }

    pub fn build_operations(self) -> Vec<Operation> {
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

    pub fn add_to_doc_with_page(self, doc: &mut Document, pages_id: ObjectId) -> ObjectId {
        let content_id = self.add_to_doc(doc);
        doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        })
    }
}

pub(crate) enum Number {
    Integer(i64),
    Real(f32),
}

impl Number {
    fn as_object<T: Into<Number>>(n: T) -> Object {
        Into::<Object>::into(Into::<Number>::into(n))
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Integer(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Integer(value as i64)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Real(value)
    }
}

impl From<Number> for Object {
    fn from(value: Number) -> Self {
        match value {
            Number::Integer(v) => Object::Integer(v),
            Number::Real(v) => Object::Real(v),
        }
    }
}

#[derive(Default)]
pub(crate) struct IdMap(HashMap<String, ObjectId>);

impl IdMap {
    pub fn set(&mut self, key: &str, id: ObjectId) {
        if !key.chars().all(|c| c.is_ascii_alphanumeric()) {
            panic!(
                "object reference must be ASCII alpha numeric but was {}",
                key
            );
        }
        self.0.insert(key.to_owned(), id);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn as_dictionary(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        for (key, &value) in self.0.iter() {
            dict.set(key.as_str(), value);
        }
        dict
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
        this.set(key, font);
        this
    }

    pub fn set<T: FontReference + 'static>(&mut self, key: &str, value: T) {
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

    #[allow(clippy::borrowed_box)]
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

#[derive(Default)]
pub(crate) struct Resources {
    xobjects: IdMap,
    fonts: FontMap,
}

impl Resources {
    pub fn set_font<T: FontReference + 'static>(&mut self, key: &str, font: T) {
        self.fonts.set(key, font);
    }

    pub fn set_xobject(&mut self, key: &str, id: ObjectId) {
        self.xobjects.set(key, id);
    }

    pub fn as_dictionary(&self) -> Dictionary {
        dictionary! {
            "Font" => self.fonts.as_dictionary(),
            "XObject" => self.xobjects.as_dictionary(),
        }
    }

    pub fn add_to_doc(&self, doc: &mut Document) -> ObjectId {
        doc.add_object(self.as_dictionary())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::fonts::InternalFontReference;

    #[test]
    #[should_panic]
    fn add_to_font_map_with_invalid_key() {
        let mut font_map = FontMap::new();
        font_map.set("this is invalid", InternalFontReference::default());
    }

    #[test]
    fn text_builder() {
        let mut resources = Resources::default();
        resources.set_font("F1", InternalFontReference::default());
        let operations = ContentBuilder::new(&resources)
            .begin_text()
            .font("F1", 36)
            .text_position(100, 200)
            .leading(36)
            .text("Some text")
            .end_text()
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
        let resources = Resources::default();
        ContentBuilder::new(&resources).text("some text");
    }

    #[test]
    #[should_panic]
    fn use_font_not_in_font_map() {
        let mut resources = Resources::default();
        resources.set_font("F1", InternalFontReference::default());
        ContentBuilder::new(&resources).font("F2", 10);
    }
}
