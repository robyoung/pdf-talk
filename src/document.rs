use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, ObjectId,
};

pub(crate) trait DocumentAdditions {
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId;
}

impl DocumentAdditions for Document {
    fn add_catalog(&mut self, pages_id: ObjectId) -> ObjectId {
        let catalog_id = self.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        self.trailer.set("Root", catalog_id);
        catalog_id
    }
}

pub(crate) enum TextItem<'a> {
    String(&'a str),
    Position(f32),
}

pub(crate) fn text_item<'a, T: Into<TextItem<'a>>>(t: T) -> TextItem<'a> {
    t.into()
}

impl<'a> Into<Object> for TextItem<'a> {
    fn into(self) -> Object {
        match self {
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

///
/// ```
/// text()
///     .font("F1", 36)
///     .move_to(100, 200)
///     .leading(48)
///     .word_spacing(-8)
///     .colour((0.106, 0.259, 0.471))
///     .text([
///         "W".into(),
///         "h".into(),
///         (-500).into(),
///         "at even i".into(),
///         200.into(),
///     ])
///     .build()
/// ```
#[derive(Default)]
pub(crate) struct TextBuilder {
    operations: Vec<Operation>,
}

impl<'a> TextBuilder {
    fn new() -> Self {
        TextBuilder {
            operations: vec![Operation::new("BT", vec![])],
        }
    }

    pub fn font(mut self, font: &str, size: u32) -> Self {
        self.operations
            .push(Operation::new("Tf", vec![font.into(), size.into()]));
        self
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

    pub fn word_spacing(mut self, s: f32) -> Self {
        self.operations.push(Operation::new("Tw", vec![s.into()]));
        self
    }

    pub fn colour(mut self, c: (f32, f32, f32)) -> Self {
        self.operations.push(Operation::new(
            "rg",
            vec![c.0.into(), c.1.into(), c.2.into()],
        ));
        self
    }

    pub fn text(mut self, text: Vec<TextItem>) -> Self {
        self.operations.push(Operation::new(
            "TJ",
            vec![Object::Array(
                text.into_iter().map(Into::into).collect::<Vec<Object>>(),
            )],
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
}

pub(crate) fn text() -> TextBuilder {
    TextBuilder::new()
}
