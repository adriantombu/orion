use thiserror::Error;

// TODO: handle more values
#[derive(Debug)]
pub struct Data {
    pub title: String,
    pub description: String,
    pub published_at: String,
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("{0}")]
    Pattern(#[from] glob::PatternError),
    #[error("{0}")]
    Glob(#[from] glob::GlobError),
    #[error("{0}")]
    StdIo(#[from] std::io::Error),
    #[error("{0}")]
    Markdown(#[from] gray_matter::Error),
    #[error("")]
    Parse,
    #[error("Filename is empty")]
    EmptyFilename,
}
