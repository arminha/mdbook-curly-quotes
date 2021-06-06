mod preprocessor;

use crate::preprocessor::CurlyQuotes;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use semver::{Version, VersionReq};
use std::io;
use std::process;
use structopt::StructOpt;

/// mdBook preprocessor which replaces straight quotes with curly quotes
#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Check whether a renderer is supported by this preprocessor
    Supports { renderer: String },
}

fn main() {
    let opts = Opt::from_args();
    let preprocessor = CurlyQuotes::new();

    if let Some(Command::Supports { renderer }) = opts.command {
        handle_supports(&preprocessor, &renderer);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let version = Version::parse(&ctx.mdbook_version).unwrap();
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION).unwrap();
    if !version_req.matches(&version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, renderer: &str) -> ! {
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
