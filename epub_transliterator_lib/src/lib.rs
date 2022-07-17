mod content_handler;
mod pandoc_wrapper;
mod transliterator;

use anyhow::{anyhow, Result};
use log::debug;
use std::{
    ffi::OsStr,
    fs::{remove_file, write},
    path::PathBuf,
};

pub struct EpubTransliteratorEngine {
    params: EpubTransliteratorEngineParams,
}

impl EpubTransliteratorEngine {
    pub fn new(params: EpubTransliteratorEngineParams) -> Result<EpubTransliteratorEngine> {
        Ok(EpubTransliteratorEngine { params })
    }

    pub fn transliterate(&mut self) -> Result<()> {
        let intermediary_output_path = &self
            .params
            .path_to_intermediary_html;
        let final_output_path = &self
            .params
            .path_to_output_epub;

        debug!("Transforming epub to html using pandoc..");
        let epub_as_html = pandoc_wrapper::epub_to_html(&self.params.path_to_input_epub)?;
        debug!("Done!");

        debug!("Transliterating epub's content..");
        let epub_as_transliterated_html =
            content_handler::rewrite_with(epub_as_html, &transliterator::transliterate_segment)?;
        debug!("Done!");

        debug!(
            "Writing intermediary html to {:?}",
            intermediary_output_path
        );
        write(&intermediary_output_path, &epub_as_transliterated_html)?;
        debug!("Done!");

        debug!("Transforming intermediary html back to epub..");
        let transliterated_epub = pandoc_wrapper::html_to_epub(intermediary_output_path)?;
        debug!("Done!");

        debug!(
            "Writing final transliterated epub to {:?}",
            final_output_path
        );
        write(final_output_path, &transliterated_epub)?;
        debug!("Done!");

        Ok(())
    }
}

#[derive(Debug)]
pub struct EpubTransliteratorEngineParams {
    path_to_input_epub: PathBuf,
    path_to_output_epub: PathBuf,
    path_to_intermediary_html: PathBuf,
}

impl EpubTransliteratorEngineParams {
    pub fn new(path_to_input_epub_as_string: &String) -> Result<EpubTransliteratorEngineParams> {
        let path_to_input_epub = Self::parse_path_to_input_epub(path_to_input_epub_as_string)?;
        let path_to_output_epub = Self::parse_path_to_output_epub(path_to_input_epub_as_string)?;
        let path_to_intermediary_html =
            Self::parse_path_to_intermediary_html(path_to_input_epub_as_string)?;

        let params = EpubTransliteratorEngineParams {
            path_to_input_epub,
            path_to_output_epub,
            path_to_intermediary_html,
        };
        debug!("Input parameters:\n{:#?}", params);
        Ok(params)
    }

    fn parse_path_to_input_epub(path_to_input_epub_string: &String) -> Result<PathBuf> {
        let path_to_epub = PathBuf::from(path_to_input_epub_string);
        if !path_to_epub.exists() {
            return Err(anyhow!("The supplied epub path doesn't exists"));
        }
        if path_to_epub.extension() != Some(OsStr::new("epub")) {
            return Err(anyhow!(
                "Expecting epub file to have .epub file extensions, real: '{:?}'",
                path_to_epub.extension()
            ));
        }
        Ok(path_to_epub)
    }

    fn parse_path_to_intermediary_html(path_to_input_epub_string: &String) -> Result<PathBuf> {
        let path_to_html_string = &path_to_input_epub_string.replace(".epub", ".html");
        let path_to_html = PathBuf::from(&path_to_html_string);
        if path_to_html.exists() {
            debug!(
                "Found existing html file at {:?}, deleting it.",
                &path_to_html
            );
            remove_file(&path_to_html)?;
        }
        Ok(path_to_html)
    }

    fn parse_path_to_output_epub(path_to_input_epub_string: &String) -> Result<PathBuf> {
        let path_to_transliterated_epub_string =
            &path_to_input_epub_string.replace(".epub", "_transliterated.epub");
        let path_to_transliterated_epub = PathBuf::from(&path_to_transliterated_epub_string);
        if path_to_transliterated_epub.exists() {
            debug!(
                "Found existing epub file at {:?}, deleting it.",
                &path_to_transliterated_epub
            );
            remove_file(&path_to_transliterated_epub)?;
        }
        Ok(path_to_transliterated_epub)
    }
}
