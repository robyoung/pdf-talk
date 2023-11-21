use std::path::PathBuf;

use clap::ValueEnum;
use lopdf::xref::XrefType;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum FontType {
    Type0,
    Type1,
    Ttf,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Driver {
    Lopdf,
    Printpdf,
}
pub struct FontFile {
    pub full: &'static str,
    pub subset: &'static str,
}

pub struct Config {
    pub xref_type: XrefType,
    pub font_type: FontType,
    pub output: PathBuf,
    pub driver: Driver,
    pub subset: bool,
}

impl Config {
    pub fn font_path(&self, font_file: &FontFile) -> &str {
        if self.subset {
            font_file.subset
        } else {
            font_file.full
        }
    }
}
