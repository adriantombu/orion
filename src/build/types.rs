use crate::build::parser::ParserError;
use chrono::NaiveDate;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("{0}")]
    PatternError(#[from] glob::PatternError),

    #[error("{0}")]
    GlobError(#[from] glob::GlobError),

    #[error("{0}")]
    StdIoError(#[from] std::io::Error),

    #[error("{0}")]
    TemplateError(#[from] tinytemplate::error::Error),

    #[error("{0}")]
    FsExtraError(#[from] fs_extra::error::Error),

    #[error("Filename is empty")]
    EmptyFilenameError,

    #[error("Unable to parse file: {0}")]
    ParserError(#[from] ParserError),
}

#[derive(Debug, Serialize, Clone)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub published_at_raw: NaiveDate,
    pub content: String,
    pub canonical: String,
    pub sitename: String,
    pub path: String,
}
