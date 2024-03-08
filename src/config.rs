use std::path::PathBuf;

use clap::ValueEnum;
use lopdf::xref::XrefType;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum FontType {
    Type0,
    Type1,
    Ttf,
}

pub struct FontFile {
    pub full: &'static str,
    pub subset: &'static str,
}

#[derive(Debug)]
pub struct CreateConfig {
    pub xref_type: XrefType,
    pub font_type: FontType,
    pub compress: bool,
    pub compress_content: bool,
    pub subset: bool,
    pub output: PathBuf,
}

impl CreateConfig {
    pub(crate) fn font_path(&self, font_file: &FontFile) -> &str {
        if self.subset {
            font_file.subset
        } else {
            font_file.full
        }
    }

    pub(crate) fn compress(&self, doc: &mut lopdf::Document) {
        if self.compress {
            doc.compress();
        }
    }

    pub(crate) fn apply_xref_table(&self, doc: &mut lopdf::Document) {
        doc.reference_table.cross_reference_type = self.xref_type;
    }

    pub(crate) fn save(&self, doc: &mut lopdf::Document) {
        doc.save(&self.output).expect("Failed to save PDF");
    }

    pub(crate) fn apply_and_save(&self, doc: &mut lopdf::Document) {
        self.compress(doc);
        self.apply_xref_table(doc);
        self.save(doc);
    }
}

impl Default for CreateConfig {
    fn default() -> Self {
        Self {
            xref_type: XrefType::CrossReferenceStream,
            font_type: FontType::Type0,
            compress: false,
            compress_content: false,
            subset: false,
            output: PathBuf::from("output.pdf"),
        }
    }
}
