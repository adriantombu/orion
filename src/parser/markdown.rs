use crate::parser::{ParsedData, Parser, ParserError};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser as CmarPulldown};

pub struct MarkdownParser {
    matter: Matter<YAML>,
}

impl Parser for MarkdownParser {
    fn new() -> Self {
        Self {
            matter: Matter::new(),
        }
    }

    fn parse(&self, text: &str) -> Result<ParsedData, ParserError> {
        let parsed_contents = &self.matter.parse(text);
        let mut content = String::new();
        html::push_html(
            &mut content,
            CmarPulldown::new_ext(&parsed_contents.content, Options::empty()),
        );

        parsed_contents
            .data
            .as_ref()
            .ok_or(ParserError::EmptyValueError)
            .and_then(|fm| {
                Ok(ParsedData {
                    title: fm["title"].as_string()?,
                    description: fm["description"].as_string()?,
                    sitename: "Blob Trotter".to_string(),
                    published_at: fm["published_at"].as_string()?,
                    content,
                })
            })
    }
}
