use std::str::Chars;

use anyhow::Result;
use lol_html::{html_content::TextChunk, text, HtmlRewriter, Settings};

type SegmentReader = dyn Fn(Chars) -> ();

pub fn read_with(html_contents: Vec<u8>, segment_reader: &SegmentReader) -> Result<()> {
    let read_text_chunk = |text: &mut TextChunk| {
        segment_reader(text.as_str().chars());
        Ok(())
    };
    let mut reader = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                text!("p", read_text_chunk),
                text!("h1", read_text_chunk),
            ],
            ..Settings::default()
        },
        |_: &[u8]| (),
    );

    reader.write(&html_contents)?;
    reader.end()?;

    Ok(())
}
