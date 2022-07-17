use anyhow::Result;
use clap::Parser;
use epub_transliterator_lib::{EpubTransliteratorEngine, EpubTransliteratorEngineParams};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    path_to_epub: String,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let args = Args::parse();
    let params = EpubTransliteratorEngineParams::new(&args.path_to_epub)?;
    EpubTransliteratorEngine::new(params)?.transliterate()
}
