use anyhow::Result;
use lol_html::{
    html_content::{ContentType, TextChunk},
    text, HtmlRewriter, Settings,
};

use crate::transliteration_dictionary::TransliterationDictionary;

pub struct Rewriter {
    dict: TransliterationDictionary,
}

impl Rewriter {
    pub fn new(dict: TransliterationDictionary) -> Rewriter {
        Rewriter { dict }
    }

    pub fn rewrite(&mut self, html_contents: Vec<u8>) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![text!("p,span,div,h1,h2,h3,h4,h5", |tc| Ok(
                    self.rewrite_text_chunk(tc)
                ))],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(&html_contents)?;
        rewriter.end()?;

        self.dict.log_failure_report();
        Ok(output)
    }

    fn rewrite_text_chunk(&mut self, text_chunk: &mut TextChunk) -> () {
        let old_content_text = text_chunk.as_str().chars();
        let new_content_text = self
            .dict
            .transliterate_segment(old_content_text);
        text_chunk.replace(&new_content_text.as_str(), ContentType::Text);
    }
}
