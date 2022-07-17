use std::str::Chars;

use anyhow::Result;
use lol_html::{
    html_content::{ContentType, TextChunk},
    text, HtmlRewriter, Settings,
};

type SegmentRewriter = dyn Fn(Chars) -> String;

pub fn rewrite_with(html_contents: Vec<u8>, segment_rewriter: &SegmentRewriter) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let rewrite_text_chunk = |text: &mut TextChunk| {
        let old_content_text = text.as_str().chars();
        let new_content_text = segment_rewriter(old_content_text);
        text.replace(&new_content_text.as_str(), ContentType::Text);
        Ok(())
    };
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                text!("p", rewrite_text_chunk),
                text!("h1", rewrite_text_chunk),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );

    rewriter.write(&html_contents)?;
    rewriter.end()?;

    Ok(output)
}
