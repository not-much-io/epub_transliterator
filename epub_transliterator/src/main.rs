use anyhow::Result;
use clap::Parser;
use epub_transliterator_lib::{EpubTransliteratorEngine, EpubTransliteratorEngineParams};
use log::{error, LevelFilter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    path_to_epub: String,
    #[clap(long)]
    path_to_dict: String,
}

fn main() -> () {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    match main_inner() {
        Err(err) => {
            error!("{}", err);
        }
        Ok(_) => {}
    }
}

fn main_inner() -> Result<()> {
    let args = Args::parse();
    let params = EpubTransliteratorEngineParams::new(&args.path_to_dict, &args.path_to_epub)?;
    EpubTransliteratorEngine::new(params)?.transliterate()
}
