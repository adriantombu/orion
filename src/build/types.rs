use crate::build::post::Post;
use crate::Config;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct TemplateData<'a> {
    pub post: &'a Post,
    pub config: &'a Config,
}
