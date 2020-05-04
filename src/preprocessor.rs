use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct CurlyQuotes;

impl CurlyQuotes {
    pub fn new() -> Self {
        CurlyQuotes
    }
}

impl Preprocessor for CurlyQuotes {
    fn name(&self) -> &str {
        "curly-quotes-preprocessor"
    }

    fn run(&self, _: &PreprocessorContext, _: Book) -> Result<Book, Error> {
        todo!()
    }
}
