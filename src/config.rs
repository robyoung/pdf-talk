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
    pub fn font_path(&self, font_file: &FontFile) -> &str {
        if self.subset {
            font_file.subset
        } else {
            font_file.full
        }
    }
}
